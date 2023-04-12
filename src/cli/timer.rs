#![allow(dead_code)]

#[derive(Debug)]
pub struct Timer {
    name: String,
    start: std::time::Instant,
}

impl Timer {
    #[must_use]
    pub fn start<T: ToString>(name: T) -> Self {
        Timer {
            name: name.to_string(),
            start: std::time::Instant::now(),
        }
    }
    pub fn point(&self) -> std::time::Duration {
        self.start.elapsed()
    }
    pub fn stop(self) -> std::time::Duration {
        self.start.elapsed()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn stop_m(self) -> String {
        format!("{} took {:?}", self.name(), self.point())
    }
}

pub struct MaybeTimer(Option<Timer>);
impl MaybeTimer {
    pub fn start<T: ToString>(create: bool, name: T) -> Self {
        match create {
            true => MaybeTimer(Some(Timer::start(name))),
            false => MaybeTimer(None),
        }
    }
    #[allow(dead_code)]
    pub fn point(&self) -> Option<std::time::Duration> {
        self.0.as_ref().map(|s| s.point())
    }
    #[allow(dead_code)]
    pub fn name(&self) -> Option<&str> {
        self.0.as_ref().map(|s| s.name())
    }
    #[allow(dead_code)]
    pub fn stop(self) -> Option<String> {
        self.0.map(|s| s.stop_m())
    }
    pub fn stop_out(self) {
        match self.0 {
            Some(t) => eprintln!("{}", t.stop_m()),
            None => {}
        }
    }
}
