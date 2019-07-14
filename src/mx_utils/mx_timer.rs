use amethyst::{
    core::timing::Time,
};

#[derive(Clone)]
pub struct MxTimer {
    time_duration   :f32,
    time_remain     :f32,
    time_ratio      :f32,
    is_looping      :bool,
    is_stopped      :bool,
    is_paused       :bool,
}

#[allow(dead_code)]
impl MxTimer {
    pub fn new(duration:f32, repeat:bool) -> Self {
        MxTimer {
            time_duration:  duration,
            time_remain:    duration,
            time_ratio:     0.,
            is_looping:     repeat,
            is_stopped:     true,
            is_paused:      false,
        }
    }

    pub fn set(&mut self, duration:f32, repeat:bool) {
        self.time_duration  = duration;
        self.time_remain    = duration;
        self.time_ratio     = 0.0;
        self.is_looping     = repeat;
        self.is_stopped     = true;
        self.is_paused      = false;
    }

    pub fn update(&mut self, time: &Time) -> bool {
        let mut alarm = false;
        if !self.is_stopped && !self.is_paused {
            self.time_remain -= time.delta_real_time().as_nanos() as f32 * 0.000000001;
            self.time_ratio  = (self.time_duration - self.time_remain) / self.time_duration;
            if self.time_remain <= 0.0 {
                if self.is_looping {
                    self.reset();
                    self.start();
                    alarm = true;
                } else {
                    self.time_remain = 0.0;
                    self.time_ratio  = 1.0;
                    self.is_stopped  = true;
                    self.is_paused   = false;
                    alarm = true;
                }
            }
        }
        alarm
    }

    pub fn start(&mut self) {
        self.is_stopped = false;
        self.is_paused  = false;
    }

    pub fn reset(&mut self) {
        self.time_remain = self.time_duration;
        self.time_ratio  = 0.;
        self.is_stopped  = true;
        self.is_paused   = false;
    }

    pub fn jump_to_end(&mut self) {
        self.time_remain = 0.0;
        self.time_ratio  = 1.0;
        self.is_stopped  = true;
        self.is_paused   = false;
    }

    pub fn get_time_past(&self) -> f32 {
        self.time_duration - self.time_remain
    }

    pub fn get_time_remain(&self) -> f32 {
        self.time_remain
    }

    pub fn get_duration(&self) -> f32 {
        self.time_duration
    }

    pub fn get_ratio(&self) -> f32 {
        self.time_ratio
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resure(&mut self) {
        self.is_paused = false;
    }

    pub fn is_running(&self) -> bool {
        return !self.is_stopped;
    }
}