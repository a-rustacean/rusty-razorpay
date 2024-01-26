use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entity::CollectionEntity;

#[derive(
    Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq,
)]
pub enum Currency {
    AED,
    ALL,
    AMD,
    ARS,
    AUD,
    AWG,
    BBD,
    BDT,
    BMD,
    BND,
    BOB,
    BSD,
    BWP,
    BZD,
    CAD,
    CHF,
    CNY,
    COP,
    CRC,
    CUP,
    CZK,
    DKK,
    DOP,
    DZD,
    EGP,
    ETB,
    EUR,
    FJD,
    GBP,
    GHS,
    GIP,
    GMD,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    #[default]
    INR,
    JMD,
    KES,
    KGS,
    KHR,
    KYD,
    KZT,
    LAK,
    LKR,
    LRD,
    LSL,
    MAD,
    MDL,
    MKD,
    MMK,
    MNT,
    MOP,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    PEN,
    PGK,
    PHP,
    PKR,
    QAR,
    RUB,
    SAR,
    SCR,
    SEK,
    SGD,
    SLL,
    SOS,
    SSP,
    SVC,
    SZL,
    THB,
    TTD,
    TZS,
    USD,
    UYU,
    UZS,
    YER,
    ZAR,
    TRY,
}

pub type Object = HashMap<String, String>;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Collection<T> {
    pub entity: CollectionEntity,
    pub count: usize,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq, Eq)]
pub struct Filter {
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub from: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub to: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<usize>,
}

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq,
)]
pub enum Country {
    #[serde(alias = "bangladesh")]
    BD,
    #[serde(alias = "belgium")]
    BE,
    #[serde(alias = "burkina faso")]
    BF,
    #[serde(alias = "bulgaria")]
    BG,
    #[serde(alias = "bosnia and herzegovina")]
    BA,
    #[serde(alias = "barbados")]
    BB,
    #[serde(alias = "wallis and futuna")]
    WF,
    #[serde(alias = "saint barthelemy")]
    BL,
    #[serde(alias = "bermuda")]
    BM,
    #[serde(alias = "brunei")]
    BN,
    #[serde(alias = "bolivia")]
    BO,
    #[serde(alias = "bahrain")]
    BH,
    #[serde(alias = "burundi")]
    BI,
    #[serde(alias = "benin")]
    BJ,
    #[serde(alias = "bhutan")]
    BT,
    #[serde(alias = "jamaica")]
    JM,
    #[serde(alias = "bouvet island")]
    BV,
    #[serde(alias = "botswana")]
    BW,
    #[serde(alias = "samoa")]
    WS,
    #[serde(alias = "bonaire saint eustatius and saba")]
    BQ,
    #[serde(alias = "brazil")]
    BR,
    #[serde(alias = "bahamas")]
    BS,
    #[serde(alias = "jersey")]
    JE,
    #[serde(alias = "belarus")]
    BY,
    #[serde(alias = "belize")]
    BZ,
    #[serde(alias = "russia")]
    RU,
    #[serde(alias = "rwanda")]
    RW,
    #[serde(alias = "serbia")]
    RS,
    #[serde(alias = "east timor")]
    TL,
    #[serde(alias = "reunion")]
    RE,
    #[serde(alias = "turkmenistan")]
    TM,
    #[serde(alias = "tajikistan")]
    TJ,
    #[serde(alias = "romania")]
    RO,
    #[serde(alias = "tokelau")]
    TK,
    #[serde(alias = "guinea bissau")]
    GW,
    #[serde(alias = "guam")]
    GU,
    #[serde(alias = "guatemala")]
    GT,
    #[serde(alias = "south georgia and the south sandwich islands")]
    GS,
    #[serde(alias = "greece")]
    GR,
    #[serde(alias = "equatorial guinea")]
    GQ,
    #[serde(alias = "guadeloupe")]
    GP,
    #[serde(alias = "japan")]
    JP,
    #[serde(alias = "guyana")]
    GY,
    #[serde(alias = "guernsey")]
    GG,
    #[serde(alias = "french guiana")]
    GF,
    #[serde(alias = "georgia")]
    GE,
    #[serde(alias = "grenada")]
    GD,
    #[serde(alias = "united kingdom")]
    GB,
    #[serde(alias = "gabon")]
    GA,
    #[serde(alias = "el salvador")]
    SV,
    #[serde(alias = "guinea")]
    GN,
    #[serde(alias = "gambia")]
    GM,
    #[serde(alias = "greenland")]
    GL,
    #[serde(alias = "gibraltar")]
    GI,
    #[serde(alias = "ghana")]
    GH,
    #[serde(alias = "oman")]
    OM,
    #[serde(alias = "tunisia")]
    TN,
    #[serde(alias = "jordan")]
    JO,
    #[serde(alias = "croatia")]
    HR,
    #[serde(alias = "haiti")]
    HT,
    #[serde(alias = "hungary")]
    HU,
    #[serde(alias = "hong kong")]
    HK,
    #[serde(alias = "honduras")]
    HN,
    #[serde(alias = "heard island and mcdonald islands")]
    HM,
    #[serde(alias = "venezuela")]
    VE,
    #[serde(alias = "puerto rico")]
    PR,
    #[serde(alias = "palestinian territory")]
    PS,
    #[serde(alias = "palau")]
    PW,
    #[serde(alias = "portugal")]
    PT,
    #[serde(alias = "svalbard and jan mayen")]
    SJ,
    #[serde(alias = "paraguay")]
    PY,
    #[serde(alias = "iraq")]
    IQ,
    #[serde(alias = "panama")]
    PA,
    #[serde(alias = "french polynesia")]
    PF,
    #[serde(alias = "papua new guinea")]
    PG,
    #[serde(alias = "peru")]
    PE,
    #[serde(alias = "pakistan")]
    PK,
    #[serde(alias = "philippines")]
    PH,
    #[serde(alias = "pitcairn")]
    PN,
    #[serde(alias = "poland")]
    PL,
    #[serde(alias = "saint pierre and miquelon")]
    PM,
    #[serde(alias = "zambia")]
    ZM,
    #[serde(alias = "western sahara")]
    EH,
    #[serde(alias = "estonia")]
    EE,
    #[serde(alias = "egypt")]
    EG,
    #[serde(alias = "south africa")]
    ZA,
    #[serde(alias = "ecuador")]
    EC,
    #[serde(alias = "italy")]
    IT,
    #[serde(alias = "vietnam")]
    VN,
    #[serde(alias = "solomon islands")]
    SB,
    #[serde(alias = "ethiopia")]
    ET,
    #[serde(alias = "somalia")]
    SO,
    #[serde(alias = "zimbabwe")]
    ZW,
    #[serde(alias = "saudi arabia")]
    SA,
    #[serde(alias = "spain")]
    ES,
    #[serde(alias = "eritrea")]
    ER,
    #[serde(alias = "montenegro")]
    ME,
    #[serde(alias = "moldova")]
    MD,
    #[serde(alias = "madagascar")]
    MG,
    #[serde(alias = "saint martin")]
    MF,
    #[serde(alias = "morocco")]
    MA,
    #[serde(alias = "monaco")]
    MC,
    #[serde(alias = "uzbekistan")]
    UZ,
    #[serde(alias = "myanmar")]
    MM,
    #[serde(alias = "mali")]
    ML,
    #[serde(alias = "macao")]
    MO,
    #[serde(alias = "mongolia")]
    MN,
    #[serde(alias = "marshall islands")]
    MH,
    #[serde(alias = "macedonia")]
    MK,
    #[serde(alias = "mauritius")]
    MU,
    #[serde(alias = "malta")]
    MT,
    #[serde(alias = "malawi")]
    MW,
    #[serde(alias = "maldives")]
    MV,
    #[serde(alias = "martinique")]
    MQ,
    #[serde(alias = "northern mariana islands")]
    MP,
    #[serde(alias = "montserrat")]
    MS,
    #[serde(alias = "mauritania")]
    MR,
    #[serde(alias = "isle of man")]
    IM,
    #[serde(alias = "uganda")]
    UG,
    #[serde(alias = "tanzania")]
    TZ,
    #[serde(alias = "malaysia")]
    MY,
    #[serde(alias = "mexico")]
    MX,
    #[serde(alias = "israel")]
    IL,
    #[serde(alias = "france")]
    FR,
    #[serde(alias = "british indian ocean territory")]
    IO,
    #[serde(alias = "saint helena")]
    SH,
    #[serde(alias = "finland")]
    FI,
    #[serde(alias = "fiji")]
    FJ,
    #[serde(alias = "falkland islands")]
    FK,
    #[serde(alias = "micronesia")]
    FM,
    #[serde(alias = "faroe islands")]
    FO,
    #[serde(alias = "nicaragua")]
    NI,
    #[serde(alias = "netherlands")]
    NL,
    #[serde(alias = "norway")]
    NO,
    #[serde(alias = "namibia")]
    NA,
    #[serde(alias = "vanuatu")]
    VU,
    #[serde(alias = "new caledonia")]
    NC,
    #[serde(alias = "niger")]
    NE,
    #[serde(alias = "norfolk island")]
    NF,
    #[serde(alias = "nigeria")]
    NG,
    #[serde(alias = "new zealand")]
    NZ,
    #[serde(alias = "nepal")]
    NP,
    #[serde(alias = "nauru")]
    NR,
    #[serde(alias = "niue")]
    NU,
    #[serde(alias = "cook islands")]
    CK,
    #[serde(alias = "kosovo")]
    XK,
    #[serde(alias = "ivory coast")]
    CI,
    #[serde(alias = "switzerland")]
    CH,
    #[serde(alias = "colombia")]
    CO,
    #[serde(alias = "china")]
    CN,
    #[serde(alias = "cameroon")]
    CM,
    #[serde(alias = "chile")]
    CL,
    #[serde(alias = "cocos islands")]
    CC,
    #[serde(alias = "canada")]
    CA,
    #[serde(alias = "republic of the congo")]
    CG,
    #[serde(alias = "central african republic")]
    CF,
    #[serde(alias = "democratic republic of the congo")]
    CD,
    #[serde(alias = "czech republic")]
    CZ,
    #[serde(alias = "cyprus")]
    CY,
    #[serde(alias = "christmas island")]
    CX,
    #[serde(alias = "costa rica")]
    CR,
    #[serde(alias = "curacao")]
    CW,
    #[serde(alias = "cape verde")]
    CV,
    #[serde(alias = "cuba")]
    CU,
    #[serde(alias = "swaziland")]
    SZ,
    #[serde(alias = "syria")]
    SY,
    #[serde(alias = "sint maarten")]
    SX,
    #[serde(alias = "kyrgyzstan")]
    KG,
    #[serde(alias = "kenya")]
    KE,
    #[serde(alias = "south sudan")]
    SS,
    #[serde(alias = "suriname")]
    SR,
    #[serde(alias = "kiribati")]
    KI,
    #[serde(alias = "cambodia")]
    KH,
    #[serde(alias = "saint kitts and nevis")]
    KN,
    #[serde(alias = "comoros")]
    KM,
    #[serde(alias = "sao tome and principe")]
    ST,
    #[serde(alias = "slovakia")]
    SK,
    #[serde(alias = "south korea")]
    KR,
    #[serde(alias = "slovenia")]
    SI,
    #[serde(alias = "north korea")]
    KP,
    #[serde(alias = "kuwait")]
    KW,
    #[serde(alias = "senegal")]
    SN,
    #[serde(alias = "san marino")]
    SM,
    #[serde(alias = "sierra leone")]
    SL,
    #[serde(alias = "seychelles")]
    SC,
    #[serde(alias = "kazakhstan")]
    KZ,
    #[serde(alias = "cayman islands")]
    KY,
    #[serde(alias = "singapore")]
    SG,
    #[serde(alias = "sweden")]
    SE,
    #[serde(alias = "sudan")]
    SD,
    #[serde(alias = "dominican republic")]
    DO,
    #[serde(alias = "dominica")]
    DM,
    #[serde(alias = "djibouti")]
    DJ,
    #[serde(alias = "denmark")]
    DK,
    #[serde(alias = "british virgin islands")]
    VG,
    #[serde(alias = "germany")]
    DE,
    #[serde(alias = "yemen")]
    YE,
    #[serde(alias = "algeria")]
    DZ,
    #[serde(alias = "united states")]
    US,
    #[serde(alias = "uruguay")]
    UY,
    #[serde(alias = "mayotte")]
    YT,
    #[serde(alias = "united states minor outlying islands")]
    UM,
    #[serde(alias = "lebanon")]
    LB,
    #[serde(alias = "saint lucia")]
    LC,
    #[serde(alias = "laos")]
    LA,
    #[serde(alias = "tuvalu")]
    TV,
    #[serde(alias = "taiwan")]
    TW,
    #[serde(alias = "trinidad and tobago")]
    TT,
    #[serde(alias = "turkey")]
    TR,
    #[serde(alias = "sri lanka")]
    LK,
    #[serde(alias = "liechtenstein")]
    LI,
    #[serde(alias = "latvia")]
    LV,
    #[serde(alias = "tonga")]
    TO,
    #[serde(alias = "lithuania")]
    LT,
    #[serde(alias = "luxembourg")]
    LU,
    #[serde(alias = "liberia")]
    LR,
    #[serde(alias = "lesotho")]
    LS,
    #[serde(alias = "thailand")]
    TH,
    #[serde(alias = "french southern territories")]
    TF,
    #[serde(alias = "togo")]
    TG,
    #[serde(alias = "chad")]
    TD,
    #[serde(alias = "turks and caicos islands")]
    TC,
    #[serde(alias = "libya")]
    LY,
    #[serde(alias = "vatican")]
    VA,
    #[serde(alias = "saint vincent and the grenadines")]
    VC,
    #[serde(alias = "united arab emirates")]
    AE,
    #[serde(alias = "andorra")]
    AD,
    #[serde(alias = "antigua and barbuda")]
    AG,
    #[serde(alias = "afghanistan")]
    AF,
    #[serde(alias = "anguilla")]
    AI,
    #[serde(alias = "us virgin islands")]
    VI,
    #[serde(alias = "iceland")]
    IS,
    #[serde(alias = "iran")]
    IR,
    #[serde(alias = "armenia")]
    AM,
    #[serde(alias = "albania")]
    AL,
    #[serde(alias = "angola")]
    AO,
    #[serde(alias = "antarctica")]
    AQ,
    #[serde(alias = "american samoa")]
    AS,
    #[serde(alias = "argentina")]
    AR,
    #[serde(alias = "australia")]
    AU,
    #[serde(alias = "austria")]
    AT,
    #[serde(alias = "aruba")]
    AW,
    #[serde(alias = "india")]
    #[default]
    IN,
    #[serde(alias = "aland islands")]
    AX,
    #[serde(alias = "azerbaijan")]
    AZ,
    #[serde(alias = "ireland")]
    IE,
    #[serde(alias = "indonesia")]
    ID,
    #[serde(alias = "ukraine")]
    UA,
    #[serde(alias = "qatar")]
    QA,
    #[serde(alias = "mozambique")]
    MZ,
}
