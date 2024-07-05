//! United States (Georgia)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Georgia)";
const COUNTY_CODE: Country = Country::US_GA;

/// Generate holiday map for United States (Georgia).
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
        [
            (
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 24)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (
                NaiveDate::from_ymd_res(2001, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 23)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (
                NaiveDate::from_ymd_res(2002, 4, 22)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 29)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (
                NaiveDate::from_ymd_res(2003, 4, 28)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 28)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (
                NaiveDate::from_ymd_res(2004, 4, 26)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 26)?,
                "Robert E. Lee's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (
                NaiveDate::from_ymd_res(2005, 4, 25)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 25)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (
                NaiveDate::from_ymd_res(2006, 4, 24)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 24)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (
                NaiveDate::from_ymd_res(2007, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 23)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (
                NaiveDate::from_ymd_res(2008, 4, 28)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 28)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (
                NaiveDate::from_ymd_res(2009, 4, 27)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (
                NaiveDate::from_ymd_res(2010, 4, 26)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 26)?,
                "Robert E. Lee's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 25)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (
                NaiveDate::from_ymd_res(2012, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 23)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (
                NaiveDate::from_ymd_res(2013, 4, 22)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 29)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (
                NaiveDate::from_ymd_res(2014, 4, 28)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 28)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (
                NaiveDate::from_ymd_res(2015, 4, 27)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 27)?,
                "Robert E. Lee's Birthday",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 4, 25)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2016, 11, 25)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2016, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 4, 24)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2017, 11, 24)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2017, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 4, 23)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2018, 11, 23)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2018, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2019, 11, 29)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2019, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2020, 11, 27)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2020, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 4, 26)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2021, 11, 26)?, "State Holiday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 4, 25)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2022, 11, 25)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2022, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 4, 24)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2023, 11, 24)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2023, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 4, 22)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2024, 11, 29)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2024, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 4, 28)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2025, 11, 28)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 4, 27)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2026, 11, 27)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2026, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 4, 26)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2027, 11, 26)?, "State Holiday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 4, 24)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2028, 11, 24)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2028, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 4, 23)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2029, 11, 23)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2029, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "State Holiday"),
            (NaiveDate::from_ymd_res(2030, 11, 29)?, "State Holiday"),
            (
                NaiveDate::from_ymd_res(2030, 12, 24)?,
                "Washington's Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
