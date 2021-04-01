use glam::f32::Vec3;
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
