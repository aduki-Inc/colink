use chrono::{NaiveDateTime, Duration, Utc};

// Function to get current date plus numbers of days supplied
pub async fn future_date(days: Option<i64>) -> Option<NaiveDateTime> {
  // If expiry days are supplied convert to future date from today
  let expiry_date: Option<NaiveDateTime> = if days.is_some() {
    let days_to_be_added: i64 = days.unwrap_or(0);
    let initial_date = Utc::now();

    let future_date = initial_date + Duration::TimeDelta::try_days(days_to_be_added);

    Some(future_date.naive_utc())
  }
  else {
    None
  };

  return expiry_date;
}