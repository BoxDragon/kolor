use super::{
    color::{RGBPrimaries, WhitePoint},
};
use crate::{
    FType, Mat3,
};#[rustfmt::skip]
pub const BT_709_D65_TO_BT_2020_D65: [FType; 9] = [
    0.6274523942229207, 0.3292484773484975, 0.04329912842858205,
    0.06910918409232207, 0.919531079275882, 0.01135973663179622,
    0.016397562151213345, 0.088030140698202, 0.8955722971505848];


#[rustfmt::skip]
pub const BT_709_D65_TO_BT_709_D65: [FType; 9] = [
    0.9999999999999997, -0.0000000000000000971445146547012, 0.0,
    0.000000000000000042500725161431774, 0.9999999999999998, 0.000000000000000013877787807814457,
    0.000000000000000003469446951953614, 0.000000000000000027755575615628914, 1.0];


#[rustfmt::skip]
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
pub const BT_709_D65_TO_CIE_XYZ_D65: [FType; 9] = [
    0.4124564390896922, 0.357576077643909, 0.18043748326639894,
    0.21267285140562253, 0.715152155287818, 0.07217499330655958,
    0.0193338955823293, 0.11919202588130297, 0.9503040785363679];


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


#[rustfmt::skip]
pub const BT_2020_D65_TO_CIE_XYZ_D65: [FType; 9] = [
    0.6370101914111008, 0.14461502739696927, 0.16884478119192986,
    0.26272171736164046, 0.6779892755022618, 0.0592890071360975,
    0.00000000000000004994515405547192, 0.028072328847646908, 1.060757671152353];


#[rustfmt::skip]
pub const AP1_D60_TO_BT_709_D65: [FType; 9] = [
    1.70150705829719, -0.6110425859262977, -0.09046447237089245,
    -0.13072902802234107, 1.140104545849696, -0.009375517827355101,
    -0.023448377492443606, -0.1272893163664402, 1.1507376938588838];


#[rustfmt::skip]
pub const AP1_D60_TO_BT_2020_D65: [FType; 9] = [
    1.0235570497856787, -0.013533964229444466, -0.010023085556234018,
    -0.002886007096581449, 1.004686935863517, -0.0018009287669354355,
    -0.004607244286351307, -0.023652830658976578, 1.028260074945328];


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
pub const AP1_D60_TO_CIE_XYZ_D65: [FType; 9] = [
    0.65082100305539, 0.13267789859374557, 0.16697109835086446,
    0.2666808251374308, 0.6762089485818938, 0.0571102262806755,
    -0.004968186659891823, 0.003113980486518425, 1.0906842061733732];


#[rustfmt::skip]
pub const AP0_D60_TO_BT_709_D65: [FType; 9] = [
    2.5160003103784887, -1.120815791571519, -0.3951845188069694,
    -0.2770794071389051, 1.3719171373747592, -0.09483773023585473,
    -0.014731740460086115, -0.15110489612376043, 1.1658366365838464];


#[rustfmt::skip]
pub const AP0_D60_TO_BT_2020_D65: [FType; 9] = [
    1.4868045741853277, -0.2580996336789089, -0.22870494050641854,
    -0.08107174635702696, 1.1823452693102523, -0.10127352295322543,
    0.0036715936220255846, -0.03293394690523746, 1.0292623532832117];


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
pub const AP0_D60_TO_CIE_XYZ_D65: [FType; 9] = [
    0.9360054029938909, 0.0010120714020409225, 0.013452525604068349,
    0.3358677616752651, 0.7318564125095669, -0.06772417418483223,
    0.0016187983759302863, -0.0017437516095178966, 1.0889549532335876];


#[rustfmt::skip]
pub const CIE_RGB_E_TO_BT_709_D65: [FType; 9] = [
    1.2185334191192088, -0.33818327540366827, 0.1196498562844589,
    -0.14361955351160366, 1.282135043995923, -0.13851549048431913,
    -0.005420528602215063, -0.14249696827539357, 1.1479174968776085];


#[rustfmt::skip]
pub const CIE_RGB_E_TO_BT_2020_D65: [FType; 9] = [
    0.717050487791725, 0.20377711062258566, 0.07917240158568983,
    -0.047912368438415454, 1.1539727225157472, -0.10606035407733123,
    0.0024836527186421226, -0.020295190175264297, 1.0178115374566221];


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
pub const CIE_RGB_E_TO_CIE_XYZ_D65: [FType; 9] = [
    0.45025897180509056, 0.2932631561804732, 0.20694787201443635,
    0.15604791692624442, 0.834714520874453, 0.009237562199302746,
    0.0012895419111052487, 0.010866423081768984, 1.076674035007126];


#[rustfmt::skip]
pub const CIE_XYZ_D65_TO_BT_709_D65: [FType; 9] = [
    3.240454162114104, -1.5371385127977162, -0.4985314095560159,
    -0.9692660305051866, 1.8760108454466937, 0.04155601753034983,
    0.05564343095911472, -0.20402591351675378, 1.057225188223179];


#[rustfmt::skip]
pub const CIE_XYZ_D65_TO_BT_2020_D65: [FType; 9] = [
    1.7165106697619739, -0.355641669986716, -0.25334554182190727,
    -0.6666930011826243, 1.616502208346911, 0.01576875038999502,
    0.01764363876745901, -0.04277978166904463, 0.9423050727200187];


#[rustfmt::skip]
pub const CIE_XYZ_D65_TO_AP1_D60: [FType; 9] = [
    1.6683875898867835, -0.32625420396515725, -0.23832751540142538,
    -0.6587733206900565, 1.6080130379716489, 0.01665203947781438,
    0.009480533572179267, -0.006077114685137944, 0.9157225204492794];


#[rustfmt::skip]
pub const CIE_XYZ_D65_TO_AP0_D60: [FType; 9] = [
    1.06893470000754, -0.0015098980622441612, -0.013299106613449435,
    -0.4907814415963974, 1.3672839905988126, 0.09109690786928681,
    -0.0023749289879929675, 0.0021916865089035313, 0.9184772758339819];


#[rustfmt::skip]
pub const CIE_XYZ_D65_TO_CIE_RGB_E: [FType; 9] = [
    2.52796883750449, -0.8819330576680001, -0.47833498646702716,
    -0.47261710988528555, 1.3630302419998659, 0.07914747245465469,
    0.0017421574634275722, -0.012700198200724612, 0.92856031685062];



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

        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_CIE_RGB_E).transpose())
        }
        (RGBPrimaries::BT_709, WhitePoint::D65, RGBPrimaries::CIE_XYZ, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&BT_709_D65_TO_CIE_XYZ_D65).transpose())
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
        (RGBPrimaries::BT_2020, WhitePoint::D65, RGBPrimaries::CIE_XYZ, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&BT_2020_D65_TO_CIE_XYZ_D65).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_CIE_RGB_E).transpose())
        }
        (RGBPrimaries::AP1, WhitePoint::D60, RGBPrimaries::CIE_XYZ, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP1_D60_TO_CIE_XYZ_D65).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_CIE_RGB_E).transpose())
        }
        (RGBPrimaries::AP0, WhitePoint::D60, RGBPrimaries::CIE_XYZ, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&AP0_D60_TO_CIE_XYZ_D65).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::CIE_RGB, WhitePoint::E, RGBPrimaries::CIE_XYZ, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&CIE_RGB_E_TO_CIE_XYZ_D65).transpose())
        }
        (RGBPrimaries::CIE_XYZ, WhitePoint::D65, RGBPrimaries::BT_709, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&CIE_XYZ_D65_TO_BT_709_D65).transpose())
        }
        (RGBPrimaries::CIE_XYZ, WhitePoint::D65, RGBPrimaries::BT_2020, WhitePoint::D65) => {
            Some(Mat3::from_cols_array(&CIE_XYZ_D65_TO_BT_2020_D65).transpose())
        }
        (RGBPrimaries::CIE_XYZ, WhitePoint::D65, RGBPrimaries::AP1, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&CIE_XYZ_D65_TO_AP1_D60).transpose())
        }
        (RGBPrimaries::CIE_XYZ, WhitePoint::D65, RGBPrimaries::AP0, WhitePoint::D60) => {
            Some(Mat3::from_cols_array(&CIE_XYZ_D65_TO_AP0_D60).transpose())
        }
        (RGBPrimaries::CIE_XYZ, WhitePoint::D65, RGBPrimaries::CIE_RGB, WhitePoint::E) => {
            Some(Mat3::from_cols_array(&CIE_XYZ_D65_TO_CIE_RGB_E).transpose())
        }
        _ => None,
    }
}