use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Score(f64);

impl Score {
    pub fn new(val: f64) -> Option<Score> {
        if val.is_nan() {
            None
        } else {
            Some(Score(val))
        }
    }

    pub fn val(self) -> f64 {
        let Score(val) = self;
        val
    }
}

impl Eq for Score {}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
