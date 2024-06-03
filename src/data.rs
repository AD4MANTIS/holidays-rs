mod de;

use std::collections::HashSet;

use crate::{Error, Holiday, HolidayMap, HolidayPerCountryMap, Result, Year};

/// Two-letter country codes defined in ISO 3166-1 alpha-2 .
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Country {
    #[cfg(feature = "AO")]
    /// Angola
    AO,
    #[cfg(feature = "AR")]
    /// Argentina
    AR,
    #[cfg(feature = "AM")]
    /// Armenia
    AM,
    #[cfg(feature = "AW")]
    /// Aruba
    AW,
    #[cfg(feature = "AU")]
    /// Australia
    AU,
    #[cfg(feature = "AT")]
    /// Austria
    AT,
    #[cfg(feature = "AZ")]
    /// Azerbaijan
    AZ,
    #[cfg(feature = "BD")]
    /// Bangladesh
    BD,
    #[cfg(feature = "BY")]
    /// Belarus
    BY,
    #[cfg(feature = "BE")]
    /// Belgium
    BE,
    #[cfg(feature = "BO")]
    /// Bolivia
    BO,
    #[cfg(feature = "BA")]
    /// Bosnia and Herzegovina
    BA,
    #[cfg(feature = "BW")]
    /// Botswana
    BW,
    #[cfg(feature = "BR")]
    /// Brazil
    BR,
    #[cfg(feature = "BG")]
    /// Bulgaria
    BG,
    #[cfg(feature = "BI")]
    /// Burundi
    BI,
    #[cfg(feature = "CA")]
    /// Canada
    CA,
    #[cfg(feature = "CL")]
    /// Chile
    CL,
    #[cfg(feature = "CN")]
    /// China
    CN,
    #[cfg(feature = "CO")]
    /// Colombia
    CO,
    #[cfg(feature = "HR")]
    /// Croatia
    HR,
    #[cfg(feature = "CU")]
    /// Cuba
    CU,
    #[cfg(feature = "CW")]
    /// Curaçao
    CW,
    #[cfg(feature = "CY")]
    /// Cyprus
    CY,
    #[cfg(feature = "CZ")]
    /// Czechia
    CZ,
    #[cfg(feature = "DK")]
    /// Denmark
    DK,
    #[cfg(feature = "DJ")]
    /// Djibouti
    DJ,
    #[cfg(feature = "DO")]
    /// Dominican Republic
    DO,
    #[cfg(feature = "EG")]
    /// Egypt
    EG,
    #[cfg(feature = "EE")]
    /// Estonia
    EE,
    #[cfg(feature = "ET")]
    /// Ethiopia
    ET,
    #[cfg(feature = "FI")]
    /// Finland
    FI,
    #[cfg(feature = "FR")]
    /// France
    FR,
    #[cfg(feature = "GE")]
    /// Georgia
    GE,
    #[cfg(feature = "DE")]
    /// Germany
    DE,
    #[cfg(feature = "GR")]
    /// Greece
    GR,
    #[cfg(feature = "HN")]
    /// Honduras
    HN,
    #[cfg(feature = "HK")]
    /// Hong Kong
    HK,
    #[cfg(feature = "HU")]
    /// Hungary
    HU,
    #[cfg(feature = "IS")]
    /// Iceland
    IS,
    #[cfg(feature = "IN")]
    /// India
    IN,
    #[cfg(feature = "ID")]
    /// Indonesia
    ID,
    #[cfg(feature = "IE")]
    /// Ireland
    IE,
    #[cfg(feature = "IM")]
    /// Isle of Man
    IM,
    #[cfg(feature = "IL")]
    /// Israel
    IL,
    #[cfg(feature = "IT")]
    /// Italy
    IT,
    #[cfg(feature = "JM")]
    /// Jamaica
    JM,
    #[cfg(feature = "JP")]
    /// Japan
    JP,
    #[cfg(feature = "KZ")]
    /// Kazakhstan
    KZ,
    #[cfg(feature = "KE")]
    /// Kenya
    KE,
    #[cfg(feature = "LV")]
    /// Latvia
    LV,
    #[cfg(feature = "LS")]
    /// Lesotho
    LS,
    #[cfg(feature = "LI")]
    /// Liechtenstein
    LI,
    #[cfg(feature = "LT")]
    /// Lithuania
    LT,
    #[cfg(feature = "LU")]
    /// Luxembourg
    LU,
    #[cfg(feature = "MG")]
    /// Madagascar
    MG,
    #[cfg(feature = "MY")]
    /// Malaysia
    MY,
    #[cfg(feature = "MW")]
    /// Malawi
    MW,
    #[cfg(feature = "MT")]
    /// Malta
    MT,
    #[cfg(feature = "MX")]
    /// Mexico
    MX,
    #[cfg(feature = "MD")]
    /// Moldova
    MD,
    #[cfg(feature = "MA")]
    /// Morocco
    MA,
    #[cfg(feature = "MZ")]
    /// Mozambique
    MZ,
    #[cfg(feature = "NL")]
    /// Netherlands
    NL,
    #[cfg(feature = "NA")]
    /// Namibia
    NA,
    #[cfg(feature = "NZ")]
    /// New Zealand
    NZ,
    #[cfg(feature = "NI")]
    /// Nicaragua
    NI,
    #[cfg(feature = "NG")]
    /// Nigeria
    NG,
    #[cfg(feature = "MK")]
    /// North Macedonia
    MK,
    #[cfg(feature = "NO")]
    /// Norway
    NO,
    #[cfg(feature = "PK")]
    /// Pakistan
    PK,
    #[cfg(feature = "PY")]
    /// Paraguay
    PY,
    #[cfg(feature = "PE")]
    /// Peru
    PE,
    #[cfg(feature = "PL")]
    /// Poland
    PL,
    #[cfg(feature = "PT")]
    /// Portugal
    PT,
    #[cfg(feature = "RO")]
    /// Romania
    RO,
    #[cfg(feature = "RU")]
    /// Russia
    RU,
    #[cfg(feature = "SA")]
    /// Saudi Arabia
    SA,
    #[cfg(feature = "RS")]
    /// Serbia
    RS,
    #[cfg(feature = "SG")]
    /// Singapore
    SG,
    #[cfg(feature = "SK")]
    /// Slovakia
    SK,
    #[cfg(feature = "SI")]
    /// Slovenia
    SI,
    #[cfg(feature = "ZA")]
    /// South Africa
    ZA,
    #[cfg(feature = "KR")]
    /// South Korea
    KR,
    #[cfg(feature = "ES")]
    /// Spain
    ES,
    #[cfg(feature = "SZ")]
    /// Swaziland
    SZ,
    #[cfg(feature = "SE")]
    /// Sweden
    SE,
    #[cfg(feature = "CH")]
    /// Switzerland
    CH,
    #[cfg(feature = "TW")]
    /// Taiwan
    TW,
    #[cfg(feature = "TR")]
    /// Turkey
    TR,
    #[cfg(feature = "TN")]
    /// Tunisia
    TN,
    #[cfg(feature = "UA")]
    /// Ukraine
    UA,
    #[cfg(feature = "AE")]
    /// United Arab Emirates
    AE,
    #[cfg(feature = "GB")]
    /// United Kingdom
    GB,
    #[cfg(feature = "US")]
    /// United States
    US,
    #[cfg(feature = "UY")]
    /// Uruguay
    UY,
    #[cfg(feature = "UZ")]
    /// Uzbekistan
    UZ,
    #[cfg(feature = "VE")]
    /// Venezuela
    VE,
    #[cfg(feature = "VN")]
    /// Vietnam
    VN,
    #[cfg(feature = "ZM")]
    /// Zambia
    ZM,
    #[cfg(feature = "ZW")]
    /// Zimbabwe
    ZW,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            #[cfg(feature = "AO")]
            Country::AO => "AO".into(),
            #[cfg(feature = "AR")]
            Country::AR => "AR".into(),
            #[cfg(feature = "AM")]
            Country::AM => "AM".into(),
            #[cfg(feature = "AW")]
            Country::AW => "AW".into(),
            #[cfg(feature = "AU")]
            Country::AU => "AU".into(),
            #[cfg(feature = "AT")]
            Country::AT => "AT".into(),
            #[cfg(feature = "AZ")]
            Country::AZ => "AZ".into(),
            #[cfg(feature = "BD")]
            Country::BD => "BD".into(),
            #[cfg(feature = "BY")]
            Country::BY => "BY".into(),
            #[cfg(feature = "BE")]
            Country::BE => "BE".into(),
            #[cfg(feature = "BO")]
            Country::BO => "BO".into(),
            #[cfg(feature = "BA")]
            Country::BA => "BA".into(),
            #[cfg(feature = "BW")]
            Country::BW => "BW".into(),
            #[cfg(feature = "BR")]
            Country::BR => "BR".into(),
            #[cfg(feature = "BG")]
            Country::BG => "BG".into(),
            #[cfg(feature = "BI")]
            Country::BI => "BI".into(),
            #[cfg(feature = "CA")]
            Country::CA => "CA".into(),
            #[cfg(feature = "CL")]
            Country::CL => "CL".into(),
            #[cfg(feature = "CN")]
            Country::CN => "CN".into(),
            #[cfg(feature = "CO")]
            Country::CO => "CO".into(),
            #[cfg(feature = "HR")]
            Country::HR => "HR".into(),
            #[cfg(feature = "CU")]
            Country::CU => "CU".into(),
            #[cfg(feature = "CW")]
            Country::CW => "CW".into(),
            #[cfg(feature = "CY")]
            Country::CY => "CY".into(),
            #[cfg(feature = "CZ")]
            Country::CZ => "CZ".into(),
            #[cfg(feature = "DK")]
            Country::DK => "DK".into(),
            #[cfg(feature = "DJ")]
            Country::DJ => "DJ".into(),
            #[cfg(feature = "DO")]
            Country::DO => "DO".into(),
            #[cfg(feature = "EG")]
            Country::EG => "EG".into(),
            #[cfg(feature = "EE")]
            Country::EE => "EE".into(),
            #[cfg(feature = "ET")]
            Country::ET => "ET".into(),
            #[cfg(feature = "FI")]
            Country::FI => "FI".into(),
            #[cfg(feature = "FR")]
            Country::FR => "FR".into(),
            #[cfg(feature = "GE")]
            Country::GE => "GE".into(),
            #[cfg(feature = "DE")]
            Country::DE => "DE".into(),
            #[cfg(feature = "GR")]
            Country::GR => "GR".into(),
            #[cfg(feature = "HN")]
            Country::HN => "HN".into(),
            #[cfg(feature = "HK")]
            Country::HK => "HK".into(),
            #[cfg(feature = "HU")]
            Country::HU => "HU".into(),
            #[cfg(feature = "IS")]
            Country::IS => "IS".into(),
            #[cfg(feature = "IN")]
            Country::IN => "IN".into(),
            #[cfg(feature = "ID")]
            Country::ID => "ID".into(),
            #[cfg(feature = "IE")]
            Country::IE => "IE".into(),
            #[cfg(feature = "IM")]
            Country::IM => "IM".into(),
            #[cfg(feature = "IL")]
            Country::IL => "IL".into(),
            #[cfg(feature = "IT")]
            Country::IT => "IT".into(),
            #[cfg(feature = "JM")]
            Country::JM => "JM".into(),
            #[cfg(feature = "JP")]
            Country::JP => "JP".into(),
            #[cfg(feature = "KZ")]
            Country::KZ => "KZ".into(),
            #[cfg(feature = "KE")]
            Country::KE => "KE".into(),
            #[cfg(feature = "LV")]
            Country::LV => "LV".into(),
            #[cfg(feature = "LS")]
            Country::LS => "LS".into(),
            #[cfg(feature = "LI")]
            Country::LI => "LI".into(),
            #[cfg(feature = "LT")]
            Country::LT => "LT".into(),
            #[cfg(feature = "LU")]
            Country::LU => "LU".into(),
            #[cfg(feature = "MG")]
            Country::MG => "MG".into(),
            #[cfg(feature = "MY")]
            Country::MY => "MY".into(),
            #[cfg(feature = "MW")]
            Country::MW => "MW".into(),
            #[cfg(feature = "MT")]
            Country::MT => "MT".into(),
            #[cfg(feature = "MX")]
            Country::MX => "MX".into(),
            #[cfg(feature = "MD")]
            Country::MD => "MD".into(),
            #[cfg(feature = "MA")]
            Country::MA => "MA".into(),
            #[cfg(feature = "MZ")]
            Country::MZ => "MZ".into(),
            #[cfg(feature = "NL")]
            Country::NL => "NL".into(),
            #[cfg(feature = "NA")]
            Country::NA => "NA".into(),
            #[cfg(feature = "NZ")]
            Country::NZ => "NZ".into(),
            #[cfg(feature = "NI")]
            Country::NI => "NI".into(),
            #[cfg(feature = "NG")]
            Country::NG => "NG".into(),
            #[cfg(feature = "MK")]
            Country::MK => "MK".into(),
            #[cfg(feature = "NO")]
            Country::NO => "NO".into(),
            #[cfg(feature = "PK")]
            Country::PK => "PK".into(),
            #[cfg(feature = "PY")]
            Country::PY => "PY".into(),
            #[cfg(feature = "PE")]
            Country::PE => "PE".into(),
            #[cfg(feature = "PL")]
            Country::PL => "PL".into(),
            #[cfg(feature = "PT")]
            Country::PT => "PT".into(),
            #[cfg(feature = "RO")]
            Country::RO => "RO".into(),
            #[cfg(feature = "RU")]
            Country::RU => "RU".into(),
            #[cfg(feature = "SA")]
            Country::SA => "SA".into(),
            #[cfg(feature = "RS")]
            Country::RS => "RS".into(),
            #[cfg(feature = "SG")]
            Country::SG => "SG".into(),
            #[cfg(feature = "SK")]
            Country::SK => "SK".into(),
            #[cfg(feature = "SI")]
            Country::SI => "SI".into(),
            #[cfg(feature = "ZA")]
            Country::ZA => "ZA".into(),
            #[cfg(feature = "KR")]
            Country::KR => "KR".into(),
            #[cfg(feature = "ES")]
            Country::ES => "ES".into(),
            #[cfg(feature = "SZ")]
            Country::SZ => "SZ".into(),
            #[cfg(feature = "SE")]
            Country::SE => "SE".into(),
            #[cfg(feature = "CH")]
            Country::CH => "CH".into(),
            #[cfg(feature = "TW")]
            Country::TW => "TW".into(),
            #[cfg(feature = "TR")]
            Country::TR => "TR".into(),
            #[cfg(feature = "TN")]
            Country::TN => "TN".into(),
            #[cfg(feature = "UA")]
            Country::UA => "UA".into(),
            #[cfg(feature = "AE")]
            Country::AE => "AE".into(),
            #[cfg(feature = "GB")]
            Country::GB => "GB".into(),
            #[cfg(feature = "US")]
            Country::US => "US".into(),
            #[cfg(feature = "UY")]
            Country::UY => "UY".into(),
            #[cfg(feature = "UZ")]
            Country::UZ => "UZ".into(),
            #[cfg(feature = "VE")]
            Country::VE => "VE".into(),
            #[cfg(feature = "VN")]
            Country::VN => "VN".into(),
            #[cfg(feature = "ZM")]
            Country::ZM => "ZM".into(),
            #[cfg(feature = "ZW")]
            Country::ZW => "ZW".into(),
        }
    }
}

impl AsRef<str> for Country {
    fn as_ref(&self) -> &str {
        match self {
            #[cfg(feature = "AO")]
            Country::AO => "AO",
            #[cfg(feature = "AR")]
            Country::AR => "AR",
            #[cfg(feature = "AM")]
            Country::AM => "AM",
            #[cfg(feature = "AW")]
            Country::AW => "AW",
            #[cfg(feature = "AU")]
            Country::AU => "AU",
            #[cfg(feature = "AT")]
            Country::AT => "AT",
            #[cfg(feature = "AZ")]
            Country::AZ => "AZ",
            #[cfg(feature = "BD")]
            Country::BD => "BD",
            #[cfg(feature = "BY")]
            Country::BY => "BY",
            #[cfg(feature = "BE")]
            Country::BE => "BE",
            #[cfg(feature = "BO")]
            Country::BO => "BO",
            #[cfg(feature = "BA")]
            Country::BA => "BA",
            #[cfg(feature = "BW")]
            Country::BW => "BW",
            #[cfg(feature = "BR")]
            Country::BR => "BR",
            #[cfg(feature = "BG")]
            Country::BG => "BG",
            #[cfg(feature = "BI")]
            Country::BI => "BI",
            #[cfg(feature = "CA")]
            Country::CA => "CA",
            #[cfg(feature = "CL")]
            Country::CL => "CL",
            #[cfg(feature = "CN")]
            Country::CN => "CN",
            #[cfg(feature = "CO")]
            Country::CO => "CO",
            #[cfg(feature = "HR")]
            Country::HR => "HR",
            #[cfg(feature = "CU")]
            Country::CU => "CU",
            #[cfg(feature = "CW")]
            Country::CW => "CW",
            #[cfg(feature = "CY")]
            Country::CY => "CY",
            #[cfg(feature = "CZ")]
            Country::CZ => "CZ",
            #[cfg(feature = "DK")]
            Country::DK => "DK",
            #[cfg(feature = "DJ")]
            Country::DJ => "DJ",
            #[cfg(feature = "DO")]
            Country::DO => "DO",
            #[cfg(feature = "EG")]
            Country::EG => "EG",
            #[cfg(feature = "EE")]
            Country::EE => "EE",
            #[cfg(feature = "ET")]
            Country::ET => "ET",
            #[cfg(feature = "FI")]
            Country::FI => "FI",
            #[cfg(feature = "FR")]
            Country::FR => "FR",
            #[cfg(feature = "GE")]
            Country::GE => "GE",
            #[cfg(feature = "DE")]
            Country::DE => "DE",
            #[cfg(feature = "GR")]
            Country::GR => "GR",
            #[cfg(feature = "HN")]
            Country::HN => "HN",
            #[cfg(feature = "HK")]
            Country::HK => "HK",
            #[cfg(feature = "HU")]
            Country::HU => "HU",
            #[cfg(feature = "IS")]
            Country::IS => "IS",
            #[cfg(feature = "IN")]
            Country::IN => "IN",
            #[cfg(feature = "ID")]
            Country::ID => "ID",
            #[cfg(feature = "IE")]
            Country::IE => "IE",
            #[cfg(feature = "IM")]
            Country::IM => "IM",
            #[cfg(feature = "IL")]
            Country::IL => "IL",
            #[cfg(feature = "IT")]
            Country::IT => "IT",
            #[cfg(feature = "JM")]
            Country::JM => "JM",
            #[cfg(feature = "JP")]
            Country::JP => "JP",
            #[cfg(feature = "KZ")]
            Country::KZ => "KZ",
            #[cfg(feature = "KE")]
            Country::KE => "KE",
            #[cfg(feature = "LV")]
            Country::LV => "LV",
            #[cfg(feature = "LS")]
            Country::LS => "LS",
            #[cfg(feature = "LI")]
            Country::LI => "LI",
            #[cfg(feature = "LT")]
            Country::LT => "LT",
            #[cfg(feature = "LU")]
            Country::LU => "LU",
            #[cfg(feature = "MG")]
            Country::MG => "MG",
            #[cfg(feature = "MY")]
            Country::MY => "MY",
            #[cfg(feature = "MW")]
            Country::MW => "MW",
            #[cfg(feature = "MT")]
            Country::MT => "MT",
            #[cfg(feature = "MX")]
            Country::MX => "MX",
            #[cfg(feature = "MD")]
            Country::MD => "MD",
            #[cfg(feature = "MA")]
            Country::MA => "MA",
            #[cfg(feature = "MZ")]
            Country::MZ => "MZ",
            #[cfg(feature = "NL")]
            Country::NL => "NL",
            #[cfg(feature = "NA")]
            Country::NA => "NA",
            #[cfg(feature = "NZ")]
            Country::NZ => "NZ",
            #[cfg(feature = "NI")]
            Country::NI => "NI",
            #[cfg(feature = "NG")]
            Country::NG => "NG",
            #[cfg(feature = "MK")]
            Country::MK => "MK",
            #[cfg(feature = "NO")]
            Country::NO => "NO",
            #[cfg(feature = "PK")]
            Country::PK => "PK",
            #[cfg(feature = "PY")]
            Country::PY => "PY",
            #[cfg(feature = "PE")]
            Country::PE => "PE",
            #[cfg(feature = "PL")]
            Country::PL => "PL",
            #[cfg(feature = "PT")]
            Country::PT => "PT",
            #[cfg(feature = "RO")]
            Country::RO => "RO",
            #[cfg(feature = "RU")]
            Country::RU => "RU",
            #[cfg(feature = "SA")]
            Country::SA => "SA",
            #[cfg(feature = "RS")]
            Country::RS => "RS",
            #[cfg(feature = "SG")]
            Country::SG => "SG",
            #[cfg(feature = "SK")]
            Country::SK => "SK",
            #[cfg(feature = "SI")]
            Country::SI => "SI",
            #[cfg(feature = "ZA")]
            Country::ZA => "ZA",
            #[cfg(feature = "KR")]
            Country::KR => "KR",
            #[cfg(feature = "ES")]
            Country::ES => "ES",
            #[cfg(feature = "SZ")]
            Country::SZ => "SZ",
            #[cfg(feature = "SE")]
            Country::SE => "SE",
            #[cfg(feature = "CH")]
            Country::CH => "CH",
            #[cfg(feature = "TW")]
            Country::TW => "TW",
            #[cfg(feature = "TR")]
            Country::TR => "TR",
            #[cfg(feature = "TN")]
            Country::TN => "TN",
            #[cfg(feature = "UA")]
            Country::UA => "UA",
            #[cfg(feature = "AE")]
            Country::AE => "AE",
            #[cfg(feature = "GB")]
            Country::GB => "GB",
            #[cfg(feature = "US")]
            Country::US => "US",
            #[cfg(feature = "UY")]
            Country::UY => "UY",
            #[cfg(feature = "UZ")]
            Country::UZ => "UZ",
            #[cfg(feature = "VE")]
            Country::VE => "VE",
            #[cfg(feature = "VN")]
            Country::VN => "VN",
            #[cfg(feature = "ZM")]
            Country::ZM => "ZM",
            #[cfg(feature = "ZW")]
            Country::ZW => "ZW",
        }
    }
}

impl std::str::FromStr for Country {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "AO")]
            "AO" => Ok(Country::AO),
            #[cfg(feature = "AR")]
            "AR" => Ok(Country::AR),
            #[cfg(feature = "AM")]
            "AM" => Ok(Country::AM),
            #[cfg(feature = "AW")]
            "AW" => Ok(Country::AW),
            #[cfg(feature = "AU")]
            "AU" => Ok(Country::AU),
            #[cfg(feature = "AT")]
            "AT" => Ok(Country::AT),
            #[cfg(feature = "AZ")]
            "AZ" => Ok(Country::AZ),
            #[cfg(feature = "BD")]
            "BD" => Ok(Country::BD),
            #[cfg(feature = "BY")]
            "BY" => Ok(Country::BY),
            #[cfg(feature = "BE")]
            "BE" => Ok(Country::BE),
            #[cfg(feature = "BO")]
            "BO" => Ok(Country::BO),
            #[cfg(feature = "BA")]
            "BA" => Ok(Country::BA),
            #[cfg(feature = "BW")]
            "BW" => Ok(Country::BW),
            #[cfg(feature = "BR")]
            "BR" => Ok(Country::BR),
            #[cfg(feature = "BG")]
            "BG" => Ok(Country::BG),
            #[cfg(feature = "BI")]
            "BI" => Ok(Country::BI),
            #[cfg(feature = "CA")]
            "CA" => Ok(Country::CA),
            #[cfg(feature = "CL")]
            "CL" => Ok(Country::CL),
            #[cfg(feature = "CN")]
            "CN" => Ok(Country::CN),
            #[cfg(feature = "CO")]
            "CO" => Ok(Country::CO),
            #[cfg(feature = "HR")]
            "HR" => Ok(Country::HR),
            #[cfg(feature = "CU")]
            "CU" => Ok(Country::CU),
            #[cfg(feature = "CW")]
            "CW" => Ok(Country::CW),
            #[cfg(feature = "CY")]
            "CY" => Ok(Country::CY),
            #[cfg(feature = "CZ")]
            "CZ" => Ok(Country::CZ),
            #[cfg(feature = "DK")]
            "DK" => Ok(Country::DK),
            #[cfg(feature = "DJ")]
            "DJ" => Ok(Country::DJ),
            #[cfg(feature = "DO")]
            "DO" => Ok(Country::DO),
            #[cfg(feature = "EG")]
            "EG" => Ok(Country::EG),
            #[cfg(feature = "EE")]
            "EE" => Ok(Country::EE),
            #[cfg(feature = "ET")]
            "ET" => Ok(Country::ET),
            #[cfg(feature = "FI")]
            "FI" => Ok(Country::FI),
            #[cfg(feature = "FR")]
            "FR" => Ok(Country::FR),
            #[cfg(feature = "GE")]
            "GE" => Ok(Country::GE),
            #[cfg(feature = "DE")]
            "DE" => Ok(Country::DE),
            #[cfg(feature = "GR")]
            "GR" => Ok(Country::GR),
            #[cfg(feature = "HN")]
            "HN" => Ok(Country::HN),
            #[cfg(feature = "HK")]
            "HK" => Ok(Country::HK),
            #[cfg(feature = "HU")]
            "HU" => Ok(Country::HU),
            #[cfg(feature = "IS")]
            "IS" => Ok(Country::IS),
            #[cfg(feature = "IN")]
            "IN" => Ok(Country::IN),
            #[cfg(feature = "ID")]
            "ID" => Ok(Country::ID),
            #[cfg(feature = "IE")]
            "IE" => Ok(Country::IE),
            #[cfg(feature = "IM")]
            "IM" => Ok(Country::IM),
            #[cfg(feature = "IL")]
            "IL" => Ok(Country::IL),
            #[cfg(feature = "IT")]
            "IT" => Ok(Country::IT),
            #[cfg(feature = "JM")]
            "JM" => Ok(Country::JM),
            #[cfg(feature = "JP")]
            "JP" => Ok(Country::JP),
            #[cfg(feature = "KZ")]
            "KZ" => Ok(Country::KZ),
            #[cfg(feature = "KE")]
            "KE" => Ok(Country::KE),
            #[cfg(feature = "LV")]
            "LV" => Ok(Country::LV),
            #[cfg(feature = "LS")]
            "LS" => Ok(Country::LS),
            #[cfg(feature = "LI")]
            "LI" => Ok(Country::LI),
            #[cfg(feature = "LT")]
            "LT" => Ok(Country::LT),
            #[cfg(feature = "LU")]
            "LU" => Ok(Country::LU),
            #[cfg(feature = "MG")]
            "MG" => Ok(Country::MG),
            #[cfg(feature = "MY")]
            "MY" => Ok(Country::MY),
            #[cfg(feature = "MW")]
            "MW" => Ok(Country::MW),
            #[cfg(feature = "MT")]
            "MT" => Ok(Country::MT),
            #[cfg(feature = "MX")]
            "MX" => Ok(Country::MX),
            #[cfg(feature = "MD")]
            "MD" => Ok(Country::MD),
            #[cfg(feature = "MA")]
            "MA" => Ok(Country::MA),
            #[cfg(feature = "MZ")]
            "MZ" => Ok(Country::MZ),
            #[cfg(feature = "NL")]
            "NL" => Ok(Country::NL),
            #[cfg(feature = "NA")]
            "NA" => Ok(Country::NA),
            #[cfg(feature = "NZ")]
            "NZ" => Ok(Country::NZ),
            #[cfg(feature = "NI")]
            "NI" => Ok(Country::NI),
            #[cfg(feature = "NG")]
            "NG" => Ok(Country::NG),
            #[cfg(feature = "MK")]
            "MK" => Ok(Country::MK),
            #[cfg(feature = "NO")]
            "NO" => Ok(Country::NO),
            #[cfg(feature = "PK")]
            "PK" => Ok(Country::PK),
            #[cfg(feature = "PY")]
            "PY" => Ok(Country::PY),
            #[cfg(feature = "PE")]
            "PE" => Ok(Country::PE),
            #[cfg(feature = "PL")]
            "PL" => Ok(Country::PL),
            #[cfg(feature = "PT")]
            "PT" => Ok(Country::PT),
            #[cfg(feature = "RO")]
            "RO" => Ok(Country::RO),
            #[cfg(feature = "RU")]
            "RU" => Ok(Country::RU),
            #[cfg(feature = "SA")]
            "SA" => Ok(Country::SA),
            #[cfg(feature = "RS")]
            "RS" => Ok(Country::RS),
            #[cfg(feature = "SG")]
            "SG" => Ok(Country::SG),
            #[cfg(feature = "SK")]
            "SK" => Ok(Country::SK),
            #[cfg(feature = "SI")]
            "SI" => Ok(Country::SI),
            #[cfg(feature = "ZA")]
            "ZA" => Ok(Country::ZA),
            #[cfg(feature = "KR")]
            "KR" => Ok(Country::KR),
            #[cfg(feature = "ES")]
            "ES" => Ok(Country::ES),
            #[cfg(feature = "SZ")]
            "SZ" => Ok(Country::SZ),
            #[cfg(feature = "SE")]
            "SE" => Ok(Country::SE),
            #[cfg(feature = "CH")]
            "CH" => Ok(Country::CH),
            #[cfg(feature = "TW")]
            "TW" => Ok(Country::TW),
            #[cfg(feature = "TR")]
            "TR" => Ok(Country::TR),
            #[cfg(feature = "TN")]
            "TN" => Ok(Country::TN),
            #[cfg(feature = "UA")]
            "UA" => Ok(Country::UA),
            #[cfg(feature = "AE")]
            "AE" => Ok(Country::AE),
            #[cfg(feature = "GB")]
            "GB" => Ok(Country::GB),
            #[cfg(feature = "US")]
            "US" => Ok(Country::US),
            #[cfg(feature = "UY")]
            "UY" => Ok(Country::UY),
            #[cfg(feature = "UZ")]
            "UZ" => Ok(Country::UZ),
            #[cfg(feature = "VE")]
            "VE" => Ok(Country::VE),
            #[cfg(feature = "VN")]
            "VN" => Ok(Country::VN),
            #[cfg(feature = "ZM")]
            "ZM" => Ok(Country::ZM),
            #[cfg(feature = "ZW")]
            "ZW" => Ok(Country::ZW),
            _ => Err(Error::CountryNotAvailable),
        }
    }
}

/// Generate holiday map for the specified countries and years.
pub(crate) fn build(
    countries: Option<&HashSet<Country>>,
    years: Option<&std::ops::Range<Year>>,
) -> Result<HolidayMap> {
    let mut map = HolidayMap::new();

    #[cfg(feature = "AO")]
    if countries.is_none() || countries.unwrap().contains(&Country::AO) {
        map.insert(Country::AO, ao::build(&years)?);
    }

    #[cfg(feature = "AR")]
    if countries.is_none() || countries.unwrap().contains(&Country::AR) {
        map.insert(Country::AR, ar::build(&years)?);
    }

    #[cfg(feature = "AM")]
    if countries.is_none() || countries.unwrap().contains(&Country::AM) {
        map.insert(Country::AM, am::build(&years)?);
    }

    #[cfg(feature = "AW")]
    if countries.is_none() || countries.unwrap().contains(&Country::AW) {
        map.insert(Country::AW, aw::build(&years)?);
    }

    #[cfg(feature = "AU")]
    if countries.is_none() || countries.unwrap().contains(&Country::AU) {
        map.insert(Country::AU, au::build(&years)?);
    }

    #[cfg(feature = "AT")]
    if countries.is_none() || countries.unwrap().contains(&Country::AT) {
        map.insert(Country::AT, at::build(&years)?);
    }

    #[cfg(feature = "AZ")]
    if countries.is_none() || countries.unwrap().contains(&Country::AZ) {
        map.insert(Country::AZ, az::build(&years)?);
    }

    #[cfg(feature = "BD")]
    if countries.is_none() || countries.unwrap().contains(&Country::BD) {
        map.insert(Country::BD, bd::build(&years)?);
    }

    #[cfg(feature = "BY")]
    if countries.is_none() || countries.unwrap().contains(&Country::BY) {
        map.insert(Country::BY, by::build(&years)?);
    }

    #[cfg(feature = "BE")]
    if countries.is_none() || countries.unwrap().contains(&Country::BE) {
        map.insert(Country::BE, be::build(&years)?);
    }

    #[cfg(feature = "BO")]
    if countries.is_none() || countries.unwrap().contains(&Country::BO) {
        map.insert(Country::BO, bo::build(&years)?);
    }

    #[cfg(feature = "BA")]
    if countries.is_none() || countries.unwrap().contains(&Country::BA) {
        map.insert(Country::BA, ba::build(&years)?);
    }

    #[cfg(feature = "BW")]
    if countries.is_none() || countries.unwrap().contains(&Country::BW) {
        map.insert(Country::BW, bw::build(&years)?);
    }

    #[cfg(feature = "BR")]
    if countries.is_none() || countries.unwrap().contains(&Country::BR) {
        map.insert(Country::BR, br::build(&years)?);
    }

    #[cfg(feature = "BG")]
    if countries.is_none() || countries.unwrap().contains(&Country::BG) {
        map.insert(Country::BG, bg::build(&years)?);
    }

    #[cfg(feature = "BI")]
    if countries.is_none() || countries.unwrap().contains(&Country::BI) {
        map.insert(Country::BI, bi::build(&years)?);
    }

    #[cfg(feature = "CA")]
    if countries.is_none() || countries.unwrap().contains(&Country::CA) {
        map.insert(Country::CA, ca::build(&years)?);
    }

    #[cfg(feature = "CL")]
    if countries.is_none() || countries.unwrap().contains(&Country::CL) {
        map.insert(Country::CL, cl::build(&years)?);
    }

    #[cfg(feature = "CN")]
    if countries.is_none() || countries.unwrap().contains(&Country::CN) {
        map.insert(Country::CN, cn::build(&years)?);
    }

    #[cfg(feature = "CO")]
    if countries.is_none() || countries.unwrap().contains(&Country::CO) {
        map.insert(Country::CO, co::build(&years)?);
    }

    #[cfg(feature = "HR")]
    if countries.is_none() || countries.unwrap().contains(&Country::HR) {
        map.insert(Country::HR, hr::build(&years)?);
    }

    #[cfg(feature = "CU")]
    if countries.is_none() || countries.unwrap().contains(&Country::CU) {
        map.insert(Country::CU, cu::build(&years)?);
    }

    #[cfg(feature = "CW")]
    if countries.is_none() || countries.unwrap().contains(&Country::CW) {
        map.insert(Country::CW, cw::build(&years)?);
    }

    #[cfg(feature = "CY")]
    if countries.is_none() || countries.unwrap().contains(&Country::CY) {
        map.insert(Country::CY, cy::build(&years)?);
    }

    #[cfg(feature = "CZ")]
    if countries.is_none() || countries.unwrap().contains(&Country::CZ) {
        map.insert(Country::CZ, cz::build(&years)?);
    }

    #[cfg(feature = "DK")]
    if countries.is_none() || countries.unwrap().contains(&Country::DK) {
        map.insert(Country::DK, dk::build(&years)?);
    }

    #[cfg(feature = "DJ")]
    if countries.is_none() || countries.unwrap().contains(&Country::DJ) {
        map.insert(Country::DJ, dj::build(&years)?);
    }

    #[cfg(feature = "DO")]
    if countries.is_none() || countries.unwrap().contains(&Country::DO) {
        map.insert(Country::DO, r#do::build(&years)?);
    }

    #[cfg(feature = "EG")]
    if countries.is_none() || countries.unwrap().contains(&Country::EG) {
        map.insert(Country::EG, eg::build(&years)?);
    }

    #[cfg(feature = "EE")]
    if countries.is_none() || countries.unwrap().contains(&Country::EE) {
        map.insert(Country::EE, ee::build(&years)?);
    }

    #[cfg(feature = "ET")]
    if countries.is_none() || countries.unwrap().contains(&Country::ET) {
        map.insert(Country::ET, et::build(&years)?);
    }

    #[cfg(feature = "FI")]
    if countries.is_none() || countries.unwrap().contains(&Country::FI) {
        map.insert(Country::FI, fi::build(&years)?);
    }

    #[cfg(feature = "FR")]
    if countries.is_none() || countries.unwrap().contains(&Country::FR) {
        map.insert(Country::FR, fr::build(&years)?);
    }

    #[cfg(feature = "GE")]
    if countries.is_none() || countries.unwrap().contains(&Country::GE) {
        map.insert(Country::GE, ge::build(&years)?);
    }

    #[cfg(feature = "DE")]
    if countries.is_none() || countries.unwrap().contains(&Country::DE) {
        map.insert(Country::DE, de::build(&years)?);
    }

    #[cfg(feature = "GR")]
    if countries.is_none() || countries.unwrap().contains(&Country::GR) {
        map.insert(Country::GR, gr::build(&years)?);
    }

    #[cfg(feature = "HN")]
    if countries.is_none() || countries.unwrap().contains(&Country::HN) {
        map.insert(Country::HN, hn::build(&years)?);
    }

    #[cfg(feature = "HK")]
    if countries.is_none() || countries.unwrap().contains(&Country::HK) {
        map.insert(Country::HK, hk::build(&years)?);
    }

    #[cfg(feature = "HU")]
    if countries.is_none() || countries.unwrap().contains(&Country::HU) {
        map.insert(Country::HU, hu::build(&years)?);
    }

    #[cfg(feature = "IS")]
    if countries.is_none() || countries.unwrap().contains(&Country::IS) {
        map.insert(Country::IS, is::build(&years)?);
    }

    #[cfg(feature = "IN")]
    if countries.is_none() || countries.unwrap().contains(&Country::IN) {
        map.insert(Country::IN, r#in::build(&years)?);
    }

    #[cfg(feature = "ID")]
    if countries.is_none() || countries.unwrap().contains(&Country::ID) {
        map.insert(Country::ID, id::build(&years)?);
    }

    #[cfg(feature = "IE")]
    if countries.is_none() || countries.unwrap().contains(&Country::IE) {
        map.insert(Country::IE, ie::build(&years)?);
    }

    #[cfg(feature = "IM")]
    if countries.is_none() || countries.unwrap().contains(&Country::IM) {
        map.insert(Country::IM, im::build(&years)?);
    }

    #[cfg(feature = "IL")]
    if countries.is_none() || countries.unwrap().contains(&Country::IL) {
        map.insert(Country::IL, il::build(&years)?);
    }

    #[cfg(feature = "IT")]
    if countries.is_none() || countries.unwrap().contains(&Country::IT) {
        map.insert(Country::IT, it::build(&years)?);
    }

    #[cfg(feature = "JM")]
    if countries.is_none() || countries.unwrap().contains(&Country::JM) {
        map.insert(Country::JM, jm::build(&years)?);
    }

    #[cfg(feature = "JP")]
    if countries.is_none() || countries.unwrap().contains(&Country::JP) {
        map.insert(Country::JP, jp::build(&years)?);
    }

    #[cfg(feature = "KZ")]
    if countries.is_none() || countries.unwrap().contains(&Country::KZ) {
        map.insert(Country::KZ, kz::build(&years)?);
    }

    #[cfg(feature = "KE")]
    if countries.is_none() || countries.unwrap().contains(&Country::KE) {
        map.insert(Country::KE, ke::build(&years)?);
    }

    #[cfg(feature = "LV")]
    if countries.is_none() || countries.unwrap().contains(&Country::LV) {
        map.insert(Country::LV, lv::build(&years)?);
    }

    #[cfg(feature = "LS")]
    if countries.is_none() || countries.unwrap().contains(&Country::LS) {
        map.insert(Country::LS, ls::build(&years)?);
    }

    #[cfg(feature = "LI")]
    if countries.is_none() || countries.unwrap().contains(&Country::LI) {
        map.insert(Country::LI, li::build(&years)?);
    }

    #[cfg(feature = "LT")]
    if countries.is_none() || countries.unwrap().contains(&Country::LT) {
        map.insert(Country::LT, lt::build(&years)?);
    }

    #[cfg(feature = "LU")]
    if countries.is_none() || countries.unwrap().contains(&Country::LU) {
        map.insert(Country::LU, lu::build(&years)?);
    }

    #[cfg(feature = "MG")]
    if countries.is_none() || countries.unwrap().contains(&Country::MG) {
        map.insert(Country::MG, mg::build(&years)?);
    }

    #[cfg(feature = "MY")]
    if countries.is_none() || countries.unwrap().contains(&Country::MY) {
        map.insert(Country::MY, my::build(&years)?);
    }

    #[cfg(feature = "MW")]
    if countries.is_none() || countries.unwrap().contains(&Country::MW) {
        map.insert(Country::MW, mw::build(&years)?);
    }

    #[cfg(feature = "MT")]
    if countries.is_none() || countries.unwrap().contains(&Country::MT) {
        map.insert(Country::MT, mt::build(&years)?);
    }

    #[cfg(feature = "MX")]
    if countries.is_none() || countries.unwrap().contains(&Country::MX) {
        map.insert(Country::MX, mx::build(&years)?);
    }

    #[cfg(feature = "MD")]
    if countries.is_none() || countries.unwrap().contains(&Country::MD) {
        map.insert(Country::MD, md::build(&years)?);
    }

    #[cfg(feature = "MA")]
    if countries.is_none() || countries.unwrap().contains(&Country::MA) {
        map.insert(Country::MA, ma::build(&years)?);
    }

    #[cfg(feature = "MZ")]
    if countries.is_none() || countries.unwrap().contains(&Country::MZ) {
        map.insert(Country::MZ, mz::build(&years)?);
    }

    #[cfg(feature = "NL")]
    if countries.is_none() || countries.unwrap().contains(&Country::NL) {
        map.insert(Country::NL, nl::build(&years)?);
    }

    #[cfg(feature = "NA")]
    if countries.is_none() || countries.unwrap().contains(&Country::NA) {
        map.insert(Country::NA, na::build(&years)?);
    }

    #[cfg(feature = "NZ")]
    if countries.is_none() || countries.unwrap().contains(&Country::NZ) {
        map.insert(Country::NZ, nz::build(&years)?);
    }

    #[cfg(feature = "NI")]
    if countries.is_none() || countries.unwrap().contains(&Country::NI) {
        map.insert(Country::NI, ni::build(&years)?);
    }

    #[cfg(feature = "NG")]
    if countries.is_none() || countries.unwrap().contains(&Country::NG) {
        map.insert(Country::NG, ng::build(&years)?);
    }

    #[cfg(feature = "MK")]
    if countries.is_none() || countries.unwrap().contains(&Country::MK) {
        map.insert(Country::MK, mk::build(&years)?);
    }

    #[cfg(feature = "NO")]
    if countries.is_none() || countries.unwrap().contains(&Country::NO) {
        map.insert(Country::NO, no::build(&years)?);
    }

    #[cfg(feature = "PK")]
    if countries.is_none() || countries.unwrap().contains(&Country::PK) {
        map.insert(Country::PK, pk::build(&years)?);
    }

    #[cfg(feature = "PY")]
    if countries.is_none() || countries.unwrap().contains(&Country::PY) {
        map.insert(Country::PY, py::build(&years)?);
    }

    #[cfg(feature = "PE")]
    if countries.is_none() || countries.unwrap().contains(&Country::PE) {
        map.insert(Country::PE, pe::build(&years)?);
    }

    #[cfg(feature = "PL")]
    if countries.is_none() || countries.unwrap().contains(&Country::PL) {
        map.insert(Country::PL, pl::build(&years)?);
    }

    #[cfg(feature = "PT")]
    if countries.is_none() || countries.unwrap().contains(&Country::PT) {
        map.insert(Country::PT, pt::build(&years)?);
    }

    #[cfg(feature = "RO")]
    if countries.is_none() || countries.unwrap().contains(&Country::RO) {
        map.insert(Country::RO, ro::build(&years)?);
    }

    #[cfg(feature = "RU")]
    if countries.is_none() || countries.unwrap().contains(&Country::RU) {
        map.insert(Country::RU, ru::build(&years)?);
    }

    #[cfg(feature = "SA")]
    if countries.is_none() || countries.unwrap().contains(&Country::SA) {
        map.insert(Country::SA, sa::build(&years)?);
    }

    #[cfg(feature = "RS")]
    if countries.is_none() || countries.unwrap().contains(&Country::RS) {
        map.insert(Country::RS, rs::build(&years)?);
    }

    #[cfg(feature = "SG")]
    if countries.is_none() || countries.unwrap().contains(&Country::SG) {
        map.insert(Country::SG, sg::build(&years)?);
    }

    #[cfg(feature = "SK")]
    if countries.is_none() || countries.unwrap().contains(&Country::SK) {
        map.insert(Country::SK, sk::build(&years)?);
    }

    #[cfg(feature = "SI")]
    if countries.is_none() || countries.unwrap().contains(&Country::SI) {
        map.insert(Country::SI, si::build(&years)?);
    }

    #[cfg(feature = "ZA")]
    if countries.is_none() || countries.unwrap().contains(&Country::ZA) {
        map.insert(Country::ZA, za::build(&years)?);
    }

    #[cfg(feature = "KR")]
    if countries.is_none() || countries.unwrap().contains(&Country::KR) {
        map.insert(Country::KR, kr::build(&years)?);
    }

    #[cfg(feature = "ES")]
    if countries.is_none() || countries.unwrap().contains(&Country::ES) {
        map.insert(Country::ES, es::build(&years)?);
    }

    #[cfg(feature = "SZ")]
    if countries.is_none() || countries.unwrap().contains(&Country::SZ) {
        map.insert(Country::SZ, sz::build(&years)?);
    }

    #[cfg(feature = "SE")]
    if countries.is_none() || countries.unwrap().contains(&Country::SE) {
        map.insert(Country::SE, se::build(&years)?);
    }

    #[cfg(feature = "CH")]
    if countries.is_none() || countries.unwrap().contains(&Country::CH) {
        map.insert(Country::CH, ch::build(&years)?);
    }

    #[cfg(feature = "TW")]
    if countries.is_none() || countries.unwrap().contains(&Country::TW) {
        map.insert(Country::TW, tw::build(&years)?);
    }

    #[cfg(feature = "TR")]
    if countries.is_none() || countries.unwrap().contains(&Country::TR) {
        map.insert(Country::TR, tr::build(&years)?);
    }

    #[cfg(feature = "TN")]
    if countries.is_none() || countries.unwrap().contains(&Country::TN) {
        map.insert(Country::TN, tn::build(&years)?);
    }

    #[cfg(feature = "UA")]
    if countries.is_none() || countries.unwrap().contains(&Country::UA) {
        map.insert(Country::UA, ua::build(&years)?);
    }

    #[cfg(feature = "AE")]
    if countries.is_none() || countries.unwrap().contains(&Country::AE) {
        map.insert(Country::AE, ae::build(&years)?);
    }

    #[cfg(feature = "GB")]
    if countries.is_none() || countries.unwrap().contains(&Country::GB) {
        map.insert(Country::GB, gb::build(&years)?);
    }

    #[cfg(feature = "US")]
    if countries.is_none() || countries.unwrap().contains(&Country::US) {
        map.insert(Country::US, us::build(&years)?);
    }

    #[cfg(feature = "UY")]
    if countries.is_none() || countries.unwrap().contains(&Country::UY) {
        map.insert(Country::UY, uy::build(&years)?);
    }

    #[cfg(feature = "UZ")]
    if countries.is_none() || countries.unwrap().contains(&Country::UZ) {
        map.insert(Country::UZ, uz::build(&years)?);
    }

    #[cfg(feature = "VE")]
    if countries.is_none() || countries.unwrap().contains(&Country::VE) {
        map.insert(Country::VE, ve::build(&years)?);
    }

    #[cfg(feature = "VN")]
    if countries.is_none() || countries.unwrap().contains(&Country::VN) {
        map.insert(Country::VN, vn::build(&years)?);
    }

    #[cfg(feature = "ZM")]
    if countries.is_none() || countries.unwrap().contains(&Country::ZM) {
        map.insert(Country::ZM, zm::build(&years)?);
    }

    #[cfg(feature = "ZW")]
    if countries.is_none() || countries.unwrap().contains(&Country::ZW) {
        map.insert(Country::ZW, zw::build(&years)?);
    }

    Ok(map)
}

fn should_build_year(years: &Option<&std::ops::Range<Year>>, year: Year) -> bool {
    years.is_none() || years.unwrap().contains(&year)
}

fn build_year(
    years: &Option<&std::ops::Range<Year>>,
    year: Year,
    holidays: impl IntoIterator<Item = Holiday>,
    map: &mut HolidayPerCountryMap,
) {
    if !should_build_year(years, year) {
        return;
    }

    let m = holidays.into_iter().map(|h| (h.date, h)).collect();

    map.insert(year, m);
}
