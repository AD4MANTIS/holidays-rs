//! Isle of Man
use super::*;

/// Generate holiday map for Isle of Man.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2000, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2000, 8, 28)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 2)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2000, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2001, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2001, 8, 27)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 1)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2001, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2002, 6, 4)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2002, 8, 26)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2002, 6, 7)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2002, 7, 5)?, "Tynwald Day"),
            (
                NaiveDate::from_ymd_res(2002, 6, 3)?,
                "Golden Jubilee of Elizabeth II",
            ),
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2003, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2003, 8, 25)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 6)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2003, 7, 7)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2004, 8, 30)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2004, 6, 4)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2004, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2005, 5, 30)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2005, 8, 29)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 3)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2005, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2006, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2006, 8, 28)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 2)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2006, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2007, 8, 27)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2007, 6, 1)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2007, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2008, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2008, 8, 25)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2008, 6, 6)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2008, 7, 7)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2009, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2009, 8, 31)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2009, 6, 5)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2009, 7, 6)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2009, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2010, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2010, 8, 30)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2010, 6, 4)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2010, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2011, 5, 30)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2011, 8, 29)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 3)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2011, 7, 5)?, "Tynwald Day"),
            (
                NaiveDate::from_ymd_res(2011, 4, 29)?,
                "Wedding of William and Catherine",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2012, 6, 4)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2012, 8, 27)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 1)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2012, 7, 5)?, "Tynwald Day"),
            (
                NaiveDate::from_ymd_res(2012, 6, 5)?,
                "Diamond Jubilee of Elizabeth II",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2013, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2013, 8, 26)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2013, 6, 7)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2013, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2014, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2014, 8, 25)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 6)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2014, 7, 7)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2015, 8, 31)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2015, 6, 5)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2015, 7, 6)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2015, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2016, 5, 30)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2016, 8, 29)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 3)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2016, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2017, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2017, 8, 28)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 2)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2017, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2018, 8, 27)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 1)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2018, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2019, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2019, 8, 26)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 7)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2019, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 5, 8)?, "May Day"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2020, 8, 31)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 5)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2020, 7, 6)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2020, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2021, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2021, 8, 30)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2021, 6, 4)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2021, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2022, 6, 2)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2022, 8, 29)?,
                "Late Summer Bank Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 3)?,
                "Platinum Jubilee of Elizabeth II; TT Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 5)?, "Tynwald Day"),
            (
                NaiveDate::from_ymd_res(2022, 9, 19)?,
                "State Funeral of Queen Elizabeth II",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2023, 8, 28)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 2)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2023, 7, 5)?, "Tynwald Day"),
            (
                NaiveDate::from_ymd_res(2023, 5, 8)?,
                "Coronation of Charles III",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2024, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2024, 8, 26)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 7)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2024, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2025, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2025, 8, 25)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2025, 7, 7)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2026, 8, 31)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2026, 6, 5)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2026, 7, 6)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2026, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2027, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2027, 8, 30)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2027, 6, 4)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2027, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2028, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2028, 8, 28)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 2)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2028, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2029, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2029, 8, 27)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2029, 6, 1)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2029, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2030, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2030, 8, 26)?,
                "Late Summer Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 7)?, "TT Bank Holiday"),
            (NaiveDate::from_ymd_res(2030, 7, 5)?, "Tynwald Day"),
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::IM,
        "Isle of Man",
    );

    Ok(map)
}
