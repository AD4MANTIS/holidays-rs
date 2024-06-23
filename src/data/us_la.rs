//! United States (Louisiana)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Louisiana)";
const COUNTY_CODE: Country = Country::US_LA;

/// Generate holiday map for United States (Louisiana).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerCountryMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_subdivision_year(
        years,
        2000,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2000, 3, 7)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2001,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2001, 1, 20)?, "Inauguration Day"),
            (NaiveDate::from_ymd_res(2001, 2, 27)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2002,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2003,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2003, 3, 4)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2004,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2004, 2, 24)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2005,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "Inauguration Day"),
            (NaiveDate::from_ymd_res(2005, 2, 8)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2006,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2006, 2, 28)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2007,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2007, 2, 20)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2008,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2008, 2, 5)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 11, 4)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2009,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2009, 1, 20)?, "Inauguration Day"),
            (NaiveDate::from_ymd_res(2009, 2, 24)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2010,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2010, 2, 16)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 11, 2)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2011,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2012,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2012, 2, 21)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 11, 6)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2013,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2013, 1, 20)?, "Inauguration Day"),
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2014,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2014, 3, 4)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 11, 4)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2015,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2015, 2, 17)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2016,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 11, 8)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2017,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2017, 1, 20)?, "Inauguration Day"),
            (NaiveDate::from_ymd_res(2017, 2, 28)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2018,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2018, 2, 13)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 11, 6)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2019,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2019, 3, 5)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2020,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2020, 2, 25)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 11, 3)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2021,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2021, 1, 20)?, "Inauguration Day"),
            (NaiveDate::from_ymd_res(2021, 2, 16)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2022,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2022, 3, 1)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 11, 8)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2023,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2023, 2, 21)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2024,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2024, 2, 13)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 11, 5)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2025,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2025, 3, 4)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2026,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 11, 3)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2027,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2028,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2028, 2, 29)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 11, 7)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2029,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2029, 1, 20)?, "Inauguration Day"),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2030,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2030, 3, 5)?, "Mardi Gras"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 11, 5)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
