use chrono::prelude::{DateTime, Utc};

type DtUtc = DateTime<Utc>;

pub struct Interval {
    start: DtUtc,
    end: Option<DtUtc>,
}

impl Interval {
    fn new(start: &DtUtc) -> Self {
        return Self {start: *start, end: None};
    }

    fn new_now() -> Self {
        let now: DtUtc = Utc::now();
        return Self::new(&now);
    }

    fn end_at(&mut self, end: &DtUtc) {
        self.end = Some(*end);
    }

    fn end_now(&mut self) {
        let now: DtUtc = Utc::now();
        self.end_at(&now);
    }

    fn has_end(&self) -> bool {
        return match self.end {
            Some(_) => true,
            None => false,
        };
    }
}

pub struct Day {
    overall_interval: Interval,
    breaks: Vec<Interval>,
    on_break: bool,
}

impl Day {
    fn new(start: &DtUtc) -> Self {
        return Self {overall_interval: Interval::new(start), breaks: Vec::new(), on_break: false};
    }

    fn new_now() -> Self {
        let now: DtUtc = Utc::now();
        return Self::new(&now);
    }

    fn end_day_at(&mut self, at: &DtUtc) -> Result<(), &str> {
        if self.overall_interval.has_end() {
            return Err("Can't end the day because the day has already ended!");
        }
        else {
            self.overall_interval.end_at(at);
            self.on_break = false;
            return Ok(());
        }
    }

    fn end_day_now(&mut self) -> Result<(), &str> {
        let now: DtUtc = Utc::now();
        return self.end_day_at(&now);
    }

    fn start_break(&mut self, at: &DtUtc) -> Result<(), &str> {
        if self.on_break {
            return Err("Can't start a break because day is already on break");
        }
        else {
            self.on_break = true;
            self.breaks.push(Interval::new(at));
            return Ok(());
        }
    }

    fn start_break_now(&mut self) -> Result<(), &str> {
        let now: DtUtc = Utc::now();
        return self.start_break(&now);
    }

    fn end_current_break_at(&mut self, at: &DtUtc) -> Result<(), &str> {
        if !self.on_break {
            return Err("Can't end the break: currently not on break!");
        }
        else {
            self.breaks.last_mut().expect("Expected break to be ongoing!").end_at(at);
            self.on_break = false;
            return Ok(());
        }
    }

    fn end_current_break_now(&mut self) -> Result<(), &str> {
        let now: DtUtc = Utc::now();
        return self.end_current_break_at(&now);
    }
}
