use std::fmt::{Debug, Display};

const MINUTES_IN_DAY: i32 = 24 * 60;

pub struct Clock(i32);

impl Clock {
    fn validate(raw_value: i32) -> i32 {
        let mut value = raw_value % MINUTES_IN_DAY;
        if value.is_negative() {
            value += MINUTES_IN_DAY;
        }
        value
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            0: Self::validate(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            0: Self::validate(self.0 + minutes),
        }
    }

    fn get_minutes(&self) -> i32 {
        self.0 % 60
    }

    fn get_hours(&self) -> i32 {
        (self.0 / 60) % 24
    }

    fn get_formatted_value(&self, value: i32) -> String {
        format!("{}{}", if value <= 9 {"0"} else {""}, value)
    }

    fn get_formatted_hours(&self) -> String {
        self.get_formatted_value(self.get_hours())
    }

    fn get_formatted_minutes(&self) -> String {
        self.get_formatted_value(self.get_minutes())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Clock").field(&self.0).finish()
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.get_formatted_hours(), self.get_formatted_minutes())
    }
}
