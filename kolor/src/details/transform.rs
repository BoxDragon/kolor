use super::{
    color::{TransformFn, WhitePoint},
    math::prelude::*,
};
use crate::{Float, Mat3, Vec3, PI, TAU};
#[cfg(all(not(feature = "std"), feature = "libm"))]
use num_traits::Float;

/// Represents a reference to a function that can apply a [`TransformFn`] or
/// its inverse.
#[derive(Copy, Clone)]
pub struct ColorTransform {
    first: for<'r> fn(Vec3, WhitePoint) -> Vec3,
    second: Option<for<'r> fn(Vec3, WhitePoint) -> Vec3>,
}
impl ColorTransform {
    #[inline]
    pub fn new(src_transform: TransformFn, dst_transform: TransformFn) -> Option<Self> {
        use super::transform::*;
        let from_transform = if src_transform == TransformFn::None {
            None
        } else {
            Some(TRANSFORMS_INVERSE[src_transform as usize - 1])
        };
        let to_transform = if dst_transform == TransformFn::None {
            None
        } else {
            Some(TRANSFORMS[dst_transform as usize - 1])
        };
        if let Some(from_transform) = from_transform {
            Some(Self {
                first: from_transform,
                second: to_transform,
            })
        } else {
            to_transform.map(|to_transform| Self {
                first: to_transform,
                second: None,
            })
        }
    }

    #[inline(always)]
    pub fn apply(&self, color: Vec3, white_point: WhitePoint) -> Vec3 {
        let mut color = (self.first)(color, white_point);
        if let Some(second) = self.second {
            color = second(color, white_point);
        }
        color
    }
}

// Keep in sync with TransformFn
const TRANSFORMS: [fn(Vec3, WhitePoint) -> Vec3; 17] = [
    // sRGB,
    srgb_oetf,
    // Oklab,
    xyz_to_ok_lab,
    // Oklch,
    xyz_to_ok_lch,
    // CIE_xyY,
    xyz_to_xyy,
    // CIE LAB,
    xyz_to_cie_lab,
    // CIE LCh,
    xyz_to_cie_lch,
    // CIE 1960 UCS,
    xyz_to_cie_1960_ucs,
    // CIE 1960 UCS_uvV,
    xyz_to_cie_1960_ucs_uvv,
    // CIE 1964 UVW,
    xyz_to_cie_1964_uvw,
    // CIE 1976 Luv,
    xyz_to_cie_1976_luv,
    // HSL,
    hsx::rgb_to_hsl,
    // HSV,
    hsx::rgb_to_hsv,
    // HSI,
    hsx::rgb_to_hsi,
    // ICtCp PQ,
    ict_cp::rgb_to_ict_cp_pq,
    // ICtCp HLG,
    ict_cp::rgb_to_ict_cp_hlg,
    // BT 601,
    bt601_oetf,
    // PQ,
    st_2084_pq_eotf_inverse,
];

// Keep in sync with TransformFn
const TRANSFORMS_INVERSE: [fn(Vec3, WhitePoint) -> Vec3; 17] = [
    // sRGB,
    srgb_eotf,
    // Oklab,
    ok_lab_to_xyz,
    // Oklch,
    ok_lch_to_xyz,
    //CIE_xyY,
    xyy_to_xyz,
    //CIELAB,
    cie_lab_to_xyz,
    //CIELCh,
    cie_lch_to_xyz,
    //CIE_1960_UCS,
    cie_1960_ucs_to_xyz,
    //CIE_1960_UCS_uvV,
    cie_1960_ucs_uvv_to_xyz,
    //CIE_1964_UVW,
    cie_1964_uvw_to_xyz,
    //CIE_1976_Luv,
    cie_1976_luv_to_xyz,
    //HSL,
    hsx::hsl_to_rgb,
    //HSV,
    hsx::hsv_to_rgb,
    //HSI,
    hsx::hsi_to_rgb,
    //ICtCp_PQ,
    ict_cp::ict_cp_pq_to_rgb,
    //ICtCp_HLG,
    ict_cp::ict_cp_hlg_to_rgb,
    //BT_601,
    bt601_oetf_inverse,
    //PQ,
    st_2084_pq_eotf,
];

/// Applies the sRGB OETF (opto-eletronic transfer function), sometimes called
/// 'gamma compensation'.
#[inline]
pub fn srgb_oetf(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let cutoff = color.cmplt(Vec3::splat(0.0031308));
    let higher = Vec3::splat(1.055) * color.powf(1.0 / 2.4) - Vec3::splat(0.055);
    let lower = color * Vec3::splat(12.92);

    Vec3::select(cutoff, lower, higher)
}

/// Applies the sRGB EOTF (electro-optical transfer function), which is also the
/// direct inverse of the sRGB OETF.
#[inline]
pub fn srgb_eotf(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let cutoff = color.cmplt(Vec3::splat(0.04045));
    let higher = ((color + Vec3::splat(0.055)) / 1.055).powf(2.4);
    let lower = color / 12.92;

    Vec3::select(cutoff, lower, higher)
}

/// Applies the BT.601/BT.709/BT.2020 OETF.
#[inline]
pub fn bt601_oetf(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let cutoff = color.cmplt(Vec3::splat(0.0181));
    let higher = 1.0993 * color.powf(0.45) - Vec3::splat(0.0993);
    let lower = 4.5 * color;

    Vec3::select(cutoff, lower, higher)
}

/// Applies the inverse of the BT.601/BT.709/BT.2020 OETF.
#[inline]
pub fn bt601_oetf_inverse(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let cutoff = color.cmplt(Vec3::splat(0.08145));
    let higher = ((color + Vec3::splat(0.0993)) / 1.0993).powf(1.0 / 0.45);
    let lower = color / 4.5;

    Vec3::select(cutoff, lower, higher)
}

#[rustfmt::skip]
const OKLAB_M_1: Mat3 =
    Mat3::from_cols_array(&[0.8189330101,0.0329845436,0.0482003018,
    0.3618667424,0.9293118715,0.2643662691,
    -0.1288597137,0.0361456387,0.6338517070]);

#[rustfmt::skip]
const OKLAB_M_2: Mat3 =
   Mat3::from_cols_array(&[0.2104542553,1.9779984951,0.02599040371,
    0.7936177850,-2.4285922050,0.7827717662,
    -0.0040720468,0.4505937099,-0.8086757660]);

#[inline]
pub fn xyz_to_ok_lab(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let mut lms = OKLAB_M_1 * color;
    // [cbrt] raises `lms` to (1. / 3.) but also avoids NaN when a component of
    // `lms` is negative. `lms` can contain negative numbers in some cases like
    // when `color` is (0.0, 0.0, 1.0)
    lms = lms.cbrt(); // non-linearity
    OKLAB_M_2 * lms
}

#[inline]
pub fn ok_lab_to_xyz(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let mut lms = OKLAB_M_2.inverse() * color;
    lms = lms.powf(3.0); // reverse non-linearity
    OKLAB_M_1.inverse() * lms
}

#[inline]
pub fn xyz_to_ok_lch(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let oklab = xyz_to_ok_lab(color, _wp);
    let lightness = oklab.x;
    let ab = oklab.yz();
    let chroma = ab.length();
    let hue = ab.y.atan2(ab.x);
    Vec3::new(lightness, chroma, hue)
}

#[inline]
pub fn ok_lch_to_xyz(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let lightness = color.x;
    let chroma = color.y;
    let hue = color.z;
    let (hue_s, hue_c) = hue.sin_cos();
    let a = chroma * hue_c;
    let b = chroma * hue_s;
    let oklab = Vec3::new(lightness, a, b);
    ok_lab_to_xyz(oklab, _wp)
}

#[inline]
pub fn xyz_to_xyy(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let x = color.x / (color.x + color.y + color.z);
    let y = color.y / (color.x + color.y + color.z);
    let Y = color.y;
    Vec3::new(x, y, Y)
}

#[inline]
pub fn xyy_to_xyz(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let x = (color.z / color.y) * color.x;
    let y = color.z;
    let z = (color.z / color.y) * (1.0 - color.x - color.y);
    Vec3::new(x, y, z)
}

// CIELAB
#[inline]
pub fn xyz_to_cie_lab(color: Vec3, wp: WhitePoint) -> Vec3 {
    fn magic_f(v: Float) -> Float {
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
    Vec3::new(l, a, b)
}

#[inline]
#[allow(non_snake_case)]
pub fn cie_lab_to_xyz(color: Vec3, wp: WhitePoint) -> Vec3 {
    fn magic_f_inverse(v: Float) -> Float {
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
    Vec3::new(X, Y, Z)
}

// CIELCh
#[inline]
pub fn xyz_to_cie_lch(color: Vec3, wp: WhitePoint) -> Vec3 {
    xyz_to_cie_lab(color, wp);
    cie_lab_to_cie_lch(color)
}
#[inline]
pub fn cie_lch_to_xyz(color: Vec3, wp: WhitePoint) -> Vec3 {
    cie_lch_to_cie_lab(color);
    cie_lab_to_xyz(color, wp)
}
#[inline]
pub fn cie_lab_to_cie_lch(color: Vec3) -> Vec3 {
    let mut h = color.z.atan2(color.y);
    if h > 0.0 {
        h = (h / PI) * 180.0;
    } else {
        h = 360.0 - (h.abs() / PI) * 180.0
    }
    let C = (color.y * color.y + color.z * color.z).sqrt();
    Vec3::new(color.x, C, h)
}
#[inline]
pub fn cie_lch_to_cie_lab(color: Vec3) -> Vec3 {
    let angle = (color.z / 360.0) * TAU;
    let a = color.y * angle.cos();
    let b = color.y * angle.sin();
    Vec3::new(color.x, a, b)
}

// CIE 1960 UCS
#[inline]
pub fn xyz_to_cie_1960_ucs(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let U = (2.0 / 3.0) * color.x;
    let V = color.y;
    let W = 0.5 * (-color.x + 3.0 * color.y + color.z);
    Vec3::new(U, V, W)
}

#[inline]
pub fn cie_1960_ucs_to_xyz(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let X = (3.0 / 2.0) * color.x;
    let Y = color.y;
    let Z = (3.0 / 2.0) * color.x - 3.0 * color.y + 2.0 * color.z;
    Vec3::new(X, Y, Z)
}

#[inline]
pub fn cie_1960_ucs_uvv_to_xyz(color: Vec3, wp: WhitePoint) -> Vec3 {
    cie_1960_ucs_to_xyz(cie_1960_uvv_to_ucs(color, wp), wp)
}
#[inline]
pub fn xyz_to_cie_1960_ucs_uvv(color: Vec3, wp: WhitePoint) -> Vec3 {
    cie_1960_ucs_to_uvv(xyz_to_cie_1960_ucs(color, wp), wp)
}

#[inline]
pub fn cie_1960_ucs_to_uvv(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let u_v_w = color.x + color.y + color.z;

    let u = color.x / u_v_w;
    let v = color.y / u_v_w;
    Vec3::new(u, v, color.y)
}

#[inline]
pub fn cie_1960_uvv_to_ucs(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let U = color.z * (color.x / color.y);
    let W = -color.z * (color.x + color.y - 1.0) / color.y;
    Vec3::new(U, color.y, W)
}

#[inline]
pub fn cie_1960_uvv_to_xyv(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let d = 2.0 * color.x - 8.0 * color.y - 4.0;
    let x = 3.0 * (color.x / d);
    let y = 2.0 * (color.y / d);
    Vec3::new(x, y, color.z)
}

#[inline]
pub fn cie_1960_xyv_to_uvv(color: Vec3, _wp: WhitePoint) -> Vec3 {
    let d = 12.0 * color.y - 2.0 * color.x + 3.0;
    let u = 4.0 * (color.x / d);
    let v = 6.0 * (color.y / d);
    Vec3::new(u, v, color.z)
}

// TODO finish implementing this. The wikipedia articles are so convoluted,
// jeez. CIE 1964 UVW
#[inline]
pub fn xyz_to_cie_1964_uvw(_color: Vec3, _wp: WhitePoint) -> Vec3 {
    //     // Convert the white point to uvV form
    //     let mut wp_value = Vec3::from_slice(wp.values());
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
    todo!();
}

#[inline]
pub fn cie_1964_uvw_to_xyz(_color: Vec3, _wp: WhitePoint) -> Vec3 {
    // TODO
    todo!();
}

// CIE 1976 Luv
#[inline]
pub fn xyz_to_cie_1976_luv(color: Vec3, wp: WhitePoint) -> Vec3 {
    let U = (4.0 * color.x) / (color.x + (15.0 * color.y) + (3.0 * color.z));
    let V = (9.0 * color.y) / (color.x + (15.0 * color.y) + (3.0 * color.z));

    let Y = color.y / 100.0;
    let Y = if Y > 0.008856 {
        Y.powf(1.0 / 3.0)
    } else {
        (7.787 * Y) + (16.0 / 116.0)
    };

    let wp_values = wp.values();
    let ref_U =
        (4.0 * wp_values[0]) / (wp_values[0] + (15.0 * wp_values[1]) + (3.0 * wp_values[2]));
    let ref_V =
        (9.0 * wp_values[1]) / (wp_values[0] + (15.0 * wp_values[1]) + (3.0 * wp_values[2]));

    Vec3::new(
        (116.0 * Y) - 16.0,
        13.0 * color.x * (U - ref_U),
        13.0 * color.x * (V - ref_V),
    )
}

// Inverse of CIE 1976 Luv
#[inline]
pub fn cie_1976_luv_to_xyz(color: Vec3, wp: WhitePoint) -> Vec3 {
    let Y = (color.x + 16.0) / 116.0;
    let Y = if Y.powf(3.0) > 0.008856 {
        Y.powf(3.0)
    } else {
        (Y - 16.0 / 116.0) / 7.787
    };

    let wp_values = wp.values();
    let ref_U =
        (4.0 * wp_values[0]) / (wp_values[0] + (15.0 * wp_values[1]) + (3.0 * wp_values[2]));
    let ref_V =
        (9.0 * wp_values[1]) / (wp_values[0] + (15.0 * wp_values[1]) + (3.0 * wp_values[2]));

    let U = color.y / (13.0 * color.x) + ref_U;
    let V = color.z / (13.0 * color.x) + ref_V;

    Vec3::new(
        Y * 100.0,
        -(9.0 * color.y * U) / ((U - 4.0) * V - U * V),
        (9.0 * color.y - (15.0 * V * color.y) - (V * color.x)) / (3.0 * V),
    )
}

/// ARIB STD-B67 or "Hybrid Log-Gamma" used in BT.2100
#[allow(non_upper_case_globals)]
pub mod hlg {
    use super::*;
    const HLG_a: Float = 0.17883277;
    const HLG_b: Float = 0.28466892;
    const HLG_c: Float = 0.55991073;
    const HLG_r: Float = 0.5;
    fn HLG_channel(E: Float) -> Float {
        if E <= 1.0 {
            HLG_r * E.sqrt()
        } else {
            HLG_a * (E - HLG_b).ln() + HLG_c
        }
    }
    /// ARIB STD-B67 or "Hybrid Log-Gamma"
    #[allow(non_upper_case_globals)]
    #[inline]
    pub fn arib_hlg_oetf(color: Vec3, _wp: WhitePoint) -> Vec3 {
        Vec3::new(
            HLG_channel(color.x),
            HLG_channel(color.y),
            HLG_channel(color.z),
        )
    }

    /// Inverse of ARIB STD-B67 or "Hybrid Log-Gamma"
    #[inline]
    pub fn arib_hlg_oetf_inverse(color: Vec3, _wp: WhitePoint) -> Vec3 {
        fn HLG_channel_inverse(E_p: Float) -> Float {
            if E_p <= HLG_channel(1.0) {
                (E_p / HLG_r).powf(2.0)
            } else {
                ((E_p - HLG_c) / HLG_a).exp() + HLG_b
            }
        }
        Vec3::new(
            HLG_channel_inverse(color.x),
            HLG_channel_inverse(color.y),
            HLG_channel_inverse(color.z),
        )
    }
}

pub use hlg::*;

/// SMPTE ST 2084:2014 EOTF aka 'Perceptual Quantizer' used in BT.2100
#[allow(non_upper_case_globals)]
pub mod pq {
    use super::*;
    const L_p: Float = 10000.0;
    const M_1: Float = 0.1593017578125;
    const M_2: Float = 78.84375;
    const M_1_d: Float = 1.0 / M_1;
    const M_2_d: Float = 1.0 / M_2;
    const C_1: Float = 0.8359375;
    const C_2: Float = 18.8515625;
    const C_3: Float = 18.6875;

    /// SMPTE ST 2084:2014 perceptual electo-optical transfer function inverse.
    #[inline]
    pub fn st_2084_pq_eotf_inverse_float(f: Float) -> Float {
        let Y_p = (f / L_p).powf(M_1);
        ((C_1 + C_2 * Y_p) / (C_3 * Y_p + 1.0)).powf(M_2)
    }

    /// SMPTE ST 2084:2014 perceptual electo-optical transfer function inverse.
    #[inline]
    pub fn st_2084_pq_eotf_inverse(color: Vec3, _wp: WhitePoint) -> Vec3 {
        Vec3::new(
            st_2084_pq_eotf_inverse_float(color.x),
            st_2084_pq_eotf_inverse_float(color.y),
            st_2084_pq_eotf_inverse_float(color.z),
        )
    }

    /// SMPTE ST 2084:2014 perceptual electo-optical transfer function inverse.
    #[inline]
    pub fn st_2084_pq_eotf_float(f: Float) -> Float {
        let V_p = f.powf(M_2_d);

        let n = (V_p - C_1).max(0.0);
        let L = (n / (C_2 - C_3 * V_p)).powf(M_1_d);
        L_p * L
    }

    /// SMPTE ST 2084:2014 perceptual electro-optical transfer function.
    #[inline]
    pub fn st_2084_pq_eotf(color: Vec3, _wp: WhitePoint) -> Vec3 {
        Vec3::new(
            st_2084_pq_eotf_float(color.x),
            st_2084_pq_eotf_float(color.y),
            st_2084_pq_eotf_float(color.z),
        )
    }
}

pub use pq::*;

/// BT.2100 ICtCp
pub mod ict_cp {
    use super::*;

    #[rustfmt::skip]
    pub(crate) const ICT_CP_LMS: Mat3 = Mat3::from_cols_array(&[
        0.412109, 0.166748, 0.0241699,
        0.523926, 0.720459, 0.0754395,
        0.0639648, 0.112793, 0.900391,
    ]);

    #[rustfmt::skip]
    pub(crate) const ICT_CP_LMS_INVERSE: Mat3 = Mat3::from_cols_array(&[
        3.43661, -0.79133, -0.0259498,
        -2.50646, 1.9836, -0.0989137,
        0.0698454, -0.192271, 1.12486,
    ]);

    #[rustfmt::skip]
    pub(crate) const ICT_CP_FROM_PQ_INVERSE: Mat3 = Mat3::from_cols_array(&[
        1.0, 1.0, 1.0,
        0.00860904, -0.00860904, 0.560031,
        0.11103, -0.11103, -0.320627,
    ]);

    #[rustfmt::skip]
    pub(crate) const ICT_CP_FROM_PQ: Mat3 = Mat3::from_cols_array(&[
        0.5, 1.61377, 4.37817,
        0.5, -3.32349, -4.24561,
        0.0, 1.70972, -0.132568,
    ]);

    /// ICtCp with the HLG transfer function.
    #[inline]
    pub fn rgb_to_ict_cp_hlg(color: Vec3, wp: WhitePoint) -> Vec3 {
        #[rustfmt::skip]
        const ICT_CP_FROM_HLG: Mat3 = Mat3::from_cols_array(&[
            0.5, 0.88501, 2.31934,
            0.5, -1.82251, -2.24902,
            0.0, 0.9375, -0.0703125
        ]);
        let lms = ICT_CP_LMS * color;
        let hlg = hlg::arib_hlg_oetf(lms, wp);
        ICT_CP_FROM_HLG * hlg
    }

    /// Inverse ICtCp with the HLG transfer function.
    #[inline]
    pub fn ict_cp_hlg_to_rgb(color: Vec3, wp: WhitePoint) -> Vec3 {
        #[rustfmt::skip]
        const ICT_CP_FROM_HLG_INVERSE: Mat3 = Mat3::from_cols_array(&[
            0.999998, 1.0, 1.0,
            0.0157186, -0.0157186, 1.02127,
            0.209581, -0.209581, -0.605275,
        ]);
        let lms_hlg = ICT_CP_FROM_HLG_INVERSE * color;
        let lms = hlg::arib_hlg_oetf_inverse(lms_hlg, wp);
        ICT_CP_LMS_INVERSE * lms
    }

    /// ICtCp with the PQ transfer function.
    #[inline]
    pub fn rgb_to_ict_cp_pq(color: Vec3, _wp: WhitePoint) -> Vec3 {
        let lms = ICT_CP_LMS * color;
        let pq = pq::st_2084_pq_eotf_inverse(lms, WhitePoint::D65);
        ICT_CP_FROM_PQ * pq
    }
    /// Inverse ICtCp with the PQ transfer function.
    #[inline]
    pub fn ict_cp_pq_to_rgb(color: Vec3, _wp: WhitePoint) -> Vec3 {
        let lms_pq = ICT_CP_FROM_PQ_INVERSE * color;
        let lms = pq::st_2084_pq_eotf(lms_pq, WhitePoint::D65);
        ICT_CP_LMS_INVERSE * lms
    }
}

pub use ict_cp::*;

/// Transforms for Hue/Saturation/X color models, like HSL, HSI, HSV.
pub mod hsx {
    use super::*;
    fn hsx_hue_and_chroma_from_rgb(color: Vec3, x_max: Float, x_min: Float) -> (Float, Float) {
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

    #[inline]
    pub fn rgb_to_hsl(color: Vec3, _wp: WhitePoint) -> Vec3 {
        rgb_to_hsx(color, |_, x_max, x_min, _| {
            let lightness = (x_max + x_min) / 2.0;
            let saturation = if lightness <= 0.0 || lightness >= 1.0 {
                0.0
            } else {
                (x_max - lightness) / lightness.min(1.0 - lightness)
            };
            (saturation, lightness)
        })
    }

    #[inline]
    pub fn rgb_to_hsv(color: Vec3, _wp: WhitePoint) -> Vec3 {
        rgb_to_hsx(color, |_, max, _, chroma| {
            let value = max;
            let saturation = if value == 0.0 { 0.0 } else { chroma / value };
            (saturation, value)
        })
    }

    #[inline]
    pub fn rgb_to_hsi(color: Vec3, _wp: WhitePoint) -> Vec3 {
        rgb_to_hsx(color, |color, _, min, _| {
            let intensity = (color.x + color.y + color.z) * (1.0 / 3.0);
            let saturation = if intensity == 0.0 {
                0.0
            } else {
                1.0 - min / intensity
            };
            (saturation, intensity)
        })
    }

    #[inline(always)]
    fn rgb_to_hsx<F: FnOnce(Vec3, Float, Float, Float) -> (Float, Float)>(
        color: Vec3,
        f: F,
    ) -> Vec3 {
        let x_max = color.x.max(color.y.max(color.z));
        let x_min = color.x.min(color.y.min(color.z));
        let (hue, chroma) = hsx_hue_and_chroma_from_rgb(color, x_max, x_min);
        let (saturation, vli) = f(color, x_max, x_min, chroma);

        Vec3::new(hue, saturation, vli)
    }

    #[inline(always)]
    fn hsx_to_rgb<
        F: FnOnce(
            Vec3,
        ) -> (
            /* hue_prime: */ Float,
            /* chroma: */ Float,
            /* largest_component: */ Float,
            /* lightness_match: */ Float,
        ),
    >(
        color: Vec3,
        f: F,
    ) -> Vec3 {
        let (hue_prime, chroma, largest_component, lightness_match) = f(color);
        let (r, g, b) = rgb_from_hcx(hue_prime, chroma, largest_component);
        Vec3::new(
            r + lightness_match,
            g + lightness_match,
            b + lightness_match,
        )
    }
    #[inline]
    pub fn hsl_to_rgb(color: Vec3, _wp: WhitePoint) -> Vec3 {
        hsx_to_rgb(color, |color| {
            let chroma = (1.0 - (2.0 * color.z - 1.0).abs()) * color.y;
            let hue_prime = color.x / 60.0;
            let largest_component = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
            let lightness_match = color.z - chroma / 2.0;
            (chroma, hue_prime, largest_component, lightness_match)
        })
    }

    #[inline]
    pub fn hsv_to_rgb(color: Vec3, _wp: WhitePoint) -> Vec3 {
        hsx_to_rgb(color, |color| {
            let chroma = color.z * color.y;
            let hue_prime = color.x / 60.0;
            let largest_component = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
            let lightness_match = color.z - chroma;
            (chroma, hue_prime, largest_component, lightness_match)
        })
    }

    #[inline]
    pub fn hsi_to_rgb(color: Vec3, _wp: WhitePoint) -> Vec3 {
        hsx_to_rgb(color, |color| {
            let hue_prime = color.x / 60.0;
            let z = 1.0 - (hue_prime % 2.0 - 1.0).abs();
            let chroma = (3.0 * color.z * color.y) / (1.0 + z);
            let largest_component = chroma * z;
            let lightness_match = color.z * (1.0 - color.y);
            (hue_prime, chroma, largest_component, lightness_match)
        })
    }

    #[inline(always)]
    fn rgb_from_hcx(
        hue_prime: Float,
        chroma: Float,
        largest_component: Float,
    ) -> (Float, Float, Float) {
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

pub use hsx::*;

#[cfg(test)]
mod test {
    use crate::spaces;
    use crate::ColorConversion;

    use super::*;
    #[test]
    fn ictcp_pq_inverse() {
        let to_conv = ColorConversion::new(spaces::LINEAR_SRGB, spaces::ICT_CP_PQ);
        let from_conv = ColorConversion::new(spaces::ICT_CP_PQ, spaces::LINEAR_SRGB);
        let value = 0.5;

        let pq_val = crate::details::transform::pq::st_2084_pq_eotf_inverse_float(value);
        let inverse_val = crate::details::transform::pq::st_2084_pq_eotf_float(pq_val);
        assert!(
            (value - inverse_val).abs() < 0.001,
            "pq_val {:?} inverse {:?}",
            value,
            inverse_val
        );
        // validate inverse matrices
        let value = Vec3::new(0.5, 0.5, 0.5);
        let result = ict_cp::ICT_CP_FROM_PQ_INVERSE * (ict_cp::ICT_CP_FROM_PQ * value);
        assert!(
            value.abs_diff_eq(result, 0.0001),
            "{:?} != {:?}",
            value,
            result
        );

        let result = ict_cp::ICT_CP_LMS_INVERSE * (ict_cp::ICT_CP_LMS * value);
        assert!(
            value.abs_diff_eq(result, 0.0001),
            "{:?} != {:?}",
            value,
            result
        );

        let to = |value: Vec3| from_conv.convert(to_conv.convert(value));
        let from = |value: Vec3| from_conv.convert(to_conv.convert(value));

        let value = Vec3::new(0.5, 0.5, 0.5);
        let result = from(to(value));
        let allowed_error = 0.002;
        assert!(
            value.abs_diff_eq(result, allowed_error),
            "{:?} != {:?}",
            value,
            result
        );

        let value = Vec3::new(1.0, 1.0, 1.0);
        let result = from(to(value));
        assert!(
            value.abs_diff_eq(result, allowed_error),
            "{:?} != {:?}",
            value,
            result
        );

        let value = Vec3::new(0.5, 0.5, 0.5);
        let result = from(to(value));
        assert!(
            value.abs_diff_eq(result, allowed_error),
            "{:?} != {:?}",
            value,
            result
        );
    }
}
