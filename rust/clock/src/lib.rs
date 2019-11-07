use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }.convert_minutes().convert_hours()
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self.convert_minutes().convert_hours()
    }

    fn convert_minutes(mut self) -> Self {
        self.hours = self.hours + self.minutes / 60;
        self.minutes = self.minutes % 60;

        if self.minutes < 0 {
            self.hours -= 1;
            self.minutes += 60;
        }

        self
    }

    fn convert_hours(mut self) -> Self {
        self.hours = (24 + self.hours % 24) % 24;

        self
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // :0>2:
        //   - Fill with 0s
        //   - Data aligned right
        //   - 2 caracters wide minimum
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}
