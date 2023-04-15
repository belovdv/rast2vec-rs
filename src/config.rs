#[derive(Debug, clap::Parser, Clone)]
pub struct Config {
    #[arg(long)]
    pub workspace: std::path::PathBuf,

    #[arg(long)]
    pub dir_source: Option<std::path::PathBuf>,

    #[arg(short = 't', default_value_t = false)]
    pub timers: bool,
}

impl Config {
    #[must_use]
    pub fn timer<T: ToString>(&self, name: T) -> MaybeTimer {
        MaybeTimer::start(self.timers, name)
    }

    // pub fn time<T>(&self, name: &str, f: &mut dyn FnMut() -> T) -> T {
    //     let t = self.timer(name);
    //     let r = f();
    //     t.stop();
    //     r
    // }
}

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
    pub fn stop(self) -> String {
        format!("{:50} took {:?}", self.name, self.point())
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
    pub fn stop(self) {
        self.0.map(|t| eprintln!("{}", t.stop()));
    }
}
