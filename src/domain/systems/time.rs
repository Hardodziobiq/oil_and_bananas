/// TIME CONSTRAINT
const STANDARD_MILISECONDS_PER_MONTH: f32 = 20_000.0;

/// DEFAULT CONSTRAINTS
const INIT_MONTH: u32 = 1;
const INIT_YEAR: u32 = 1920;
const INIT_MONTH_PROGRESS: f32 = 0.0;
const INIT_TIME_MULTIPLYER: f32 = 1.0;
const INIT_LAST_DT: f32 = 0.0;

/// TIME STRUCT
pub struct Time {
    month: u32,
    year: u32,
    month_progress: f32,
    time_multiplyer: f32,
    last_dt: f32,
}

/// TIME DEFAULT
impl Default for Time {
    fn default() -> Self {
        Self {
            month: INIT_MONTH,
            year: INIT_YEAR,
            month_progress: INIT_MONTH_PROGRESS,
            time_multiplyer: INIT_TIME_MULTIPLYER,
            last_dt: INIT_LAST_DT,
        }
    }
}

/// TIME MAIN FUNCTIONS
impl Time {
    /// UPDATING FUNCTIONS
    pub fn update(&mut self, dt: f32) {
        if self.update_month_progress(dt) && self.next_month() {
            self.next_year();
        }
    }
    pub fn change_time_multiplyer(&mut self, new_time_multiplyer: TimeMultiplyer) {
        self.time_multiplyer = new_time_multiplyer.to_float();
    }

    /// GETTING FUNCTIONS
    pub fn get_month(&self) -> u32 {
        self.month
    }
    pub fn get_year(&self) -> u32 {
        self.year
    }
    pub fn get_month_progress(&self) -> f32 {
        self.month_progress
    }
    pub fn get_last_dt(&self) -> f32 {
        self.last_dt
    }
}

/// TIME UTILITY FUNCTIONS
impl Time {
    /// UPDATE FUNCTION UTILITY FUNCTIONS
    fn update_month_progress(&mut self, dt: f32) -> bool {
        self.month_progress += dt * self.time_multiplyer / STANDARD_MILISECONDS_PER_MONTH;
        if 1.0 < self.month_progress {
            self.month_progress -= 1.0;
            return true;
        }
        false
    }
    fn next_month(&mut self) -> bool {
        self.month += 1;
        if 12 < self.month {
            self.month = 1;
            return true;
        }
        false
    }
    fn next_year(&mut self) {
        self.year += 1;
    }
}

/// TIME MULTIPLYER ENUM
pub enum TimeMultiplyer {
    Freeze,
    Times1,
    Times2,
    Times4,
}

/// TIME MULTIPLYER UTILITY FUNCTIONS
impl TimeMultiplyer {
    /// CHANGE MULTIPLYER FUNCTION UTILITY FUNCTIONS
    fn to_float(&self) -> f32 {
        match self {
            TimeMultiplyer::Freeze => 0.0,
            TimeMultiplyer::Times1 => 1.0,
            TimeMultiplyer::Times2 => 2.0,
            TimeMultiplyer::Times4 => 4.0,
        }
    }
}
