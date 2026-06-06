use std::f32;

#[derive(Clone, Copy, Debug, PartialEq)]
enum IntervalType {
    Empty,
    Universe,
    Other,
}

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    interval_type: IntervalType,
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        if min == f32::INFINITY && max == -f32::INFINITY {
            Self {
                interval_type: IntervalType::Empty,
                min,
                max,
            }
        } else if min == -f32::INFINITY && max == f32::INFINITY {
            Self {
                interval_type: IntervalType::Universe,
                min,
                max,
            }
        } else {
            Self {
                interval_type: IntervalType::Other,
                min,
                max,
            }
        }
    }

    pub fn new_overlap(a: &Interval, b: &Interval) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }

    pub fn extend(&mut self, other: &Interval) {
        *self = Self::new(self.min.min(other.min), self.max.max(other.max));
    }

    pub fn empty() -> Self {
        Self::new(f32::INFINITY, -f32::INFINITY)
    }

    pub fn universe() -> Self {
        Self::new(-f32::INFINITY, f32::INFINITY)
    }

    pub fn half_universe_pos() -> Self {
        Self::new(0., f32::INFINITY)
    }

    pub fn half_universe_neg() -> Self {
        Self::new(-f32::INFINITY, 0.)
    }

    pub fn size(&self) -> f32 {
        if self.interval_type == IntervalType::Empty {
            0.
        } else if self.interval_type == IntervalType::Universe {
            f32::INFINITY
        } else {
            self.max - self.min
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min == f32::INFINITY && self.max == -f32::INFINITY
    }

    pub fn is_universe(&self) -> bool {
        self.min == -f32::INFINITY && self.max == f32::INFINITY
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}
