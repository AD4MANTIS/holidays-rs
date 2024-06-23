//! Japan
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Japan";
const COUNTY_CODE: Country = Country::JP;

/// Generate holiday map for Japan.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerCountryMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2000, 1, 10)?, "成人の日"),
            (NaiveDate::from_ymd_res(2000, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2000, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2000, 4, 29)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2000, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2000, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2000, 7, 20)?, "海の日"),
            (NaiveDate::from_ymd_res(2000, 9, 15)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2000, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2000, 10, 9)?, "体育の日"),
            (NaiveDate::from_ymd_res(2000, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2000, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2000, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2000, 5, 4)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2001, 1, 8)?, "成人の日"),
            (NaiveDate::from_ymd_res(2001, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2001, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2001, 4, 29)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2001, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2001, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2001, 7, 20)?, "海の日"),
            (NaiveDate::from_ymd_res(2001, 9, 15)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2001, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2001, 10, 8)?, "体育の日"),
            (NaiveDate::from_ymd_res(2001, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2001, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2001, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2001, 2, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2001, 4, 30)?, "振替休日"),
            (NaiveDate::from_ymd_res(2001, 9, 24)?, "振替休日"),
            (NaiveDate::from_ymd_res(2001, 12, 24)?, "振替休日"),
            (NaiveDate::from_ymd_res(2001, 5, 4)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2002, 1, 14)?, "成人の日"),
            (NaiveDate::from_ymd_res(2002, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2002, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2002, 4, 29)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2002, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2002, 7, 20)?, "海の日"),
            (NaiveDate::from_ymd_res(2002, 9, 15)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2002, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2002, 10, 14)?, "体育の日"),
            (NaiveDate::from_ymd_res(2002, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2002, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2002, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2002, 9, 16)?, "振替休日"),
            (NaiveDate::from_ymd_res(2002, 11, 4)?, "振替休日"),
            (NaiveDate::from_ymd_res(2002, 5, 4)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2003, 1, 13)?, "成人の日"),
            (NaiveDate::from_ymd_res(2003, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2003, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2003, 4, 29)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2003, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2003, 7, 21)?, "海の日"),
            (NaiveDate::from_ymd_res(2003, 9, 15)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2003, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2003, 10, 13)?, "体育の日"),
            (NaiveDate::from_ymd_res(2003, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2003, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2003, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2003, 11, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2004, 1, 12)?, "成人の日"),
            (NaiveDate::from_ymd_res(2004, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2004, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2004, 4, 29)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2004, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2004, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2004, 7, 19)?, "海の日"),
            (NaiveDate::from_ymd_res(2004, 9, 20)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2004, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2004, 10, 11)?, "体育の日"),
            (NaiveDate::from_ymd_res(2004, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2004, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2004, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2004, 5, 4)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2005, 1, 10)?, "成人の日"),
            (NaiveDate::from_ymd_res(2005, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2005, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2005, 4, 29)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2005, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2005, 7, 18)?, "海の日"),
            (NaiveDate::from_ymd_res(2005, 9, 19)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2005, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2005, 10, 10)?, "体育の日"),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2005, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2005, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2005, 3, 21)?, "振替休日"),
            (NaiveDate::from_ymd_res(2005, 5, 4)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2006, 1, 9)?, "成人の日"),
            (NaiveDate::from_ymd_res(2006, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2006, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2006, 4, 29)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2006, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2006, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2006, 7, 17)?, "海の日"),
            (NaiveDate::from_ymd_res(2006, 9, 18)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2006, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2006, 10, 9)?, "体育の日"),
            (NaiveDate::from_ymd_res(2006, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2006, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2006, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "振替休日"),
            (NaiveDate::from_ymd_res(2006, 5, 4)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2007, 1, 8)?, "成人の日"),
            (NaiveDate::from_ymd_res(2007, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2007, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2007, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2007, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2007, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2007, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2007, 7, 16)?, "海の日"),
            (NaiveDate::from_ymd_res(2007, 9, 17)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2007, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2007, 10, 8)?, "体育の日"),
            (NaiveDate::from_ymd_res(2007, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2007, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2007, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2007, 2, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2007, 4, 30)?, "振替休日"),
            (NaiveDate::from_ymd_res(2007, 9, 24)?, "振替休日"),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2008, 1, 14)?, "成人の日"),
            (NaiveDate::from_ymd_res(2008, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2008, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2008, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2008, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2008, 7, 21)?, "海の日"),
            (NaiveDate::from_ymd_res(2008, 9, 15)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2008, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2008, 10, 13)?, "体育の日"),
            (NaiveDate::from_ymd_res(2008, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2008, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2008, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2008, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2008, 11, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2009, 1, 12)?, "成人の日"),
            (NaiveDate::from_ymd_res(2009, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2009, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2009, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2009, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2009, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2009, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2009, 7, 20)?, "海の日"),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2009, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2009, 10, 12)?, "体育の日"),
            (NaiveDate::from_ymd_res(2009, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2009, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2009, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2009, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2009, 9, 22)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2010, 1, 11)?, "成人の日"),
            (NaiveDate::from_ymd_res(2010, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2010, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2010, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2010, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2010, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2010, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2010, 7, 19)?, "海の日"),
            (NaiveDate::from_ymd_res(2010, 9, 20)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2010, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2010, 10, 11)?, "体育の日"),
            (NaiveDate::from_ymd_res(2010, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2010, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2010, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2010, 3, 22)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2011, 1, 10)?, "成人の日"),
            (NaiveDate::from_ymd_res(2011, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2011, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2011, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2011, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2011, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2011, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2011, 7, 18)?, "海の日"),
            (NaiveDate::from_ymd_res(2011, 9, 19)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2011, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2011, 10, 10)?, "体育の日"),
            (NaiveDate::from_ymd_res(2011, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2011, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2011, 12, 23)?, "天皇誕生日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2012, 1, 9)?, "成人の日"),
            (NaiveDate::from_ymd_res(2012, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2012, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2012, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2012, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2012, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2012, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2012, 7, 16)?, "海の日"),
            (NaiveDate::from_ymd_res(2012, 9, 17)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2012, 9, 22)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2012, 10, 8)?, "体育の日"),
            (NaiveDate::from_ymd_res(2012, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2012, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2012, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "振替休日"),
            (NaiveDate::from_ymd_res(2012, 4, 30)?, "振替休日"),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2013, 1, 14)?, "成人の日"),
            (NaiveDate::from_ymd_res(2013, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2013, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2013, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2013, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2013, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2013, 7, 15)?, "海の日"),
            (NaiveDate::from_ymd_res(2013, 9, 16)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2013, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2013, 10, 14)?, "体育の日"),
            (NaiveDate::from_ymd_res(2013, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2013, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2013, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2013, 11, 4)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2014, 1, 13)?, "成人の日"),
            (NaiveDate::from_ymd_res(2014, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2014, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2014, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2014, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2014, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2014, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2014, 7, 21)?, "海の日"),
            (NaiveDate::from_ymd_res(2014, 9, 15)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2014, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2014, 10, 13)?, "体育の日"),
            (NaiveDate::from_ymd_res(2014, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2014, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2014, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2014, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2014, 11, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2015, 1, 12)?, "成人の日"),
            (NaiveDate::from_ymd_res(2015, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2015, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2015, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2015, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2015, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2015, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2015, 7, 20)?, "海の日"),
            (NaiveDate::from_ymd_res(2015, 9, 21)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2015, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2015, 10, 12)?, "体育の日"),
            (NaiveDate::from_ymd_res(2015, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2015, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2015, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2015, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2015, 9, 22)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2016, 1, 11)?, "成人の日"),
            (NaiveDate::from_ymd_res(2016, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2016, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2016, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2016, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2016, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2016, 7, 18)?, "海の日"),
            (NaiveDate::from_ymd_res(2016, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2016, 9, 19)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2016, 9, 22)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2016, 10, 10)?, "体育の日"),
            (NaiveDate::from_ymd_res(2016, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2016, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2016, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2016, 3, 21)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2017, 1, 9)?, "成人の日"),
            (NaiveDate::from_ymd_res(2017, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2017, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2017, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2017, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2017, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2017, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2017, 7, 17)?, "海の日"),
            (NaiveDate::from_ymd_res(2017, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2017, 9, 18)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2017, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2017, 10, 9)?, "体育の日"),
            (NaiveDate::from_ymd_res(2017, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2017, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2017, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2018, 1, 8)?, "成人の日"),
            (NaiveDate::from_ymd_res(2018, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2018, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2018, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2018, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2018, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2018, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2018, 7, 16)?, "海の日"),
            (NaiveDate::from_ymd_res(2018, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2018, 9, 17)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2018, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2018, 10, 8)?, "体育の日"),
            (NaiveDate::from_ymd_res(2018, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2018, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2018, 12, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2018, 2, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2018, 4, 30)?, "振替休日"),
            (NaiveDate::from_ymd_res(2018, 9, 24)?, "振替休日"),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2019, 1, 14)?, "成人の日"),
            (NaiveDate::from_ymd_res(2019, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2019, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2019, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2019, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2019, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2019, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2019, 7, 15)?, "海の日"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2019, 9, 16)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2019, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2019, 10, 14)?, "体育の日"),
            (NaiveDate::from_ymd_res(2019, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2019, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2019, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2019, 8, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2019, 11, 4)?, "振替休日"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "天皇の即位の日"),
            (
                NaiveDate::from_ymd_res(2019, 10, 22)?,
                "即位礼正殿の儀が行われる日",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 30)?, "国民の休日"),
            (NaiveDate::from_ymd_res(2019, 5, 2)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2020, 1, 13)?, "成人の日"),
            (NaiveDate::from_ymd_res(2020, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2020, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2020, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2020, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2020, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2020, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2020, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2020, 7, 23)?, "海の日"),
            (NaiveDate::from_ymd_res(2020, 8, 10)?, "山の日"),
            (NaiveDate::from_ymd_res(2020, 9, 21)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2020, 9, 22)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2020, 7, 24)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2020, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2020, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2020, 2, 24)?, "振替休日"),
            (NaiveDate::from_ymd_res(2020, 5, 6)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2021, 1, 11)?, "成人の日"),
            (NaiveDate::from_ymd_res(2021, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2021, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2021, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2021, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2021, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2021, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2021, 7, 22)?, "海の日"),
            (NaiveDate::from_ymd_res(2021, 8, 8)?, "山の日"),
            (NaiveDate::from_ymd_res(2021, 9, 20)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2021, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2021, 7, 23)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2021, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2021, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2021, 8, 9)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2022, 1, 10)?, "成人の日"),
            (NaiveDate::from_ymd_res(2022, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2022, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2022, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2022, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2022, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2022, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2022, 7, 18)?, "海の日"),
            (NaiveDate::from_ymd_res(2022, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2022, 9, 19)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2022, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2022, 10, 10)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2022, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2022, 11, 23)?, "勤労感謝の日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2023, 1, 9)?, "成人の日"),
            (NaiveDate::from_ymd_res(2023, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2023, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2023, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2023, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2023, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2023, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2023, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2023, 7, 17)?, "海の日"),
            (NaiveDate::from_ymd_res(2023, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2023, 9, 18)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2023, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2023, 10, 9)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2023, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2023, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2024, 1, 8)?, "成人の日"),
            (NaiveDate::from_ymd_res(2024, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2024, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2024, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2024, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2024, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2024, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2024, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2024, 7, 15)?, "海の日"),
            (NaiveDate::from_ymd_res(2024, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2024, 9, 16)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2024, 9, 22)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2024, 10, 14)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2024, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2024, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2024, 8, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2024, 9, 23)?, "振替休日"),
            (NaiveDate::from_ymd_res(2024, 11, 4)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2025, 1, 13)?, "成人の日"),
            (NaiveDate::from_ymd_res(2025, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2025, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2025, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2025, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2025, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2025, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2025, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2025, 7, 21)?, "海の日"),
            (NaiveDate::from_ymd_res(2025, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2025, 9, 15)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2025, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2025, 10, 13)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2025, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2025, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2025, 2, 24)?, "振替休日"),
            (NaiveDate::from_ymd_res(2025, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2025, 11, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2026, 1, 12)?, "成人の日"),
            (NaiveDate::from_ymd_res(2026, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2026, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2026, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2026, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2026, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2026, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2026, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2026, 7, 20)?, "海の日"),
            (NaiveDate::from_ymd_res(2026, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2026, 9, 21)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2026, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2026, 10, 12)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2026, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2026, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2026, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2026, 9, 22)?, "国民の休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2027, 1, 11)?, "成人の日"),
            (NaiveDate::from_ymd_res(2027, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2027, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2027, 3, 21)?, "春分の日"),
            (NaiveDate::from_ymd_res(2027, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2027, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2027, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2027, 7, 19)?, "海の日"),
            (NaiveDate::from_ymd_res(2027, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2027, 9, 20)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2027, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2027, 10, 11)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2027, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2027, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2027, 3, 22)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2028, 1, 10)?, "成人の日"),
            (NaiveDate::from_ymd_res(2028, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2028, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2028, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2028, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2028, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2028, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2028, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2028, 7, 17)?, "海の日"),
            (NaiveDate::from_ymd_res(2028, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2028, 9, 18)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2028, 9, 22)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2028, 10, 9)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2028, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2028, 11, 23)?, "勤労感謝の日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2029, 1, 8)?, "成人の日"),
            (NaiveDate::from_ymd_res(2029, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2029, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2029, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2029, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2029, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2029, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2029, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2029, 7, 16)?, "海の日"),
            (NaiveDate::from_ymd_res(2029, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2029, 9, 17)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2029, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2029, 10, 8)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2029, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2029, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2029, 4, 30)?, "振替休日"),
            (NaiveDate::from_ymd_res(2029, 9, 24)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "元日"),
            (NaiveDate::from_ymd_res(2030, 1, 14)?, "成人の日"),
            (NaiveDate::from_ymd_res(2030, 2, 11)?, "建国記念の日"),
            (NaiveDate::from_ymd_res(2030, 2, 23)?, "天皇誕生日"),
            (NaiveDate::from_ymd_res(2030, 3, 20)?, "春分の日"),
            (NaiveDate::from_ymd_res(2030, 4, 29)?, "昭和の日"),
            (NaiveDate::from_ymd_res(2030, 5, 3)?, "憲法記念日"),
            (NaiveDate::from_ymd_res(2030, 5, 4)?, "みどりの日"),
            (NaiveDate::from_ymd_res(2030, 5, 5)?, "こどもの日"),
            (NaiveDate::from_ymd_res(2030, 7, 15)?, "海の日"),
            (NaiveDate::from_ymd_res(2030, 8, 11)?, "山の日"),
            (NaiveDate::from_ymd_res(2030, 9, 16)?, "敬老の日"),
            (NaiveDate::from_ymd_res(2030, 9, 23)?, "秋分の日"),
            (NaiveDate::from_ymd_res(2030, 10, 14)?, "スポーツの日"),
            (NaiveDate::from_ymd_res(2030, 11, 3)?, "文化の日"),
            (NaiveDate::from_ymd_res(2030, 11, 23)?, "勤労感謝の日"),
            (NaiveDate::from_ymd_res(2030, 5, 6)?, "振替休日"),
            (NaiveDate::from_ymd_res(2030, 8, 12)?, "振替休日"),
            (NaiveDate::from_ymd_res(2030, 11, 4)?, "振替休日"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
