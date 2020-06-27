use num_rational::Ratio;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn slope(&self, other: &Point) -> DirectedSlope {
        if self == other {
            panic!(
                "no slope can exist between equal points {:?} {:?}",
                self, other
            );
        }

        let rise = other.y - self.y;
        let run = other.x - self.x;

        let slope = match run == 0 {
            true => Slope::Inf,
            false => Slope::Ratio(Ratio::new(rise, run)),
        };

        // how to calculate direction ?!
        let direction = {
            if rise > 0 {
                SlopeDirection::Up
            } else if rise < 0 {
                SlopeDirection::Down
            } else {
                if run > 0 {
                    SlopeDirection::Up
                } else {
                    SlopeDirection::Down
                }
            }
        };

        DirectedSlope { slope, direction }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct DirectedSlope {
    slope: Slope,
    direction: SlopeDirection,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Slope {
    Inf,
    Ratio(Ratio<i32>),
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum SlopeDirection {
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn x_equals_y(xy: i32) -> Point {
        Point { x: xy, y: xy }
    }

    #[test]
    fn test_slope() {
        let origin = origin();
        let slopeA = origin.slope(&x_equals_y(5));
        let slopeB = origin.slope(&x_equals_y(7));
        let slope1 = DirectedSlope {
            slope: Slope::Ratio(Ratio::from(1)),
            direction: SlopeDirection::Up,
        };

        assert_eq!(slopeA, slopeB);
        assert_eq!(slopeA, slope1);
        assert_eq!(slope1, slopeB);
    }
}
