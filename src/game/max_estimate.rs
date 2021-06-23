use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MaxEstimate {
    pub max_seen: i32,
    pub estimate: i32,
    pub samples: i32,
}

impl MaxEstimate {
    pub fn new() -> Self {
        Self {
            max_seen: 0,
            estimate: 0,
            samples: 0
        }
    }

    pub fn sample(&mut self, value: i32) {
        if self.samples == 0 {
            self.estimate = value;
        } else if value >= self.max_seen {
            self.estimate = f32::ceil(self.estimate as f32 * 0.95 + value as f32 * 0.05) as i32;
        }
        self.samples += 1;
        if self.estimate > self.max_seen {
            self.max_seen = self.estimate;
        }
    }
}
