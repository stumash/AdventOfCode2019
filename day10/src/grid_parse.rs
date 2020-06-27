use crate::point::Point;

pub fn string_to_points(s: &str) -> Vec<Point> {
    let mut points = Vec::new();
    for (j, line) in s.split("\n").enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => points.push(Point {
                    x: i as i32,
                    y: j as i32,
                }),
                _ => panic!(""),
            }
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![".#..#", ".....", "#####", "....#", "...##"].join("\n");

        let expected_output = vec![
            Point { x: 1, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 2 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 4 },
        ];

        let actual_output = string_to_points(&input);

        assert_eq!(actual_output, expected_output);
    }
}
