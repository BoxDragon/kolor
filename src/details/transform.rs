use super::color::{TransformFn, WhitePoint};
use crate::{FType, Mat3, Vec3, PI, TAU};

/// [ColorTransform] represents a reference to a function that can apply a [TransformFn]
/// or its inverse.
#[derive(Copy, Clone)]
pub struct ColorTransform {
    first: for<'r> fn(&'r mut Vec3, WhitePoint),
    second: Option<for<'r> fn(&'r mut Vec3, WhitePoint)>,
}
impl ColorTransform {
    pub fn new(src_transform: TransformFn, dst_transform: TransformFn) -> Option<Self> {
        use super::transform::*;
        let from_transform = if src_transform == TransformFn::NONE {
            None
        } else {
            Some(TRANSFORMS_INVERSE[src_transform as usize - 1])
        };
        let to_transform = if dst_transform == TransformFn::NONE {
            None
        } else {
            Some(TRANSFORMS[dst_transform as usize - 1])
        };
        let (first, second) = if from_transform.is_some() {
            (from_transform.unwrap(), to_transform)
        } else if to_transform.is_some() {
            (to_transform.unwrap(), None)
        } else {
            return None;
        };
        Some(Self { first, second })
    }
    pub fn apply(&self, color: &mut Vec3, white_point: WhitePoint) {
        (self.first)(color, white_point);
        if let Some(second) = self.second {
            second(color, white_point);
        }
    }
}

// Keep in sync with TransformFn
const TRANSFORMS: [fn(&mut Vec3, WhitePoint); 11] = [
    // sRGB_Gamma,
    sRGB_gamma,
    // Oklab,
    XYZ_to_Oklab,
    //CIE_xyY,
    XYZ_to_xyY,
    //CIELAB,
    XYZ_to_CIELAB,
    //CIELCh,
    XYZ_to_CIELCh,
    //CIE_1960_UCS,
    XYZ_to_CIE_1960_UCS,
    //CIE_1960_UCS_uvV,
    XYZ_to_CIE_1960_UCS_uvV,
    //CIE_1964_UVW,
    XYZ_to_CIE_1964_UVW,
    //HSL,
    hsx::RGB_to_HSL,
    //HSV,
    hsx::RGB_to_HSV,
    //HSI,
    hsx::RGB_to_HSI,
];

// Keep in sync with TransformFn
const TRANSFORMS_INVERSE: [fn(&mut Vec3, WhitePoint); 11] = [
    // sRGB_Gamma,
    sRGB_gamma_inverse,
    // Oklab,
    Oklab_to_XYZ,
    //CIE_xyY,
    xyY_to_XYZ,
    //CIELAB,
    CIELAB_to_XYZ,
    //CIELCh,
    CIELCh_to_XYZ,
    //CIE_1960_UCS,
    CIE_1960_UCS_to_XYZ,
    //CIE_1960_UCS_uvV,
    CIE_1960_UCS_uvV_to_XYZ,
    //CIE_1964_UVW,
    CIE_1964_UVW_to_XYZ,
    //HSL,
    hsx::HSL_to_RGB,
    //HSV,
    hsx::HSV_to_RGB,
    //HSI,
    hsx::HSI_to_RGB,
];

// Applies the sRGB "opto-eletric transfer function", i.e. gamma compensation
pub fn sRGB_gamma(color: &mut Vec3, _wp: WhitePoint) {
    let cutoff = color.cmplt(Vec3::splat(0.0031308));
    let higher = Vec3::splat(1.055) * color.powf(1.0 / 2.4) - Vec3::splat(0.055);
    let lower = *color * Vec3::splat(12.92);

    *color = Vec3::select(cutoff, lower, higher);
}

// Inverse of `color_gamma`
pub fn sRGB_gamma_inverse(color: &mut Vec3, _wp: WhitePoint) {
    let cutoff = color.cmplt(Vec3::splat(0.04045));
    let higher = ((*color + Vec3::splat(0.055)) / 1.055).powf(2.4);
    let lower = *color / 12.92;

    *color = Vec3::select(cutoff, lower, higher);
}

#[rustfmt::skip]
const OKLAB_M_1: [FType;9] =
    [0.8189330101,0.3618667424,-0.1288597137,
    0.0329845436,0.9293118715,0.0361456387,
    0.0482003018,0.2643662691,0.6338517070];

#[rustfmt::skip]
const OKLAB_M_2: [FType;9] =
   [0.2104542553,0.7936177850,-0.0040720468,
   1.9779984951,-2.4285922050,0.4505937099,
   0.0259040371,0.7827717662,-0.8086757660];

pub fn XYZ_to_Oklab(color: &mut Vec3, _wp: WhitePoint) {
    let mut lms = Mat3::from_cols_array(&OKLAB_M_1).transpose() * *color;
    lms = lms.powf(1.0 / 3.0); // non-linearity
    *color = Mat3::from_cols_array(&OKLAB_M_2).transpose() * lms
}

pub fn Oklab_to_XYZ(color: &mut Vec3, _wp: WhitePoint) {
    let mut lms = Mat3::from_cols_array(&OKLAB_M_2).transpose().inverse() * *color;
    lms = lms.powf(3.0); // reverse non-linearity
    *color = Mat3::from_cols_array(&OKLAB_M_1).transpose().inverse() * lms
}

pub fn XYZ_to_xyY(color: &mut Vec3, _wp: WhitePoint) {
    let x = color.x / (color.x + color.y + color.z);
    let y = color.y / (color.x + color.y + color.z);
    let Y = color.y;
    *color = Vec3::new(x, y, Y);
}

pub fn xyY_to_XYZ(color: &mut Vec3, _wp: WhitePoint) {
    let x = (color.z / color.y) * color.x;
    let y = color.z;
    let z = (color.z / color.y) * (1.0 - color.x - color.y);
    *color = Vec3::new(x, y, z);
}

// CIELAB
pub fn XYZ_to_CIELAB(color: &mut Vec3, wp: WhitePoint) {
    fn magic_f(v: FType) -> FType {
        if v > 0.008856 {
            v.powf(1.0 / 3.0)
        } else {
            v * 7.78703703704 + 0.13793103448
        }
    }
    let wp_value = wp.values();
    let x = magic_f(color.x / wp_value[0]);
    let y = magic_f(color.y / wp_value[1]);
    let z = magic_f(color.z / wp_value[2]);
    let l = 116.0 * y - 16.0;
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);
    *color = Vec3::new(l, a, b);
}

pub fn CIELAB_to_XYZ(color: &mut Vec3, wp: WhitePoint) {
    fn magic_f_inverse(v: FType) -> FType {
        if v > 0.008856 {
            v.powf(3.0)
        } else {
            0.12841854934 * (v - 0.13793103448)
        }
    }
    let wp_value = wp.values();
    let L = (color.x + 16.0) / 116.0;
    let a = color.y / 500.0;
    let b = color.z / 200.0;
    let X = wp_value[0] * magic_f_inverse(L + a);
    let Y = wp_value[1] * magic_f_inverse(L);
    let Z = wp_value[2] * magic_f_inverse(L - b);
    *color = Vec3::new(X, Y, Z);
}

// CIELCh
pub fn XYZ_to_CIELCh(color: &mut Vec3, wp: WhitePoint) {
    XYZ_to_CIELAB(color, wp);
    CIELAB_to_CIELCh(color);
}
pub fn CIELCh_to_XYZ(color: &mut Vec3, wp: WhitePoint) {
    CIELCh_to_CIELAB(color);
    CIELAB_to_XYZ(color, wp);
}
pub fn CIELAB_to_CIELCh(color: &mut Vec3) {
    let mut h = color.z.atan2(color.y);
    if h > 0.0 {
        h = (h / PI) * 180.0;
    } else {
        h = 360.0 - (h.abs() / PI) * 180.0
    }
    let C = (color.y * color.y + color.z * color.z).sqrt();
    color.y = C;
    color.z = h;
}
pub fn CIELCh_to_CIELAB(color: &mut Vec3) {
    let angle = (color.z / 360.0) * TAU;
    let a = color.y * angle.cos();
    let b = color.y * angle.sin();
    color.y = a;
    color.z = b;
}

// CIE 1960 UCS
pub fn XYZ_to_CIE_1960_UCS(color: &mut Vec3, _wp: WhitePoint) {
    let U = (2.0 / 3.0) * color.x;
    let V = color.y;
    let W = 0.5 * (-color.x + 3.0 * color.y + color.z);
    *color = Vec3::new(U, V, W);
}

pub fn CIE_1960_UCS_to_XYZ(color: &mut Vec3, _wp: WhitePoint) {
    let X = (3.0 / 2.0) * color.x;
    let Y = color.y;
    let Z = (3.0 / 2.0) * color.x - 3.0 * color.y + 2.0 * color.z;
    *color = Vec3::new(X, Y, Z)
}

pub fn CIE_1960_UCS_uvV_to_XYZ(color: &mut Vec3, wp: WhitePoint) {
    CIE_1960_uvV_to_UCS(color, wp);
    CIE_1960_UCS_to_XYZ(color, wp);
}
pub fn XYZ_to_CIE_1960_UCS_uvV(color: &mut Vec3, wp: WhitePoint) {
    XYZ_to_CIE_1960_UCS(color, wp);
    CIE_1960_UCS_to_uvV(color, wp);
}

pub fn CIE_1960_UCS_to_uvV(color: &mut Vec3, _wp: WhitePoint) {
    let u_v_w = color.x + color.y + color.z;

    let u = color.x / u_v_w;
    let v = color.y / u_v_w;
    *color = Vec3::new(u, v, color.y)
}

pub fn CIE_1960_uvV_to_UCS(color: &mut Vec3, _wp: WhitePoint) {
    let U = color.z * (color.x / color.y);
    let W = -color.z * (color.x + color.y - 1.0) / color.y;
    *color = Vec3::new(U, color.y, W);
}

pub fn CIE_1960_uvV_to_xyV(color: &mut Vec3, _wp: WhitePoint) {
    let d = 2.0 * color.x - 8.0 * color.y - 4.0;
    let x = 3.0 * (color.x / d);
    let y = 2.0 * (color.y / d);
    *color = Vec3::new(x, y, color.z);
}

pub fn CIE_1960_xyV_to_uvV(color: &mut Vec3, _wp: WhitePoint) {
    let d = 12.0 * color.y - 2.0 * color.x + 3.0;
    let u = 4.0 * (color.x / d);
    let v = 6.0 * (color.y / d);
    *color = Vec3::new(u, v, color.z);
}

// TODO finish implementing this. The wikipedia articles are so convoluted, jeez.
// CIE 1964 UVW
pub fn XYZ_to_CIE_1964_UVW(color: &mut Vec3, wp: WhitePoint) {
    //     // Convert the white point to uvV form
    //     let mut wp_value = Vec3::from_slice_unaligned(wp.values());
    //     XYZ_to_CIE_1960_UCS(&mut wp_value, wp);
    //     CIE_1960_UCS_to_uvV(&mut wp_value);

    //     // Convert the color to uvV form
    //     let mut XYZ = *color;
    //     XYZ_to_CIE_1960_UCS(&mut XYZ, wp);
    //     CIE_1960_UCS_to_uvV(&mut XYZ);

    //     // apply the UVW transform
    //     let uvV = XYZ;
    //     let W = 25.0 * color.z.powf(1.0 / 3.0) - 17.0;
    //     let U = 13.0 * W * (uvV.x - wp_value.x);
    //     let V = 13.0 * W * (uvV.y - wp_value.y);
    //     *color = Vec3::new(U, V, W);
}

pub fn CIE_1964_UVW_to_XYZ(color: &mut Vec3, wp: WhitePoint) {
    // TODO
}

/// transforms for Hue/Saturation/X color models, like HSL, HSI, HSV
pub mod hsx {
    use super::*;
    fn HSX_hue_and_chroma_from_RGB(color: &Vec3, x_max: FType, x_min: FType) -> (FType, FType) {
        let chroma = x_max - x_min;
        let hue = if chroma == 0.0 {
            0.0
        } else if color.x > color.y && color.x > color.z {
            60.0 * (color.y - color.z) / chroma
        } else if color.y > color.x && color.y > color.z {
            60.0 * (2.0 + (color.z - color.x) / chroma)
        } else {
            60.0 * (4.0 + (color.x - color.y) / chroma)
        };
        let hue = if hue < 0.0 { 360.0 + hue } else { hue };
        (hue, chroma)
    }

    pub fn RGB_to_HSL(color: &mut Vec3, _wp: WhitePoint) {
        RGB_to_HSX(color, |_, x_max, x_min, _| {
            let lightness = (x_max + x_min) / 2.0;
            let saturation = if lightness <= 0.0 || lightness >= 1.0 {
                0.0
            } else {
                (x_max - lightness) / lightness.min(1.0 - lightness)
            };
            (saturation, lightness)
        });
    }

    pub fn RGB_to_HSV(color: &mut Vec3, _wp: WhitePoint) {
        RGB_to_HSX(color, |_, max, _, chroma| {
            let value = max;
            let saturation = if value == 0.0 { 0.0 } else { chroma / value };
            (saturation, value)
        });
    }

    pub fn RGB_to_HSI(color: &mut Vec3, _wp: WhitePoint) {
        RGB_to_HSX(color, |color, _, min, _| {
            let intensity = (color.x + color.y + color.z) * (1.0 / 3.0);
            let saturation = if intensity == 0.0 {
                0.0
            } else {
                1.0 - min / intensity
            };
            (saturation, intensity)
        })
    }

    fn RGB_to_HSX<F: FnOnce(&mut Vec3, FType, FType, FType) -> (FType, FType)>(
        color: &mut Vec3,
        f: F,
    ) {
        let x_max = color.x.max(color.y.max(color.z));
        let x_min = color.x.min(color.y.min(color.z));
        let (hue, chroma) = HSX_hue_and_chroma_from_RGB(color, x_max, x_min);
        let (saturation, vli) = f(color, x_max, x_min, chroma);

        color.x = hue;
        color.y = saturation;
        color.z = vli;
    }

    fn HSX_to_RGB<
        F: FnOnce(
            &Vec3,
        ) -> (
            /* hue_prime: */ FType,
            /*chroma: */ FType,
            /*largest_component: */ FType,
            /*lightness_match:*/ FType,
        ),
    >(
        color: &mut Vec3,
        f: F,
    ) {
        let (hue_prime, chroma, largest_component, lightness_match) = f(color);
        let (r, g, b) = RGB_from_HCX(hue_prime, chroma, largest_component);
        color.x = r + lightness_match;
        color.y = g + lightness_match;
        color.z = b + lightness_match;
    }
    pub fn HSL_to_RGB(color: &mut Vec3, _wp: WhitePoint) {
        HSX_to_RGB(color, |color| {
            let chroma = (1.0 - (2.0 * color.z - 1.0).abs()) * color.y;
            let hue_prime = color.x / 60.0;
            let largest_component = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
            let lightness_match = color.z - chroma / 2.0;
            (chroma, hue_prime, largest_component, lightness_match)
        });
    }

    pub fn HSV_to_RGB(color: &mut Vec3, _wp: WhitePoint) {
        HSX_to_RGB(color, |color| {
            let chroma = color.z * color.y;
            let hue_prime = color.x / 60.0;
            let largest_component = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
            let lightness_match = color.z - chroma;
            (chroma, hue_prime, largest_component, lightness_match)
        });
    }

    pub fn HSI_to_RGB(color: &mut Vec3, _wp: WhitePoint) {
        HSX_to_RGB(color, |color| {
            let hue_prime = color.x / 60.0;
            let z = 1.0 - (hue_prime % 2.0 - 1.0).abs();
            let chroma = (3.0 * color.z * color.y) / (1.0 + z);
            let largest_component = chroma * z;
            let lightness_match = color.z * (1.0 - color.y);
            (hue_prime, chroma, largest_component, lightness_match)
        });
    }

    fn RGB_from_HCX(
        hue_prime: FType,
        chroma: FType,
        largest_component: FType,
    ) -> (FType, FType, FType) {
        let (r, g, b) = if hue_prime < 1.0 {
            (chroma, largest_component, 0.0)
        } else if hue_prime < 2.0 {
            (largest_component, chroma, 0.0)
        } else if hue_prime < 3.0 {
            (0.0, chroma, largest_component)
        } else if hue_prime < 4.0 {
            (0.0, largest_component, chroma)
        } else if hue_prime < 5.0 {
            (largest_component, 0.0, chroma)
        } else {
            (chroma, 0.0, largest_component)
        };
        (r, g, b)
    }
}
