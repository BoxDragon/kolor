use kolor::conversion::LinearColorConversion;
use kolor_64 as kolor;

fn main() {
    let mut conversions = Vec::new();
    for src in &kolor::color_spaces::ALL_COLOR_SPACES {
        for dst in &kolor::color_spaces::ALL_COLOR_SPACES {
            if src == dst {
                continue;
            }
            if !src.is_linear() || !dst.is_linear() {
                continue;
            }
            conversions.push(LinearColorConversion::new(*src, *dst));
        }
    }
    let mut out_str = String::with_capacity(conversions.len() * 256);
    out_str += "use crate::{
    color::{RGBPrimaries, WhitePoint},
    FType, Mat3,
};";
    let mut const_matches = String::with_capacity(conversions.len() * 128);
    for conversion in conversions {
        let src = conversion.input_space();
        let dst = conversion.output_space();
        let mat = conversion.matrix();
        let from_name = format!("{:?}_{:?}", src.primaries(), src.white_point());
        let to_name = format!("{:?}_{:?}", dst.primaries(), dst.white_point());
        out_str += &format!(
            "#[rustfmt::skip]
pub const {}_TO_{}: [FType; 9] = [
    {:?}, {:?}, {:?},
    {:?}, {:?}, {:?},
    {:?}, {:?}, {:?}];
\n
",
            from_name,
            to_name,
            mat.x_axis.x,
            mat.y_axis.x,
            mat.z_axis.x,
            mat.x_axis.y,
            mat.y_axis.y,
            mat.z_axis.y,
            mat.x_axis.z,
            mat.y_axis.z,
            mat.z_axis.z,
        );

        const_matches += &format!(
            "
        (RGBPrimaries::{:?}, WhitePoint::{:?}, RGBPrimaries::{:?}, WhitePoint::{:?}) => {{
            Some(Mat3::from_cols_array(&{}_TO_{}).transpose())
        }}",
            src.primaries(),
            src.white_point(),
            dst.primaries(),
            dst.white_point(),
            from_name,
            to_name
        );
    }

    out_str += &format!(
        r#"
pub fn const_conversion_matrix(
    src_primaries: RGBPrimaries,
    src_wp: WhitePoint,
    dst_primaries: RGBPrimaries,
    dst_wp: WhitePoint,
) -> Option<Mat3> {{
    if src_primaries == dst_primaries && src_wp == dst_wp {{
        return Some(Mat3::IDENTITY);
    }}
    match (src_primaries, src_wp, dst_primaries, dst_wp) {{
{}
        _ => None,
    }}
}}"#,
        const_matches
    );

    std::fs::write("../src/generated_matrices.rs", out_str).unwrap();
}
