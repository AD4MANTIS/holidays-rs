//! Finland
#[allow(clippy::wildcard_imports)]
use super::*;

/// Generate holiday map for Finland.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2000, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2000, 6, 23)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2000, 6, 24)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2000, 11, 4)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2000, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2000, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2001, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2001, 6, 22)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2001, 6, 23)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2001, 11, 3)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2001, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2001, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2002, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2002, 6, 21)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2002, 6, 22)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2002, 11, 2)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2002, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2002, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2003, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2003, 6, 20)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2003, 6, 21)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2003, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2003, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2004, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2004, 6, 25)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2004, 6, 26)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2004, 11, 6)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2004, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2004, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2005, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2005, 6, 24)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2005, 6, 25)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2005, 11, 5)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2005, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2006, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2006, 6, 23)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2006, 6, 24)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2006, 11, 4)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2006, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2007, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2007, 6, 22)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2007, 6, 23)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2007, 11, 3)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2007, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2008, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Helatorstai; Vappu"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2008, 6, 20)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2008, 6, 21)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2008, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2008, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2009, 6, 19)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2009, 6, 20)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2009, 10, 31)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2009, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2009, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2010, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2010, 6, 25)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2010, 6, 26)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2010, 11, 6)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2010, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2010, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2011, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2011, 6, 24)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2011, 6, 25)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2011, 11, 5)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2011, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2012, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2012, 6, 22)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2012, 6, 23)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2012, 11, 3)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2012, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2013, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2013, 6, 21)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2013, 6, 22)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2013, 11, 2)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2013, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2014, 6, 20)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2014, 6, 21)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2014, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2015, 6, 19)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2015, 6, 20)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2015, 10, 31)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2015, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2016, 6, 24)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2016, 6, 25)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2016, 11, 5)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2016, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2017, 6, 23)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2017, 6, 24)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2017, 11, 4)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2017, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2018, 6, 22)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2018, 6, 23)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2018, 11, 3)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2018, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2019, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2019, 6, 21)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2019, 6, 22)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2019, 11, 2)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2019, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2020, 6, 19)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2020, 6, 20)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2020, 10, 31)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2020, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2021, 6, 25)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2021, 6, 26)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2021, 11, 6)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2021, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2021, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2022, 6, 24)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2022, 6, 25)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2022, 11, 5)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2022, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2023, 6, 23)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2023, 6, 24)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2023, 11, 4)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2023, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2024, 6, 21)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2024, 6, 22)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2024, 11, 2)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2024, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2025, 6, 20)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2025, 6, 21)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2025, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2026, 6, 19)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2026, 6, 20)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2026, 10, 31)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2026, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2027, 6, 25)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2027, 6, 26)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2027, 11, 6)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2027, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2027, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2028, 6, 23)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2028, 6, 24)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2028, 11, 4)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2028, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2029, 6, 22)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2029, 6, 23)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2029, 11, 3)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2029, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Uudenvuodenpäivä"),
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Loppiainen"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Pitkäperjantai"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "2. pääsiäispäivä"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Vappu"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Helatorstai"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Helluntaipäivä"),
            (NaiveDate::from_ymd_res(2030, 6, 21)?, "Juhannusaatto"),
            (NaiveDate::from_ymd_res(2030, 6, 22)?, "Juhannuspäivä"),
            (NaiveDate::from_ymd_res(2030, 11, 2)?, "Pyhäinpäivä"),
            (NaiveDate::from_ymd_res(2030, 12, 6)?, "Itsenäisyyspäivä"),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "Jouluaatto"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Joulupäivä"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Tapaninpäivä"),
        ],
        &mut map,
        Country::FI,
        "Finland",
    );

    Ok(map)
}
