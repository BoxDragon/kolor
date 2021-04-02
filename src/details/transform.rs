use super::color::TransformFn;
use crate::{FType, Mat3, Vec3};

#[derive(Copy, Clone)]
pub struct ColorTransform(for<'r> fn(&'r mut Vec3));
impl ColorTransform {
    pub fn new(src_transform: TransformFn, dst_transform: TransformFn) -> Option<Self> {
        println!("transform from {:?} to {:?}", src_transform, dst_transform);
        use super::transform::*;
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
   [0.2104542553,0.7936177850,-0.0040720468,
   1.9779984951,-2.4285922050,0.4505937099,
   0.0259040371,0.7827717662,-0.8086757660];

pub(crate) fn xyz_to_oklab(color: &mut Vec3) {
    let mut lms = Mat3::from_cols_array(&OKLAB_M_1).transpose() * *color;
    println!("lms {:?} from {:?}", lms, color);
    lms = lms.powf(1.0 / 3.0); // non-linearity
    println!("non-linear {:?}", lms);
    *color = Mat3::from_cols_array(&OKLAB_M_2).transpose() * lms
}

pub(crate) fn oklab_to_xyz(color: &mut Vec3) {
    let mut lms = Mat3::from_cols_array(&OKLAB_M_2).transpose().inverse() * *color;
    lms = lms.powf(3.0); // reverse non-linearity
    *color = Mat3::from_cols_array(&OKLAB_M_1).transpose().inverse() * lms
}
