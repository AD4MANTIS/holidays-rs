//! United States (Maryland)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Maryland)";
const COUNTY_CODE: Country = Country::US_MD;

/// Generate holiday map for United States (Maryland).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_year(years, 2000, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2001,
        [(NaiveDate::from_ymd_res(2001, 1, 20)?, "Inauguration Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2002, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2003, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2004, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2005,
        [(NaiveDate::from_ymd_res(2005, 1, 20)?, "Inauguration Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2006, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2007, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2008,
        [(
            NaiveDate::from_ymd_res(2008, 11, 28)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 20)?, "Inauguration Day"),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "American Indian Heritage Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [(
            NaiveDate::from_ymd_res(2010, 11, 26)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [(
            NaiveDate::from_ymd_res(2011, 11, 25)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [(
            NaiveDate::from_ymd_res(2012, 11, 23)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 20)?, "Inauguration Day"),
            (
                NaiveDate::from_ymd_res(2013, 11, 29)?,
                "American Indian Heritage Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [(
            NaiveDate::from_ymd_res(2014, 11, 28)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [(
            NaiveDate::from_ymd_res(2015, 11, 27)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [(
            NaiveDate::from_ymd_res(2016, 11, 25)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 20)?, "Inauguration Day"),
            (
                NaiveDate::from_ymd_res(2017, 11, 24)?,
                "American Indian Heritage Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [(
            NaiveDate::from_ymd_res(2018, 11, 23)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [(
            NaiveDate::from_ymd_res(2019, 11, 29)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [(
            NaiveDate::from_ymd_res(2020, 11, 27)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 20)?, "Inauguration Day"),
            (
                NaiveDate::from_ymd_res(2021, 11, 26)?,
                "American Indian Heritage Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [(
            NaiveDate::from_ymd_res(2022, 11, 25)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [(
            NaiveDate::from_ymd_res(2023, 11, 24)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [(
            NaiveDate::from_ymd_res(2024, 11, 29)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [(
            NaiveDate::from_ymd_res(2025, 11, 28)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [(
            NaiveDate::from_ymd_res(2026, 11, 27)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [(
            NaiveDate::from_ymd_res(2027, 11, 26)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [(
            NaiveDate::from_ymd_res(2028, 11, 24)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 20)?, "Inauguration Day"),
            (
                NaiveDate::from_ymd_res(2029, 11, 23)?,
                "American Indian Heritage Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [(
            NaiveDate::from_ymd_res(2030, 11, 29)?,
            "American Indian Heritage Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
