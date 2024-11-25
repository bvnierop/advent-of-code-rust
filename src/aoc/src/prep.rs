use time::{OffsetDateTime, Month};

#[derive(Debug, Clone)]
pub enum YearOrDay {
    Year(u16),
    Day(u8),
}

pub fn parse_year_or_day(arg: &str) -> Result<YearOrDay, String> {
    let num: u16 = arg.parse().map_err(|_| "Not a number".to_string())?;
    
    if num >= 2015 {
        Ok(YearOrDay::Year(num))
    } else if (1..=25).contains(&(num as u8)) {
        Ok(YearOrDay::Day(num as u8))
    } else {
        Err("Invalid year or day".to_string())
    }
}

fn get_current_advent() -> (u16, u8) {
    let now = OffsetDateTime::now_utc();
    let year = now.year() as u16;
    
    let day = if now.month() == Month::December && (1..=25).contains(&now.day()) {
        now.day() as u8
    } else {
        1
    };
    
    (year, day)
}

pub fn extract_year_and_day(first: Option<YearOrDay>, second: Option<YearOrDay>) -> (Option<u16>, Option<u8>) {
    match (first, second) {
        (Option::None, Option::None) => (None, None),
        (Some(YearOrDay::Year(y)), Option::None) => (Some(y), None),
        (Some(YearOrDay::Day(d)), Option::None) => (None, Some(d)),
        (Some(YearOrDay::Year(y)), Some(YearOrDay::Day(d))) => (Some(y), Some(d)),
        (Some(YearOrDay::Day(d)), Some(YearOrDay::Year(y))) => (Some(y), Some(d)),
        _ => (None, None), // Invalid combinations (year+year or day+day)
    }
}

pub fn handle(first: Option<YearOrDay>, second: Option<YearOrDay>) {
    let (year, day) = extract_year_and_day(first, second);
    let (current_year, current_day) = get_current_advent();
    let year = year.unwrap_or(current_year);
    let day = day.unwrap_or(current_day);
    
    println!("Preparing environment for year {} day {}...", year, day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_day_parsing() {
        assert!(matches!(parse_year_or_day("2024").unwrap(), YearOrDay::Year(2024)));
        assert!(matches!(parse_year_or_day("1").unwrap(), YearOrDay::Day(1)));
        assert!(matches!(parse_year_or_day("25").unwrap(), YearOrDay::Day(25)));
        assert!(parse_year_or_day("26").is_err());
        assert!(parse_year_or_day("2014").is_err());
    }
} 