//! South Korea
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "South Korea";
const COUNTY_CODE: Country = Country::KR;

/// Generate holiday map for South Korea.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2000, 2, 5)?, "설날"),
            (NaiveDate::from_ymd_res(2000, 2, 4)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2000, 2, 6)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2000, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2000, 4, 5)?, "식목일"),
            (NaiveDate::from_ymd_res(2000, 5, 11)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2000, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2000, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2000, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2000, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2000, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2000, 9, 12)?, "추석"),
            (NaiveDate::from_ymd_res(2000, 9, 11)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2000, 9, 13)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2000, 4, 13)?, "국회의원 선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2001, 1, 24)?, "설날"),
            (NaiveDate::from_ymd_res(2001, 1, 23)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2001, 1, 25)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2001, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2001, 4, 5)?, "식목일"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2001, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2001, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2001, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2001, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2001, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2001, 10, 1)?, "추석"),
            (NaiveDate::from_ymd_res(2001, 9, 30)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2001, 10, 2)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "기독탄신일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "설날"),
            (NaiveDate::from_ymd_res(2002, 2, 11)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2002, 2, 13)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2002, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2002, 4, 5)?, "식목일"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2002, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2002, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2002, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2002, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2002, 9, 21)?, "추석"),
            (NaiveDate::from_ymd_res(2002, 9, 20)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2002, 9, 22)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2002, 6, 13)?, "지방선거일"),
            (
                NaiveDate::from_ymd_res(2002, 7, 1)?,
                "2002년 한일 월드컵 대표팀 4강 진출",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 19)?, "대통령 선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2003, 2, 1)?, "설날"),
            (NaiveDate::from_ymd_res(2003, 1, 31)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2003, 2, 2)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2003, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2003, 4, 5)?, "식목일"),
            (NaiveDate::from_ymd_res(2003, 5, 8)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2003, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2003, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2003, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2003, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2003, 9, 11)?, "추석"),
            (NaiveDate::from_ymd_res(2003, 9, 10)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2003, 9, 12)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "기독탄신일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2004, 1, 22)?, "설날"),
            (NaiveDate::from_ymd_res(2004, 1, 21)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2004, 1, 23)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2004, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2004, 4, 5)?, "식목일"),
            (NaiveDate::from_ymd_res(2004, 5, 26)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2004, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2004, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2004, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2004, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2004, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2004, 9, 28)?, "추석"),
            (NaiveDate::from_ymd_res(2004, 9, 27)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2004, 9, 29)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2004, 4, 15)?, "국회의원 선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "설날"),
            (NaiveDate::from_ymd_res(2005, 2, 8)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2005, 2, 10)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2005, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2005, 4, 5)?, "식목일"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2005, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2005, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2005, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2005, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2005, 9, 18)?, "추석"),
            (NaiveDate::from_ymd_res(2005, 9, 17)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2005, 9, 19)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "기독탄신일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2006, 1, 29)?, "설날"),
            (NaiveDate::from_ymd_res(2006, 1, 28)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2006, 1, 30)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2006, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2006, 5, 5)?, "석가탄신일; 어린이날"),
            (NaiveDate::from_ymd_res(2006, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2006, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2006, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2006, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2006, 10, 6)?, "추석"),
            (NaiveDate::from_ymd_res(2006, 10, 5)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2006, 10, 7)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2006, 5, 31)?, "지방선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2007, 2, 18)?, "설날"),
            (NaiveDate::from_ymd_res(2007, 2, 17)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2007, 2, 19)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2007, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2007, 5, 24)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2007, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2007, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2007, 7, 17)?, "제헌절"),
            (NaiveDate::from_ymd_res(2007, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2007, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2007, 9, 25)?, "추석"),
            (NaiveDate::from_ymd_res(2007, 9, 24)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2007, 9, 26)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2007, 12, 19)?, "대통령 선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2008, 2, 7)?, "설날"),
            (NaiveDate::from_ymd_res(2008, 2, 6)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2008, 2, 8)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2008, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2008, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2008, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2008, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2008, 9, 14)?, "추석"),
            (NaiveDate::from_ymd_res(2008, 9, 13)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2008, 9, 15)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2008, 4, 9)?, "국회의원 선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "설날"),
            (NaiveDate::from_ymd_res(2009, 1, 25)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2009, 1, 27)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2009, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2009, 5, 2)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2009, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2009, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2009, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2009, 10, 3)?, "개천절; 추석"),
            (NaiveDate::from_ymd_res(2009, 10, 2)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2009, 10, 4)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "기독탄신일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2010, 2, 14)?, "설날"),
            (NaiveDate::from_ymd_res(2010, 2, 13)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2010, 2, 15)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2010, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2010, 5, 21)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2010, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2010, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2010, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2010, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2010, 9, 22)?, "추석"),
            (NaiveDate::from_ymd_res(2010, 9, 21)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2010, 9, 23)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2010, 6, 2)?, "지방선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2011, 2, 3)?, "설날"),
            (NaiveDate::from_ymd_res(2011, 2, 2)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2011, 2, 4)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2011, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2011, 5, 10)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2011, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2011, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2011, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2011, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2011, 9, 12)?, "추석"),
            (NaiveDate::from_ymd_res(2011, 9, 11)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2011, 9, 13)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "기독탄신일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2012, 1, 23)?, "설날"),
            (NaiveDate::from_ymd_res(2012, 1, 22)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2012, 1, 24)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2012, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2012, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2012, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2012, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2012, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2012, 9, 30)?, "추석"),
            (NaiveDate::from_ymd_res(2012, 9, 29)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2012, 10, 1)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2012, 4, 11)?, "국회의원 선거일"),
            (NaiveDate::from_ymd_res(2012, 12, 19)?, "대통령 선거일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2013, 2, 10)?, "설날"),
            (NaiveDate::from_ymd_res(2013, 2, 9)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2013, 2, 11)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2013, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2013, 5, 17)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2013, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2013, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2013, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2013, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2013, 9, 19)?, "추석"),
            (NaiveDate::from_ymd_res(2013, 9, 18)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2013, 9, 20)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "기독탄신일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2014, 1, 31)?, "설날"),
            (NaiveDate::from_ymd_res(2014, 1, 30)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2014, 2, 1)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2014, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2014, 5, 6)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2014, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2014, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2014, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2014, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2014, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2014, 9, 8)?, "추석"),
            (NaiveDate::from_ymd_res(2014, 9, 7)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2014, 9, 9)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2014, 6, 4)?, "지방선거일"),
            (NaiveDate::from_ymd_res(2014, 9, 10)?, "추석 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2015, 2, 19)?, "설날"),
            (NaiveDate::from_ymd_res(2015, 2, 18)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2015, 2, 20)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2015, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2015, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2015, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2015, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2015, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2015, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2015, 9, 27)?, "추석"),
            (NaiveDate::from_ymd_res(2015, 9, 26)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2015, 9, 28)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2015, 9, 29)?, "추석 대체 휴일"),
            (NaiveDate::from_ymd_res(2015, 8, 14)?, "임시공휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "설날"),
            (NaiveDate::from_ymd_res(2016, 2, 7)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2016, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2016, 5, 14)?, "석가탄신일"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2016, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2016, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2016, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2016, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2016, 9, 15)?, "추석"),
            (NaiveDate::from_ymd_res(2016, 9, 14)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2016, 9, 16)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2016, 4, 13)?, "국회의원 선거일"),
            (NaiveDate::from_ymd_res(2016, 2, 10)?, "설날 대체 휴일"),
            (NaiveDate::from_ymd_res(2016, 5, 6)?, "임시공휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2017, 1, 28)?, "설날"),
            (NaiveDate::from_ymd_res(2017, 1, 27)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2017, 1, 29)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2017, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2017, 5, 3)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2017, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2017, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2017, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2017, 10, 3)?, "개천절; 추석 전날"),
            (NaiveDate::from_ymd_res(2017, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2017, 10, 4)?, "추석"),
            (NaiveDate::from_ymd_res(2017, 10, 5)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2017, 1, 30)?, "설날 대체 휴일"),
            (NaiveDate::from_ymd_res(2017, 10, 6)?, "추석 대체 휴일"),
            (NaiveDate::from_ymd_res(2017, 5, 9)?, "대통령 선거일"),
            (NaiveDate::from_ymd_res(2017, 10, 2)?, "임시공휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2018, 2, 16)?, "설날"),
            (NaiveDate::from_ymd_res(2018, 2, 15)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2018, 2, 17)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2018, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2018, 5, 22)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2018, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2018, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2018, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2018, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2018, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2018, 9, 24)?, "추석"),
            (NaiveDate::from_ymd_res(2018, 9, 23)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2018, 9, 25)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2018, 6, 13)?, "지방선거일"),
            (NaiveDate::from_ymd_res(2018, 5, 7)?, "어린이날 대체 휴일"),
            (NaiveDate::from_ymd_res(2018, 9, 26)?, "추석 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "설날"),
            (NaiveDate::from_ymd_res(2019, 2, 4)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2019, 2, 6)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2019, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2019, 5, 12)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2019, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2019, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2019, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2019, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2019, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2019, 9, 13)?, "추석"),
            (NaiveDate::from_ymd_res(2019, 9, 12)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2019, 9, 14)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2019, 5, 6)?, "어린이날 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2020, 1, 25)?, "설날"),
            (NaiveDate::from_ymd_res(2020, 1, 24)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2020, 1, 26)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2020, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2020, 4, 30)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2020, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2020, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2020, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2020, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2020, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2020, 10, 1)?, "추석"),
            (NaiveDate::from_ymd_res(2020, 9, 30)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2020, 10, 2)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2020, 4, 15)?, "국회의원 선거일"),
            (NaiveDate::from_ymd_res(2020, 1, 27)?, "설날 대체 휴일"),
            (NaiveDate::from_ymd_res(2020, 8, 17)?, "임시공휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "설날"),
            (NaiveDate::from_ymd_res(2021, 2, 11)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2021, 2, 13)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2021, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2021, 5, 19)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2021, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2021, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2021, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2021, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2021, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2021, 9, 21)?, "추석"),
            (NaiveDate::from_ymd_res(2021, 9, 20)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2021, 9, 22)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2021, 8, 16)?, "광복절 대체 휴일"),
            (NaiveDate::from_ymd_res(2021, 10, 4)?, "개천절 대체 휴일"),
            (NaiveDate::from_ymd_res(2021, 10, 11)?, "한글날 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2022, 2, 1)?, "설날"),
            (NaiveDate::from_ymd_res(2022, 1, 31)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2022, 2, 2)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2022, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2022, 5, 8)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2022, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2022, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2022, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2022, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2022, 9, 10)?, "추석"),
            (NaiveDate::from_ymd_res(2022, 9, 9)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2022, 9, 11)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2022, 3, 9)?, "대통령 선거일"),
            (NaiveDate::from_ymd_res(2022, 6, 1)?, "지방선거일"),
            (NaiveDate::from_ymd_res(2022, 9, 12)?, "추석 대체 휴일"),
            (NaiveDate::from_ymd_res(2022, 10, 10)?, "한글날 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2023, 1, 22)?, "설날"),
            (NaiveDate::from_ymd_res(2023, 1, 21)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2023, 1, 23)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2023, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2023, 5, 27)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2023, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2023, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2023, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2023, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2023, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2023, 9, 29)?, "추석"),
            (NaiveDate::from_ymd_res(2023, 9, 28)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2023, 9, 30)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2023, 1, 24)?, "설날 대체 휴일"),
            (
                NaiveDate::from_ymd_res(2023, 5, 29)?,
                "부처님오신날 대체 휴일",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 2)?, "임시공휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2024, 2, 10)?, "설날"),
            (NaiveDate::from_ymd_res(2024, 2, 9)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2024, 2, 11)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2024, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2024, 5, 15)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2024, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2024, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2024, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2024, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2024, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2024, 9, 17)?, "추석"),
            (NaiveDate::from_ymd_res(2024, 9, 16)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2024, 9, 18)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "국회의원 선거일"),
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "설날 대체 휴일"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "어린이날 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2025, 1, 29)?, "설날"),
            (NaiveDate::from_ymd_res(2025, 1, 28)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2025, 1, 30)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2025, 3, 1)?, "삼일절"),
            (
                NaiveDate::from_ymd_res(2025, 5, 5)?,
                "부처님오신날; 어린이날",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2025, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2025, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2025, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2025, 10, 6)?, "추석"),
            (NaiveDate::from_ymd_res(2025, 10, 5)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2025, 10, 7)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2025, 3, 3)?, "삼일절 대체 휴일"),
            (
                NaiveDate::from_ymd_res(2025, 5, 6)?,
                "부처님오신날 대체 휴일; 어린이날 대체 휴일",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 8)?, "추석 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "설날"),
            (NaiveDate::from_ymd_res(2026, 2, 16)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2026, 2, 18)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2026, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2026, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2026, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2026, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2026, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2026, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2026, 9, 25)?, "추석"),
            (NaiveDate::from_ymd_res(2026, 9, 24)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2026, 9, 26)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2026, 6, 3)?, "지방선거일"),
            (NaiveDate::from_ymd_res(2026, 3, 2)?, "삼일절 대체 휴일"),
            (
                NaiveDate::from_ymd_res(2026, 5, 25)?,
                "부처님오신날 대체 휴일",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 17)?, "광복절 대체 휴일"),
            (NaiveDate::from_ymd_res(2026, 10, 5)?, "개천절 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2027, 2, 7)?, "설날"),
            (NaiveDate::from_ymd_res(2027, 2, 6)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2027, 2, 8)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2027, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2027, 5, 13)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2027, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2027, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2027, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2027, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2027, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2027, 9, 15)?, "추석"),
            (NaiveDate::from_ymd_res(2027, 9, 14)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2027, 9, 16)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2027, 3, 3)?, "대통령 선거일"),
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "설날 대체 휴일"),
            (NaiveDate::from_ymd_res(2027, 8, 16)?, "광복절 대체 휴일"),
            (NaiveDate::from_ymd_res(2027, 10, 4)?, "개천절 대체 휴일"),
            (NaiveDate::from_ymd_res(2027, 10, 11)?, "한글날 대체 휴일"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "기독탄신일 대체 휴일",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2028, 1, 27)?, "설날"),
            (NaiveDate::from_ymd_res(2028, 1, 26)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2028, 1, 28)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2028, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2028, 5, 2)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2028, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2028, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2028, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2028, 10, 3)?, "개천절; 추석"),
            (NaiveDate::from_ymd_res(2028, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2028, 10, 2)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2028, 10, 4)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2028, 4, 12)?, "국회의원 선거일"),
            (NaiveDate::from_ymd_res(2028, 10, 5)?, "추석 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "설날"),
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2029, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2029, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2029, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2029, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2029, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2029, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2029, 9, 22)?, "추석"),
            (NaiveDate::from_ymd_res(2029, 9, 21)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2029, 9, 23)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2029, 5, 7)?, "어린이날 대체 휴일"),
            (
                NaiveDate::from_ymd_res(2029, 5, 21)?,
                "부처님오신날 대체 휴일",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 24)?, "추석 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "신정연휴"),
            (NaiveDate::from_ymd_res(2030, 2, 3)?, "설날"),
            (NaiveDate::from_ymd_res(2030, 2, 2)?, "설날 전날"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "설날 다음날"),
            (NaiveDate::from_ymd_res(2030, 3, 1)?, "삼일절"),
            (NaiveDate::from_ymd_res(2030, 5, 9)?, "부처님오신날"),
            (NaiveDate::from_ymd_res(2030, 5, 5)?, "어린이날"),
            (NaiveDate::from_ymd_res(2030, 6, 6)?, "현충일"),
            (NaiveDate::from_ymd_res(2030, 8, 15)?, "광복절"),
            (NaiveDate::from_ymd_res(2030, 10, 3)?, "개천절"),
            (NaiveDate::from_ymd_res(2030, 10, 9)?, "한글날"),
            (NaiveDate::from_ymd_res(2030, 9, 12)?, "추석"),
            (NaiveDate::from_ymd_res(2030, 9, 11)?, "추석 전날"),
            (NaiveDate::from_ymd_res(2030, 9, 13)?, "추석 다음날"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "기독탄신일"),
            (NaiveDate::from_ymd_res(2030, 6, 12)?, "지방선거일"),
            (NaiveDate::from_ymd_res(2030, 2, 5)?, "설날 대체 휴일"),
            (NaiveDate::from_ymd_res(2030, 5, 6)?, "어린이날 대체 휴일"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
