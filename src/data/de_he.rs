//! Germany (Hesse (Hessen))
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Germany (Hesse (Hessen))";
const COUNTY_CODE: Country = Country::DE_HE;

/// Generate holiday map for Germany (Hesse (Hessen)).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_year(
        years,
        2000,
        [(NaiveDate::from_ymd_res(2000, 6, 22)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [(NaiveDate::from_ymd_res(2001, 6, 14)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [(NaiveDate::from_ymd_res(2002, 5, 30)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [(NaiveDate::from_ymd_res(2003, 6, 19)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [(NaiveDate::from_ymd_res(2004, 6, 10)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [(NaiveDate::from_ymd_res(2005, 5, 26)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [(NaiveDate::from_ymd_res(2006, 6, 15)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [(NaiveDate::from_ymd_res(2007, 6, 7)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [(NaiveDate::from_ymd_res(2008, 5, 22)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [(NaiveDate::from_ymd_res(2009, 6, 11)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [(NaiveDate::from_ymd_res(2010, 6, 3)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [(NaiveDate::from_ymd_res(2011, 6, 23)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [(NaiveDate::from_ymd_res(2012, 6, 7)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [(NaiveDate::from_ymd_res(2013, 5, 30)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [(NaiveDate::from_ymd_res(2014, 6, 19)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [(NaiveDate::from_ymd_res(2015, 6, 4)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [(NaiveDate::from_ymd_res(2016, 5, 26)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [(NaiveDate::from_ymd_res(2017, 6, 15)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [(NaiveDate::from_ymd_res(2018, 5, 31)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [(NaiveDate::from_ymd_res(2019, 6, 20)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [(NaiveDate::from_ymd_res(2020, 6, 11)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [(NaiveDate::from_ymd_res(2021, 6, 3)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [(NaiveDate::from_ymd_res(2022, 6, 16)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [(NaiveDate::from_ymd_res(2023, 6, 8)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [(NaiveDate::from_ymd_res(2024, 5, 30)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [(NaiveDate::from_ymd_res(2025, 6, 19)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [(NaiveDate::from_ymd_res(2026, 6, 4)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [(NaiveDate::from_ymd_res(2027, 5, 27)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [(NaiveDate::from_ymd_res(2028, 6, 15)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [(NaiveDate::from_ymd_res(2029, 5, 31)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [(NaiveDate::from_ymd_res(2030, 6, 20)?, "Fronleichnam")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
