use chrono::prelude::{DateTime, Utc};

type DtUtc = DateTime<Utc>;

pub struct Interval {
    start: DtUtc,
    end: Option<DtUtc>,
}

impl Interval {
    pub fn new(start: &DtUtc) -> Self {
        return Self {start: *start, end: None};
    }

    #[allow(dead_code)]
    pub fn new_now() -> Self {
        let now: DtUtc = Utc::now();
        return Self::new(&now);
    }

    pub fn end_at(&mut self, end: &DtUtc) {
        self.end = Some(*end);
    }

    #[allow(dead_code)]
    pub fn end_now(&mut self) {
        let now: DtUtc = Utc::now();
        self.end_at(&now);
    }

    pub fn has_end(&self) -> bool {
        return match self.end {
            Some(_) => true,
            None => false,
        };
    }

    pub fn get_start(&self) -> DtUtc {
        return self.start;
    }

    pub fn get_end(&self) -> Option<DtUtc> {
        return self.end;
    }
}

pub struct Day {
    pub overall_interval: Interval,
    pub breaks: Vec<Interval>,
    pub on_break: bool,
}

impl Day {
    pub fn new(start: &DtUtc) -> Self {
        return Self {overall_interval: Interval::new(start), breaks: Vec::new(), on_break: false};
    }

    #[allow(dead_code)]
    pub fn new_now() -> Self {
        let now: DtUtc = Utc::now();
        return Self::new(&now);
    }

    pub fn end_day_at(&mut self, at: &DtUtc) -> Result<(), &str> {
        if self.overall_interval.has_end() {
            return Err("Can't end the day because the day has already ended!");
        }
        else {
            self.overall_interval.end_at(at);
            self.on_break = false;
            return Ok(());
        }
    }

    #[allow(dead_code)]
    pub fn end_day_now(&mut self) -> Result<(), &str> {
        let now: DtUtc = Utc::now();
        return self.end_day_at(&now);
    }

    pub fn start_break(&mut self, at: &DtUtc) -> Result<(), &str> {
        if self.on_break {
            return Err("Can't start a break because day is already on break");
        }
        else {
            self.on_break = true;
            self.breaks.push(Interval::new(at));
            return Ok(());
        }
    }

    #[allow(dead_code)]
    pub fn start_break_now(&mut self) -> Result<(), &str> {
        let now: DtUtc = Utc::now();
        return self.start_break(&now);
    }

    pub fn end_current_break_at(&mut self, at: &DtUtc) -> Result<(), &str> {
        if !self.on_break {
            return Err("Can't end the break: currently not on break!");
        }
        else {
            self.breaks.last_mut().expect("Expected break to be ongoing!").end_at(at);
            self.on_break = false;
            return Ok(());
        }
    }

    #[allow(dead_code)]
    pub fn end_current_break_now(&mut self) -> Result<(), &str> {
        let now: DtUtc = Utc::now();
        return self.end_current_break_at(&now);
    }

    pub fn get_day_start(&self) -> DtUtc {
        return self.overall_interval.get_start();
    }

    pub fn get_day_end(&self) -> Option<DtUtc> {
        return self.overall_interval.get_end();
    }
}
