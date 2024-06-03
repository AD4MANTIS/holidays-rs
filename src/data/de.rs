use std::collections::{BTreeMap, HashMap};

use chrono::NaiveDate;

use crate::{Holiday, HolidayCombo, NaiveDateExt, RepeatingHoliday};

/// Germany.
#[cfg(feature = "DE")]
use super::*;

pub enum Subdivisions {
    BB,
    BE,
    BW,
    BY,
    BYP,
    HB,
    HE,
    HH,
    MV,
    NI,
    NW,
    RP,
    SH,
    SL,
    SN,
    ST,
    TH,
}

/// Generate holiday map for Germany.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HolidayCombo> {
    let mut map = HashMap::new();

    RepeatingHoliday::new_fixed("Neujahr", 1, 1);
    RepeatingHoliday::new_fixed("Erster Mai", 5, 1);

    RepeatingHoliday::new_fixed("Tag der Deutschen Einheit", 10, 3);

    RepeatingHoliday::new_fixed("Erster Weihnachtstag", 12, 25);
    RepeatingHoliday::new_fixed("Zweiter Weihnachtstag", 12, 26);

    if build_year(years, 2000) {
        let mut m = vec![
            Holiday::new(
                Country::DE,
                "Germany",
                NaiveDate::from_ymd_res(2000, 4, 21)?,
                "Karfreitag",
            ),
            Holiday::new(
                Country::DE,
                "Germany",
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Ostermontag",
            ),
            Holiday::new(
                Country::DE,
                "Germany",
                NaiveDate::from_ymd_res(2000, 6, 1)?,
                "Christi Himmelfahrt",
            ),
            Holiday::new(
                Country::DE,
                "Germany",
                NaiveDate::from_ymd_res(2000, 6, 12)?,
                "Pfingstmontag",
            ),
        ]
        .into_iter()
        .map(|h| (h.date, h))
        .collect();

        map.insert(2000, m);
    }

    if build_year(years, 2001) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2001, 4, 13)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2001, 4, 16)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2001, 5, 24)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2001, 6, 4)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2001, m);
    }

    if build_year(years, 2002) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2002, 3, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2002, 4, 1)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2002, 5, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2002, 5, 20)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2002, m);
    }

    if build_year(years, 2003) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2003, 4, 18)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2003, 4, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2003, 5, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2003, 6, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2003, m);
    }

    if build_year(years, 2004) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2004, 4, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2004, 4, 12)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2004, 5, 20)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2004, 5, 31)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2004, m);
    }

    if build_year(years, 2005) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2005, 3, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2005, 3, 28)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2005, 5, 5)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2005, 5, 16)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2005, m);
    }

    if build_year(years, 2006) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2006, 4, 14)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2006, 4, 17)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2006, 5, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2006, 6, 5)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2006, m);
    }

    if build_year(years, 2007) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2007, 4, 6)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2007, 4, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2007, 5, 17)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2007, 5, 28)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2007, m);
    }

    if build_year(years, 2008) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2008, 3, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2008, 3, 24)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        m.insert(
            date,
            Holiday::new(
                Country::DE,
                "Germany",
                date,
                "Christi Himmelfahrt, Erster Mai",
            ),
        );
        let date = NaiveDate::from_ymd_res(2008, 5, 12)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2008, m);
    }

    if build_year(years, 2009) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2009, 4, 10)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2009, 4, 13)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2009, 5, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2009, 6, 1)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2009, m);
    }

    if build_year(years, 2010) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2010, 4, 2)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2010, 4, 5)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2010, 5, 13)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2010, 5, 24)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2010, m);
    }

    if build_year(years, 2011) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2011, 4, 22)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2011, 4, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2011, 6, 2)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2011, 6, 13)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2011, m);
    }

    if build_year(years, 2012) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2012, 4, 6)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2012, 4, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2012, 5, 17)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2012, 5, 28)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2012, m);
    }

    if build_year(years, 2013) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2013, 3, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2013, 4, 1)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2013, 5, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2013, 5, 20)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2013, m);
    }

    if build_year(years, 2014) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2014, 4, 18)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2014, 4, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2014, 5, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2014, 6, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2014, m);
    }

    if build_year(years, 2015) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2015, 4, 3)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2015, 4, 6)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2015, 5, 14)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2015, 5, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2015, m);
    }

    if build_year(years, 2016) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2016, 3, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2016, 3, 28)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2016, 5, 5)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2016, 5, 16)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2016, m);
    }

    if build_year(years, 2017) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2017, 4, 14)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2017, 4, 17)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2017, 5, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2017, 6, 5)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        let date = NaiveDate::from_ymd_res(2017, 10, 31)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Reformationstag"),
        );

        map.insert(2017, m);
    }

    if build_year(years, 2018) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2018, 3, 30)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2018, 4, 2)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2018, 5, 10)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2018, 5, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2018, m);
    }

    if build_year(years, 2019) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2019, 4, 19)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2019, 4, 22)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2019, 5, 30)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2019, 6, 10)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2019, m);
    }

    if build_year(years, 2020) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2020, 4, 10)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2020, 4, 13)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2020, 5, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2020, 6, 1)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2020, m);
    }

    if build_year(years, 2021) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2021, 4, 2)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2021, 4, 5)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2021, 5, 13)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2021, 5, 24)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2021, m);
    }

    if build_year(years, 2022) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2022, 4, 15)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2022, 4, 18)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2022, 5, 26)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2022, 6, 6)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2022, m);
    }

    if build_year(years, 2023) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2023, 4, 7)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2023, 4, 10)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2023, 5, 18)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2023, 5, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2023, m);
    }

    if build_year(years, 2024) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2024, 3, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2024, 4, 1)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2024, 5, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2024, 5, 20)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2024, m);
    }

    if build_year(years, 2025) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2025, 4, 18)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2025, 4, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2025, 5, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2025, 6, 9)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2025, m);
    }

    if build_year(years, 2026) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2026, 4, 3)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2026, 4, 6)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2026, 5, 14)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2026, 5, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2026, m);
    }

    if build_year(years, 2027) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2027, 3, 26)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2027, 3, 29)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2027, 5, 6)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2027, 5, 17)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2027, m);
    }

    if build_year(years, 2028) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2028, 4, 14)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2028, 4, 17)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2028, 5, 25)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2028, 6, 5)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2028, m);
    }

    if build_year(years, 2029) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2029, 3, 30)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2029, 4, 2)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2029, 5, 10)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2029, 5, 21)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2029, m);
    }

    if build_year(years, 2030) {
        let mut m = BTreeMap::new();

        let date = NaiveDate::from_ymd_res(2030, 4, 19)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Karfreitag"),
        );
        let date = NaiveDate::from_ymd_res(2030, 4, 22)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Ostermontag"),
        );

        let date = NaiveDate::from_ymd_res(2030, 5, 30)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Christi Himmelfahrt"),
        );
        let date = NaiveDate::from_ymd_res(2030, 6, 10)?;
        m.insert(
            date,
            Holiday::new(Country::DE, "Germany", date, "Pfingstmontag"),
        );

        map.insert(2030, m);
    }

    Ok(HolidayCombo {
        repeating_holidays: vec![],
        special_days: map,
    })
}
