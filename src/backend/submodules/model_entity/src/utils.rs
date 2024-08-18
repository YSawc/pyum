use chrono::{Days, NaiveDateTime, Utc};

pub fn generate_1day_after_date_time() -> NaiveDateTime {
    (Utc::now().naive_utc())
        .checked_add_days(Days::new(1))
        .expect("failed to construct datetime")
}

pub fn generate_1day_before_date_time() -> NaiveDateTime {
    (Utc::now().naive_utc())
        .checked_sub_days(Days::new(1))
        .expect("failed to construct datetime")
}
