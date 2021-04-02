use crate::{color::TransformFn, FType, Mat3, Vec3};

#[derive(Copy, Clone)]
pub struct ColorTransform(for<'r> fn(&'r mut Vec3));
impl ColorTransform {
    pub fn new(src_transform: TransformFn, dst_transform: TransformFn) -> Option<Self> {
        use crate::transform::*;
        Some(Self(match (src_transform, dst_transform) {
            (TransformFn::SRGB_Gamma, TransformFn::NONE) => {
                // invert gamma correction
                srgb_gamma_inverse
            }
            (TransformFn::NONE, TransformFn::SRGB_Gamma) => {
                // apply gamma correction
                srgb_gamma
            }
            (TransformFn::Oklab, TransformFn::NONE) => {
                // invert gamma correction
                oklab_to_xyz
            }
            (TransformFn::NONE, TransformFn::Oklab) => {
                // apply gamma correction
                xyz_to_oklab
            }
            _ => return None,
        }))
    }
    pub fn apply(&self, color: &mut Vec3) {
        self.0(color);
    }
}

// Applies the sRGB "opto-eletric transfer function", i.e. gamma compensation
pub(crate) fn srgb_gamma(srgb: &mut Vec3) {
    let cutoff = srgb.cmplt(Vec3::splat(0.0031308));
    let higher = Vec3::splat(1.055) * srgb.powf(1.0 / 2.4) - Vec3::splat(0.055);
    let lower = *srgb * Vec3::splat(12.92);

    *srgb = Vec3::select(cutoff, lower, higher);
}

// Inverse of `srgb_gamma`
pub(crate) fn srgb_gamma_inverse(srgb: &mut Vec3) {
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
    [0.2104542553,1.9779984951,0.0259040371,
    0.7936177850,-2.4285922050,0.7827717662,
    -0.0040720468,0.4505937099,-0.8086757660];

pub(crate) fn xyz_to_oklab(color: &mut Vec3) {
    let mut lms = Mat3::from_cols_array(&OKLAB_M_1).transpose() * *color;
    lms *= Vec3::splat(1.0 / 3.0); // non-linearity
    *color = Mat3::from_cols_array(&OKLAB_M_2).transpose() * lms
}

pub(crate) fn oklab_to_xyz(color: &mut Vec3) {
    let mut lms = Mat3::from_cols_array(&OKLAB_M_2).transpose().inverse() * *color;
    lms = lms.powf(3.0); // reverse non-linearity
    *color = Mat3::from_cols_array(&OKLAB_M_1).transpose().inverse() * lms
}

// pub(crate) fn linear_srgb_to_oklab(RGB c)
// {
//     float l = 0.4122214708f * c.r + 0.5363325363f * c.g + 0.0514459929f * c.b;
// 	float m = 0.2119034982f * c.r + 0.6806995451f * c.g + 0.1073969566f * c.b;
// 	float s = 0.0883024619f * c.r + 0.2817188376f * c.g + 0.6299787005f * c.b;

//     float l_ = cbrtf(l);
//     float m_ = cbrtf(m);
//     float s_ = cbrtf(s);

//     return {
//         0.2104542553f*l_ + 0.7936177850f*m_ - 0.0040720468f*s_,
//         1.9779984951f*l_ - 2.4285922050f*m_ + 0.4505937099f*s_,
//         0.0259040371f*l_ + 0.7827717662f*m_ - 0.8086757660f*s_,
//     };
// }

// RGB oklab_to_linear_srgb(Lab c)
// {
//     float l_ = c.L + 0.3963377774f * c.a + 0.2158037573f * c.b;
//     float m_ = c.L - 0.1055613458f * c.a - 0.0638541728f * c.b;
//     float s_ = c.L - 0.0894841775f * c.a - 1.2914855480f * c.b;

//     float l = l_*l_*l_;
//     float m = m_*m_*m_;
//     float s = s_*s_*s_;

//     return {
// 		+4.0767416621f * l - 3.3077115913f * m + 0.2309699292f * s,
// 		-1.2684380046f * l + 2.6097574011f * m - 0.3413193965f * s,
// 		-0.0041960863f * l - 0.7034186147f * m + 1.7076147010f * s,
//     };
// }
