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
        let (first, second) =
            if src_transform == TransformFn::NONE || dst_transform == TransformFn::NONE {
                let f = fn_from_one_way_transform(src_transform, dst_transform)?;
                (f, None)
            } else {
                let first = fn_from_one_way_transform(src_transform, TransformFn::NONE)?;
                let second = fn_from_one_way_transform(TransformFn::NONE, dst_transform);
                (first, second)
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

fn fn_from_one_way_transform(
    src_transform: TransformFn,
    dst_transform: TransformFn,
) -> Option<fn(&mut Vec3, WhitePoint)> {
    Some(match (src_transform, dst_transform) {
        (TransformFn::sRGB_Gamma, TransformFn::NONE) => sRGB_gamma_inverse,
        (TransformFn::NONE, TransformFn::sRGB_Gamma) => sRGB_gamma,
        (TransformFn::Oklab, TransformFn::NONE) => Oklab_to_XYZ,
        (TransformFn::NONE, TransformFn::Oklab) => XYZ_to_Oklab,
        (TransformFn::CIE_xyY, TransformFn::NONE) => xyY_to_XYZ,
        (TransformFn::NONE, TransformFn::CIE_xyY) => XYZ_to_xyY,
        (TransformFn::CIELAB, TransformFn::NONE) => CIELAB_to_XYZ,
        (TransformFn::NONE, TransformFn::CIELAB) => XYZ_to_CIELAB,
        (TransformFn::CIELCh, TransformFn::NONE) => CIELCh_to_XYZ,
        (TransformFn::NONE, TransformFn::CIELCh) => XYZ_to_CIELCh,
        (TransformFn::CIE_1960_UCS, TransformFn::NONE) => CIE_1960_UCS_to_XYZ,
        (TransformFn::NONE, TransformFn::CIE_1960_UCS) => XYZ_to_CIE_1960_UCS,
        (TransformFn::CIE_1960_UCS_uvV, TransformFn::NONE) => CIE_1960_UCS_to_XYZ,
        (TransformFn::NONE, TransformFn::CIE_1960_UCS_uvV) => XYZ_to_CIE_1960_UCS_uvV,
        _ => return None,
    })
}

// Applies the sRGB "opto-eletric transfer function", i.e. gamma compensation
pub fn sRGB_gamma(srgb: &mut Vec3, _wp: WhitePoint) {
    let cutoff = srgb.cmplt(Vec3::splat(0.0031308));
    let higher = Vec3::splat(1.055) * srgb.powf(1.0 / 2.4) - Vec3::splat(0.055);
    let lower = *srgb * Vec3::splat(12.92);

    *srgb = Vec3::select(cutoff, lower, higher);
}

// Inverse of `srgb_gamma`
pub fn sRGB_gamma_inverse(srgb: &mut Vec3, _wp: WhitePoint) {
    let cutoff = srgb.cmplt(Vec3::splat(0.04045));
    let higher = ((*srgb + Vec3::splat(0.055)) / 1.055).powf(2.4);
    let lower = *srgb / 12.92;

    *srgb = Vec3::select(cutoff, lower, higher);
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
// pub fn XYZ_to_CIE_1964_UVW(color: &mut Vec3, wp: WhitePoint) {
//     // Convert the white point to uvV form
//     let mut wp_value = Vec3::from_slice_unaligned(wp.values());
//     XYZ_to_CIE_1960_UCS(&mut wp_value, wp);
//     CIE_1960_UCS_to_uvV(&mut wp_value);

//     // Convert the color coordinate to uvV form
//     let mut XYZ = *color;
//     XYZ_to_CIE_1960_UCS(&mut XYZ, wp);
//     CIE_1960_UCS_to_uvV(&mut XYZ);

//     // apply the UVW transform
//     let uvV = XYZ;
//     let W = 25.0 * color.z.powf(1.0 / 3.0) - 17.0;
//     let U = 13.0 * W * (uvV.x - wp_value.x);
//     let V = 13.0 * W * (uvV.y - wp_value.y);
//     *color = Vec3::new(U, V, W);
// }

// pub fn CIE_1964_UVW_to_XYZ(color: &mut Vec3, wp: WhitePoint) {
// TODO
// }
