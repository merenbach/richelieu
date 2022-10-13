#[cfg(test)]
mod tests {
    use super::Step;

    #[test]
    fn step_from_works() {
        let table = &[
            (Step::NW, 0b00),
            (Step::NE, 0b01),
            (Step::SW, 0b10),
            (Step::SE, 0b11),
            (Step::NW, 0b100),
            (Step::NE, 0b101),
            (Step::SW, 0b110),
            (Step::SE, 0b111),
            (Step::NW, 0),
            (Step::NE, 1),
            (Step::SW, 2),
            (Step::SE, 3),
            (Step::NW, 4),
            (Step::NE, 5),
            (Step::SW, 6),
            (Step::SE, 7),
        ];
        for t in table {
            assert_eq!(t.0, Step::from(t.1));
        }
    }
}

/// A step for the bishop to take.
#[derive(Debug, PartialEq)]
pub enum Step {
    /// northwest (up and left)
    NW,
    /// northeast (up and right)
    NE,
    /// southwest (down and left)
    SW,
    /// southeast (down and right)
    SE,
}

impl Step {
    /// Transform coordinates based on the direction of this step.
    pub fn transform(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Step::NW => (x.saturating_sub(1), y.saturating_sub(1)),
            Step::NE => (x.saturating_add(1), y.saturating_sub(1)),
            Step::SW => (x.saturating_sub(1), y.saturating_add(1)),
            Step::SE => (x.saturating_add(1), y.saturating_add(1)),
        }
    }
}

impl From<u8> for Step {
    fn from(v: u8) -> Self {
        match v & 0b11 {
            // &0b11 is the same here (with unsigned type) as %4
            0b00 => Step::NW, // same as == 0
            0b01 => Step::NE, // same as == 1
            0b10 => Step::SW, // same as == 2
            0b11 => Step::SE, // same as == 3
            _ => unreachable!("Last two bits will always yield one of four values"),
        }
    }
}

/// Convert a byte slice to a sequence of steps.
/// The count of items returned is restricted to `limit`.
/// If `limit` is zero, all created steps will be returned.
pub fn steps_from_bytes(xs: &[u8], limit: usize) -> Vec<Step> {
    xs.iter()
        .flat_map(|b| (0..8).step_by(2).map(move |i| b >> i))
        .take(if limit == 0 { 4 * xs.len() } else { limit })
        .map(Step::from)
        .collect()
}
