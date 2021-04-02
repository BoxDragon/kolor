use crate::{
    color::{RGBPrimaries, WhitePoint},
    FType, Mat3,
};#[rustfmt::skip]
pub const BT_709_D65_TO_AP1_D60: [FType; 9] = [
    0.6141439932086278, 0.3348473538601437, 0.05100865293122844,
    0.07058736865623247, 0.9163972000879493, 0.01301543125581777,
    0.020322353402197846, 0.1081907727011861, 0.8714868738966164];


#[rustfmt::skip]
pub const BT_709_D65_TO_AP0_D60: [FType; 9] = [
    0.44031076211968634, 0.3795605229730174, 0.18012871490729662,
    0.09011947729436987, 0.8131824149012692, 0.09669810780436126,
    0.017244301211473752, 0.11019333877094085, 0.8725623600175858];


#[rustfmt::skip]
pub const BT_709_D65_TO_CIE_RGB_E: [FType; 9] = [
    0.8458657280419494, 0.21621113822313157, -0.0620768662650813,
    0.09647578689010416, 0.8152111904937543, 0.08831302261614174,
    0.0159702649068187, 0.10221736503498981, 0.8818123700581916];


#[rustfmt::skip]
pub const BT_709_D65_TO_BT_2020_D65: [FType; 9] = [
    0.6274523942229207, 0.3292484773484975, 0.04329912842858205,
    0.06910918409232207, 0.919531079275882, 0.01135973663179622,
    0.016397562151213345, 0.088030140698202, 0.8955722971505848];


#[rustfmt::skip]
pub const AP1_D60_TO_BT_709_D65: [FType; 9] = [
    1.70150705829719, -0.6110425859262977, -0.09046447237089245,
    -0.13072902802234107, 1.140104545849696, -0.009375517827355101,
    -0.023448377492443606, -0.1272893163664402, 1.1507376938588838];


#[rustfmt::skip]
pub const AP1_D60_TO_AP0_D60: [FType; 9] = [
    0.6953485652425724, 0.1407615899913178, 0.16388984476611007,
    0.044764966266919184, 0.8597374933493469, 0.09549754038373406,
    -0.005524339448726105, 0.004027057756628111, 1.0014972816920982];


#[rustfmt::skip]
pub const AP1_D60_TO_CIE_RGB_E: [FType; 9] = [
    1.4124370364812864, -0.2624549583886008, -0.1499820780926857,
    0.05551166869030181, 0.8592338655338057, 0.08525446577589288,
    -0.00686632764683196, -0.0054653231735984735, 1.0123316508204308];


#[rustfmt::skip]
pub const AP1_D60_TO_BT_2020_D65: [FType; 9] = [
    1.0235570497856787, -0.013533964229444466, -0.010023085556234018,
    -0.002886007096581449, 1.004686935863517, -0.0018009287669354355,
    -0.004607244286351307, -0.023652830658976578, 1.028260074945328];


#[rustfmt::skip]
pub const AP0_D60_TO_BT_709_D65: [FType; 9] = [
    2.5160003103784887, -1.120815791571519, -0.3951845188069694,
    -0.2770794071389051, 1.3719171373747592, -0.09483773023585473,
    -0.014731740460086115, -0.15110489612376043, 1.1658366365838464];


#[rustfmt::skip]
pub const AP0_D60_TO_AP1_D60: [FType; 9] = [
    1.4516557250041906, -0.2366671199242865, -0.2149886050799043,
    -0.0765086914114762, 1.1761388905796017, -0.09963019916812614,
    0.00831509386898968, -0.006034772993874998, 0.9977196791248852];


#[rustfmt::skip]
pub const AP0_D60_TO_CIE_RGB_E: [FType; 9] = [
    2.069205280578771, -0.642055781290266, -0.427149499288505,
    0.015553871876073877, 0.9969260872585566, -0.012479959134630403,
    -0.001131766411004165, -0.010912136852638905, 1.0120439032636432];


#[rustfmt::skip]
pub const AP0_D60_TO_BT_2020_D65: [FType; 9] = [
    1.4868045741853277, -0.2580996336789089, -0.22870494050641854,
    -0.08107174635702696, 1.1823452693102523, -0.10127352295322543,
    0.0036715936220255846, -0.03293394690523746, 1.0292623532832117];


#[rustfmt::skip]
pub const CIE_RGB_E_TO_BT_709_D65: [FType; 9] = [
    1.2185334191192088, -0.33818327540366827, 0.1196498562844589,
    -0.14361955351160366, 1.282135043995923, -0.13851549048431913,
    -0.005420528602215063, -0.14249696827539357, 1.1479174968776085];


#[rustfmt::skip]
pub const CIE_RGB_E_TO_AP1_D60: [FType; 9] = [
    0.6999878585579238, 0.21435772118210014, 0.08565442025997627,
    -0.04567003955800025, 1.149218837423462, -0.10354879786546178,
    0.004501236779897692, 0.007658263657653921, 0.9878404995624483];


#[rustfmt::skip]
pub const CIE_RGB_E_TO_AP0_D60: [FType; 9] = [
    0.48104467274943546, 0.3120743163183718, 0.20688101093219316,
    -0.007499455413997103, 0.9983535840969844, 0.009145871317012957,
    0.00045708997161162675, 0.011113516053665774, 0.9884293939747227];


#[rustfmt::skip]
pub const CIE_RGB_E_TO_BT_2020_D65: [FType; 9] = [
    0.717050487791725, 0.20377711062258566, 0.07917240158568983,
    -0.047912368438415454, 1.1539727225157472, -0.10606035407733123,
    0.0024836527186421226, -0.020295190175264297, 1.0178115374566221];


#[rustfmt::skip]
pub const BT_2020_D65_TO_BT_709_D65: [FType; 9] = [
    1.6603626561622697, -0.5875399968755312, -0.07282265928673903,
    -0.12456354851556833, 1.1329113745720412, -0.008347826056473065,
    -0.018156605779362994, -0.1006017318464808, 1.1187583376258436];


#[rustfmt::skip]
pub const BT_2020_D65_TO_AP1_D60: [FType; 9] = [
    0.9770658332195041, 0.013386657258545143, 0.009547509521950659,
    0.0028146277665209727, 0.9954145343208096, 0.0017708379126687088,
    0.004442606498810242, 0.022957272767006, 0.9726001207341838];


#[rustfmt::skip]
pub const BT_2020_D65_TO_AP0_D60: [FType; 9] = [
    0.6805256148458169, 0.15318698933962568, 0.16628739581455765,
    0.046582418078859995, 0.8585868129242537, 0.09483076899688637,
    -0.0009370503256916523, 0.02692628565393756, 0.9740107646717544];


#[rustfmt::skip]
pub const BT_2020_D65_TO_CIE_RGB_E: [FType; 9] = [
    1.3786389455514936, -0.24578684915486262, -0.13285209639663143,
    0.05703573036187016, 0.8579942038410319, 0.08497006579709794,
    -0.0022268458228812643, 0.017708194541964817, 0.9845186512809164];



pub fn const_conversion_matrix(
    src_primaries: RGBPrimaries,
    src_wp: WhitePoint,
    dst_primaries: RGBPrimaries,
    dst_wp: WhitePoint,
) -> Option<Mat3> {
    if src_primaries == dst_primaries && src_wp == dst_wp {
        return Some(Mat3::IDENTITY);
    }
    match (src_primaries, src_wp, dst_primaries, dst_wp) {

        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_CIE_RGB_E).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_CIE_RGB_E).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_CIE_RGB_E).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::BT_2020, WhitePoint::D65, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&BT_2020_D65_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::BT_2020, WhitePoint::D65, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&BT_2020_D65_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::BT_2020, WhitePoint::D65, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&BT_2020_D65_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::BT_2020, WhitePoint::D65, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&BT_2020_D65_TO_CIE_RGB_E).transpose())
        }
        _ => None,
    }
}