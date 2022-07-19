use std::time::Instant;

pub(crate) struct Timer {
    start_time: Instant,
}

impl Timer {
    /// start a timer
    pub(crate) fn start() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// stop the timer and get the elapsed time
    pub(crate) fn stop(&self) -> String {
        let elapsed = self.start_time.elapsed();
        let millis = elapsed.as_millis() as u64;
        if millis > 1000 {
            let secs = elapsed.as_secs();
            format!("{} seconds, {} ms", secs, millis - secs * 1000)
        } else {
            format!("{} ms", millis)
        }
    }
}
