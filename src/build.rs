use std::collections::HashSet;

use crate::{data, prelude::*, HolidayMap, Result, Year};

/// Generate holiday map for the specified countries and years.
pub fn build(
    countries: Option<&HashSet<Country>>,
    years: Option<&std::ops::Range<Year>>,
) -> Result<HolidayMap> {
    let mut map = HolidayMap::new();

    #[cfg(feature = "AO")]
    if countries.map_or(true, |c| c.contains(&Country::AO)) {
        map.insert(Country::AO, data::ao::build(years)?);
    }

    #[cfg(feature = "AR")]
    if countries.map_or(true, |c| c.contains(&Country::AR)) {
        map.insert(Country::AR, data::ar::build(years)?);
    }

    #[cfg(feature = "AM")]
    if countries.map_or(true, |c| c.contains(&Country::AM)) {
        map.insert(Country::AM, data::am::build(years)?);
    }

    #[cfg(feature = "AW")]
    if countries.map_or(true, |c| c.contains(&Country::AW)) {
        map.insert(Country::AW, data::aw::build(years)?);
    }

    #[cfg(feature = "AU")]
    if countries.map_or(true, |c| c.contains(&Country::AU)) {
        map.insert(Country::AU, data::au::build(years)?);
    }

    #[cfg(feature = "AT")]
    if countries.map_or(true, |c| c.contains(&Country::AT)) {
        map.insert(Country::AT, data::at::build(years)?);
    }

    #[cfg(feature = "AZ")]
    if countries.map_or(true, |c| c.contains(&Country::AZ)) {
        map.insert(Country::AZ, data::az::build(years)?);
    }

    #[cfg(feature = "BD")]
    if countries.map_or(true, |c| c.contains(&Country::BD)) {
        map.insert(Country::BD, data::bd::build(years)?);
    }

    #[cfg(feature = "BY")]
    if countries.map_or(true, |c| c.contains(&Country::BY)) {
        map.insert(Country::BY, data::by::build(years)?);
    }

    #[cfg(feature = "BE")]
    if countries.map_or(true, |c| c.contains(&Country::BE)) {
        map.insert(Country::BE, data::be::build(years)?);
    }

    #[cfg(feature = "BO")]
    if countries.map_or(true, |c| c.contains(&Country::BO)) {
        map.insert(Country::BO, data::bo::build(years)?);
    }

    #[cfg(feature = "BA")]
    if countries.map_or(true, |c| c.contains(&Country::BA)) {
        map.insert(Country::BA, data::ba::build(years)?);
    }

    #[cfg(feature = "BW")]
    if countries.map_or(true, |c| c.contains(&Country::BW)) {
        map.insert(Country::BW, data::bw::build(years)?);
    }

    #[cfg(feature = "BR")]
    if countries.map_or(true, |c| c.contains(&Country::BR)) {
        map.insert(Country::BR, data::br::build(years)?);
    }

    #[cfg(feature = "BG")]
    if countries.map_or(true, |c| c.contains(&Country::BG)) {
        map.insert(Country::BG, data::bg::build(years)?);
    }

    #[cfg(feature = "BI")]
    if countries.map_or(true, |c| c.contains(&Country::BI)) {
        map.insert(Country::BI, data::bi::build(years)?);
    }

    #[cfg(feature = "CA")]
    if countries.map_or(true, |c| c.contains(&Country::CA)) {
        map.insert(Country::CA, data::ca::build(years)?);
    }

    #[cfg(feature = "CL")]
    if countries.map_or(true, |c| c.contains(&Country::CL)) {
        map.insert(Country::CL, data::cl::build(years)?);
    }

    #[cfg(feature = "CN")]
    if countries.map_or(true, |c| c.contains(&Country::CN)) {
        map.insert(Country::CN, data::cn::build(years)?);
    }

    #[cfg(feature = "CO")]
    if countries.map_or(true, |c| c.contains(&Country::CO)) {
        map.insert(Country::CO, data::co::build(years)?);
    }

    #[cfg(feature = "HR")]
    if countries.map_or(true, |c| c.contains(&Country::HR)) {
        map.insert(Country::HR, data::hr::build(years)?);
    }

    #[cfg(feature = "CU")]
    if countries.map_or(true, |c| c.contains(&Country::CU)) {
        map.insert(Country::CU, data::cu::build(years)?);
    }

    #[cfg(feature = "CW")]
    if countries.map_or(true, |c| c.contains(&Country::CW)) {
        map.insert(Country::CW, data::cw::build(years)?);
    }

    #[cfg(feature = "CY")]
    if countries.map_or(true, |c| c.contains(&Country::CY)) {
        map.insert(Country::CY, data::cy::build(years)?);
    }

    #[cfg(feature = "CZ")]
    if countries.map_or(true, |c| c.contains(&Country::CZ)) {
        map.insert(Country::CZ, data::cz::build(years)?);
    }

    #[cfg(feature = "DK")]
    if countries.map_or(true, |c| c.contains(&Country::DK)) {
        map.insert(Country::DK, data::dk::build(years)?);
    }

    #[cfg(feature = "DJ")]
    if countries.map_or(true, |c| c.contains(&Country::DJ)) {
        map.insert(Country::DJ, data::dj::build(years)?);
    }

    #[cfg(feature = "DO")]
    if countries.map_or(true, |c| c.contains(&Country::DO)) {
        map.insert(Country::DO, data::r#do::build(years)?);
    }

    #[cfg(feature = "EG")]
    if countries.map_or(true, |c| c.contains(&Country::EG)) {
        map.insert(Country::EG, data::eg::build(years)?);
    }

    #[cfg(feature = "EE")]
    if countries.map_or(true, |c| c.contains(&Country::EE)) {
        map.insert(Country::EE, data::ee::build(years)?);
    }

    #[cfg(feature = "ET")]
    if countries.map_or(true, |c| c.contains(&Country::ET)) {
        map.insert(Country::ET, data::et::build(years)?);
    }

    #[cfg(feature = "FI")]
    if countries.map_or(true, |c| c.contains(&Country::FI)) {
        map.insert(Country::FI, data::fi::build(years)?);
    }

    #[cfg(feature = "FR")]
    if countries.map_or(true, |c| c.contains(&Country::FR)) {
        map.insert(Country::FR, data::fr::build(years)?);
    }

    #[cfg(feature = "GE")]
    if countries.map_or(true, |c| c.contains(&Country::GE)) {
        map.insert(Country::GE, data::ge::build(years)?);
    }

    #[cfg(feature = "DE")]
    if countries.map_or(true, |c| c.contains(&Country::DE)) {
        map.insert(Country::DE, data::de::build(years)?);
    }

    #[cfg(feature = "DE")]
    if countries.map_or(true, |c| c.contains(&Country::DE_NW)) {
        map.insert(Country::DE_NW, data::de_nw::build(years)?);
    }

    #[cfg(feature = "GR")]
    if countries.map_or(true, |c| c.contains(&Country::GR)) {
        map.insert(Country::GR, data::gr::build(years)?);
    }

    #[cfg(feature = "HN")]
    if countries.map_or(true, |c| c.contains(&Country::HN)) {
        map.insert(Country::HN, data::hn::build(years)?);
    }

    #[cfg(feature = "HK")]
    if countries.map_or(true, |c| c.contains(&Country::HK)) {
        map.insert(Country::HK, data::hk::build(years)?);
    }

    #[cfg(feature = "HU")]
    if countries.map_or(true, |c| c.contains(&Country::HU)) {
        map.insert(Country::HU, data::hu::build(years)?);
    }

    #[cfg(feature = "IS")]
    if countries.map_or(true, |c| c.contains(&Country::IS)) {
        map.insert(Country::IS, data::is::build(years)?);
    }

    #[cfg(feature = "IN")]
    if countries.map_or(true, |c| c.contains(&Country::IN)) {
        map.insert(Country::IN, data::r#in::build(years)?);
    }

    #[cfg(feature = "ID")]
    if countries.map_or(true, |c| c.contains(&Country::ID)) {
        map.insert(Country::ID, data::id::build(years)?);
    }

    #[cfg(feature = "IE")]
    if countries.map_or(true, |c| c.contains(&Country::IE)) {
        map.insert(Country::IE, data::ie::build(years)?);
    }

    #[cfg(feature = "IM")]
    if countries.map_or(true, |c| c.contains(&Country::IM)) {
        map.insert(Country::IM, data::im::build(years)?);
    }

    #[cfg(feature = "IL")]
    if countries.map_or(true, |c| c.contains(&Country::IL)) {
        map.insert(Country::IL, data::il::build(years)?);
    }

    #[cfg(feature = "IT")]
    if countries.map_or(true, |c| c.contains(&Country::IT)) {
        map.insert(Country::IT, data::it::build(years)?);
    }

    #[cfg(feature = "JM")]
    if countries.map_or(true, |c| c.contains(&Country::JM)) {
        map.insert(Country::JM, data::jm::build(years)?);
    }

    #[cfg(feature = "JP")]
    if countries.map_or(true, |c| c.contains(&Country::JP)) {
        map.insert(Country::JP, data::jp::build(years)?);
    }

    #[cfg(feature = "KZ")]
    if countries.map_or(true, |c| c.contains(&Country::KZ)) {
        map.insert(Country::KZ, data::kz::build(years)?);
    }

    #[cfg(feature = "KE")]
    if countries.map_or(true, |c| c.contains(&Country::KE)) {
        map.insert(Country::KE, data::ke::build(years)?);
    }

    #[cfg(feature = "LV")]
    if countries.map_or(true, |c| c.contains(&Country::LV)) {
        map.insert(Country::LV, data::lv::build(years)?);
    }

    #[cfg(feature = "LS")]
    if countries.map_or(true, |c| c.contains(&Country::LS)) {
        map.insert(Country::LS, data::ls::build(years)?);
    }

    #[cfg(feature = "LI")]
    if countries.map_or(true, |c| c.contains(&Country::LI)) {
        map.insert(Country::LI, data::li::build(years)?);
    }

    #[cfg(feature = "LT")]
    if countries.map_or(true, |c| c.contains(&Country::LT)) {
        map.insert(Country::LT, data::lt::build(years)?);
    }

    #[cfg(feature = "LU")]
    if countries.map_or(true, |c| c.contains(&Country::LU)) {
        map.insert(Country::LU, data::lu::build(years)?);
    }

    #[cfg(feature = "MG")]
    if countries.map_or(true, |c| c.contains(&Country::MG)) {
        map.insert(Country::MG, data::mg::build(years)?);
    }

    #[cfg(feature = "MY")]
    if countries.map_or(true, |c| c.contains(&Country::MY)) {
        map.insert(Country::MY, data::my::build(years)?);
    }

    #[cfg(feature = "MW")]
    if countries.map_or(true, |c| c.contains(&Country::MW)) {
        map.insert(Country::MW, data::mw::build(years)?);
    }

    #[cfg(feature = "MT")]
    if countries.map_or(true, |c| c.contains(&Country::MT)) {
        map.insert(Country::MT, data::mt::build(years)?);
    }

    #[cfg(feature = "MX")]
    if countries.map_or(true, |c| c.contains(&Country::MX)) {
        map.insert(Country::MX, data::mx::build(years)?);
    }

    #[cfg(feature = "MD")]
    if countries.map_or(true, |c| c.contains(&Country::MD)) {
        map.insert(Country::MD, data::md::build(years)?);
    }

    #[cfg(feature = "MA")]
    if countries.map_or(true, |c| c.contains(&Country::MA)) {
        map.insert(Country::MA, data::ma::build(years)?);
    }

    #[cfg(feature = "MZ")]
    if countries.map_or(true, |c| c.contains(&Country::MZ)) {
        map.insert(Country::MZ, data::mz::build(years)?);
    }

    #[cfg(feature = "NL")]
    if countries.map_or(true, |c| c.contains(&Country::NL)) {
        map.insert(Country::NL, data::nl::build(years)?);
    }

    #[cfg(feature = "NA")]
    if countries.map_or(true, |c| c.contains(&Country::NA)) {
        map.insert(Country::NA, data::na::build(years)?);
    }

    #[cfg(feature = "NZ")]
    if countries.map_or(true, |c| c.contains(&Country::NZ)) {
        map.insert(Country::NZ, data::nz::build(years)?);
    }

    #[cfg(feature = "NI")]
    if countries.map_or(true, |c| c.contains(&Country::NI)) {
        map.insert(Country::NI, data::ni::build(years)?);
    }

    #[cfg(feature = "NG")]
    if countries.map_or(true, |c| c.contains(&Country::NG)) {
        map.insert(Country::NG, data::ng::build(years)?);
    }

    #[cfg(feature = "MK")]
    if countries.map_or(true, |c| c.contains(&Country::MK)) {
        map.insert(Country::MK, data::mk::build(years)?);
    }

    #[cfg(feature = "NO")]
    if countries.map_or(true, |c| c.contains(&Country::NO)) {
        map.insert(Country::NO, data::no::build(years)?);
    }

    #[cfg(feature = "PK")]
    if countries.map_or(true, |c| c.contains(&Country::PK)) {
        map.insert(Country::PK, data::pk::build(years)?);
    }

    #[cfg(feature = "PY")]
    if countries.map_or(true, |c| c.contains(&Country::PY)) {
        map.insert(Country::PY, data::py::build(years)?);
    }

    #[cfg(feature = "PE")]
    if countries.map_or(true, |c| c.contains(&Country::PE)) {
        map.insert(Country::PE, data::pe::build(years)?);
    }

    #[cfg(feature = "PL")]
    if countries.map_or(true, |c| c.contains(&Country::PL)) {
        map.insert(Country::PL, data::pl::build(years)?);
    }

    #[cfg(feature = "PT")]
    if countries.map_or(true, |c| c.contains(&Country::PT)) {
        map.insert(Country::PT, data::pt::build(years)?);
    }

    #[cfg(feature = "RO")]
    if countries.map_or(true, |c| c.contains(&Country::RO)) {
        map.insert(Country::RO, data::ro::build(years)?);
    }

    #[cfg(feature = "RU")]
    if countries.map_or(true, |c| c.contains(&Country::RU)) {
        map.insert(Country::RU, data::ru::build(years)?);
    }

    #[cfg(feature = "SA")]
    if countries.map_or(true, |c| c.contains(&Country::SA)) {
        map.insert(Country::SA, data::sa::build(years)?);
    }

    #[cfg(feature = "RS")]
    if countries.map_or(true, |c| c.contains(&Country::RS)) {
        map.insert(Country::RS, data::rs::build(years)?);
    }

    #[cfg(feature = "SG")]
    if countries.map_or(true, |c| c.contains(&Country::SG)) {
        map.insert(Country::SG, data::sg::build(years)?);
    }

    #[cfg(feature = "SK")]
    if countries.map_or(true, |c| c.contains(&Country::SK)) {
        map.insert(Country::SK, data::sk::build(years)?);
    }

    #[cfg(feature = "SI")]
    if countries.map_or(true, |c| c.contains(&Country::SI)) {
        map.insert(Country::SI, data::si::build(years)?);
    }

    #[cfg(feature = "ZA")]
    if countries.map_or(true, |c| c.contains(&Country::ZA)) {
        map.insert(Country::ZA, data::za::build(years)?);
    }

    #[cfg(feature = "KR")]
    if countries.map_or(true, |c| c.contains(&Country::KR)) {
        map.insert(Country::KR, data::kr::build(years)?);
    }

    #[cfg(feature = "ES")]
    if countries.map_or(true, |c| c.contains(&Country::ES)) {
        map.insert(Country::ES, data::es::build(years)?);
    }

    #[cfg(feature = "SZ")]
    if countries.map_or(true, |c| c.contains(&Country::SZ)) {
        map.insert(Country::SZ, data::sz::build(years)?);
    }

    #[cfg(feature = "SE")]
    if countries.map_or(true, |c| c.contains(&Country::SE)) {
        map.insert(Country::SE, data::se::build(years)?);
    }

    #[cfg(feature = "CH")]
    if countries.map_or(true, |c| c.contains(&Country::CH)) {
        map.insert(Country::CH, data::ch::build(years)?);
    }

    #[cfg(feature = "TW")]
    if countries.map_or(true, |c| c.contains(&Country::TW)) {
        map.insert(Country::TW, data::tw::build(years)?);
    }

    #[cfg(feature = "TR")]
    if countries.map_or(true, |c| c.contains(&Country::TR)) {
        map.insert(Country::TR, data::tr::build(years)?);
    }

    #[cfg(feature = "TN")]
    if countries.map_or(true, |c| c.contains(&Country::TN)) {
        map.insert(Country::TN, data::tn::build(years)?);
    }

    #[cfg(feature = "UA")]
    if countries.map_or(true, |c| c.contains(&Country::UA)) {
        map.insert(Country::UA, data::ua::build(years)?);
    }

    #[cfg(feature = "AE")]
    if countries.map_or(true, |c| c.contains(&Country::AE)) {
        map.insert(Country::AE, data::ae::build(years)?);
    }

    #[cfg(feature = "GB")]
    if countries.map_or(true, |c| c.contains(&Country::GB)) {
        map.insert(Country::GB, data::gb::build(years)?);
    }

    #[cfg(feature = "US")]
    if countries.map_or(true, |c| c.contains(&Country::US)) {
        map.insert(Country::US, data::us::build(years)?);
    }

    #[cfg(feature = "UY")]
    if countries.map_or(true, |c| c.contains(&Country::UY)) {
        map.insert(Country::UY, data::uy::build(years)?);
    }

    #[cfg(feature = "UZ")]
    if countries.map_or(true, |c| c.contains(&Country::UZ)) {
        map.insert(Country::UZ, data::uz::build(years)?);
    }

    #[cfg(feature = "VE")]
    if countries.map_or(true, |c| c.contains(&Country::VE)) {
        map.insert(Country::VE, data::ve::build(years)?);
    }

    #[cfg(feature = "VN")]
    if countries.map_or(true, |c| c.contains(&Country::VN)) {
        map.insert(Country::VN, data::vn::build(years)?);
    }

    #[cfg(feature = "ZM")]
    if countries.map_or(true, |c| c.contains(&Country::ZM)) {
        map.insert(Country::ZM, data::zm::build(years)?);
    }

    #[cfg(feature = "ZW")]
    if countries.map_or(true, |c| c.contains(&Country::ZW)) {
        map.insert(Country::ZW, data::zw::build(years)?);
    }

    Ok(map)
}
