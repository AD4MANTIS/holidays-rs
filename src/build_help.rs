use chrono::NaiveDate;

use crate::{prelude::*, Holiday, HolidayPerCountryMap, Year};

#[allow(clippy::needless_pass_by_value)]
pub fn build_year(
    years: Option<&std::ops::Range<Year>>,
    year: Year,
    holidays: impl IntoIterator<Item = (NaiveDate, &'static str)>,
    map: &mut HolidayPerCountryMap,
    country: Country,
    county_name: impl ToString,
) {
    if years.map_or(false, |r| !r.contains(&year)) {
        return;
    }

    let m = holidays
        .into_iter()
        .map(|h| {
            (
                h.0,
                Holiday::new(country, county_name.to_string(), h.0, h.1),
            )
        })
        .collect();

    map.insert(year, m);
}
