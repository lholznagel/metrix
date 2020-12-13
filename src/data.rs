use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub enum MetricData {
    Counter(u64),
    Gauge(u64),
    Duration(Vec<u64>),
}

impl MetricData {
    pub fn new_current_timestamp() -> Self {
        let val = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.as_millis() as u64)
            .unwrap_or(0);
        MetricData::Gauge(val)
    }

    /// Increases the a counter or gauge type by the given value
    /// Will fail silently if the type is from type time
    pub fn increase(&mut self, val: u64) {
        match self {
            Self::Counter(x) => *x += val,
            Self::Gauge(x) => *x += val,
            _ => (),
        }
    }

    /// Decreases the a gauge type by the given value
    /// Will fail silently if the type is from type counter or time
    pub fn decrease(&mut self, val: u64) {
        match self {
            Self::Gauge(x) => *x -= val,
            _ => (),
        }
    }

    /// Sets the given value for a counter or a gauge type
    /// Will fail silently if the type is from type duration
    pub fn set(&mut self, val: u64) {
        match self {
            Self::Counter(x) => *x = val,
            Self::Gauge(x) => *x = val,
            _ => (),
        }
    }

    /// Adds the given value to the timing
    /// Will fail silently if the type is from type duration or gauge
    pub fn duration(&mut self, val: u64) {
        match self {
            Self::Duration(x) => {
                x.push(val);
                x.sort_unstable();
            },
            _ => (),
        }
    }

    /// Creates a new timestamp and inserts it as gauge
    /// Will fail silently if the type is from type counter or duration
    pub fn current_timestamp(&mut self) {
        match self {
            Self::Gauge(x) => {
                let val = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|x| x.as_millis() as u64)
                    .unwrap_or(0);
                *x = val;
            }
            _ => (),
        }
    }
}