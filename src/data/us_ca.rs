//! United States (California)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (California)";
const COUNTY_CODE: Country = Country::US_CA;

/// Generate holiday map for United States (California).
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
            (NaiveDate::from_ymd_res(2000, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2000, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2000, 11, 24)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2001, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2001, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2001, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2002, 4, 1)?,
                "Cesar Chavez Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 29)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2003, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2003, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2003, 11, 28)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2004, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2004, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2004, 11, 26)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2005, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2005, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2005, 11, 25)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2006, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2006, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2006, 11, 24)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2007, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2007, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2007, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2008, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2008, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2008, 11, 28)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2009, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2009, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2010, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2010, 11, 26)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2011, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2011, 11, 25)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2012, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2012, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2013, 4, 1)?,
                "Cesar Chavez Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 29)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2014, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2014, 11, 28)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2015, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2015, 11, 27)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2016, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2016, 11, 25)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2017, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2017, 11, 24)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2018, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2018, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2019, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2019, 4, 1)?,
                "Cesar Chavez Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 29)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2020, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2020, 11, 27)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2021, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2021, 11, 26)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2022, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2022, 11, 25)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2023, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2023, 11, 24)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2024, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2024, 4, 1)?,
                "Cesar Chavez Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 29)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2025, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2025, 11, 28)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2026, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2026, 11, 27)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2027, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2027, 11, 26)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2028, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2028, 11, 24)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2029, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (
                NaiveDate::from_ymd_res(2030, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2030, 4, 1)?,
                "Cesar Chavez Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 29)?,
                "Day After Thanksgiving",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
