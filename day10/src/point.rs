use num_rational::Ratio;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn distance_squared(p1: &Point, p2: &Point) -> i32 {
        (p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2)
    }
}

// `DirectedSlope` is ordered counter-clockwise starting at negative infinity, which is
// represented as
// ```rust
// DirectedSlope {
//     direction: SlopeDirection::PostiveX,
//     slope: Slope::Inf,
// }
// ```
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Hash)]
pub struct DirectedSlope {
    direction: SlopeDirection,
    slope: Slope,
}
impl From<(&Point, &Point)> for DirectedSlope {
    fn from((from, to): (&Point, &Point)) -> DirectedSlope {
        let rise = to.y - from.y;
        let run = to.x - from.x;

        let slope = match run == 0 {
            true => Slope::Inf,
            false => Slope::Ratio(Ratio::new(rise, run)),
        };

        let direction = {
            if run > 0 {
                SlopeDirection::PostiveX
            } else if run < 0 {
                SlopeDirection::NegativeX
            } else {
                if rise > 0 {
                    // pos inf
                    SlopeDirection::NegativeX
                } else if rise < 0 {
                    // neg inf
                    SlopeDirection::PostiveX
                } else {
                    // rise and run both zero
                    panic!(
                        "no slope can exist between equal points {:?} {:?}",
                        from, to
                    );
                }
            }
        };

        DirectedSlope { slope, direction }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Hash)]
pub enum Slope {
    Inf,
    Ratio(Ratio<i32>),
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Hash)]
pub enum SlopeDirection {
    PostiveX,
    NegativeX,
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
        let slope_a = DirectedSlope::from((&origin, &x_equals_y(5)));
        let slope_b = DirectedSlope::from((&origin, &x_equals_y(7)));
        let slope1 = DirectedSlope {
            slope: Slope::Ratio(Ratio::from(1)),
            direction: SlopeDirection::PostiveX,
        };

        assert_eq!(slope_a, slope_b);
        assert_eq!(slope_a, slope1);
        assert_eq!(slope1, slope_b);
    }

    #[test]
    fn test_directed_slope_ordering() {
        let origin = origin();
        // some points starting on the negative y axis, going counter-clockwise around the
        // cartesian plane
        let points = vec![
            Point { x: 0, y: -1 },
            Point { x: 1, y: -3 },
            Point { x: 1, y: -2 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 4 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 4 },
            Point { x: -1, y: 3 },
            Point { x: -2, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: -1 },
            Point { x: -1, y: -2 },
            Point { x: -1, y: -6 },
        ];
        let directed_slopes: Vec<DirectedSlope> = points
            .iter()
            .map(|p| DirectedSlope::from((&origin, p)))
            .collect();
        let mut sorted_directed_slopes = directed_slopes.clone();
        sorted_directed_slopes.sort();

        assert_eq!(directed_slopes, sorted_directed_slopes);
    }
}
