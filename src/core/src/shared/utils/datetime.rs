use chrono::{DateTime, Utc};

pub fn is_future(date: DateTime<Utc>) -> bool {
    date > Utc::now()
}
