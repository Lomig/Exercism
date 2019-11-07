use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;

        self.hours = (self.hours + new_minutes.div_euclid(60)).rem_euclid(24);
        self.minutes = new_minutes.rem_euclid(60);

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
