mod grid_parse;
mod point;

use grid_parse::string_to_points;
use num_rational::Ratio;
use point::*;
use std::collections::HashMap;
use std::fs::read_to_string;

fn best_point(points: &Vec<Point>) -> (Point, usize) {
    let mut current_best = (Point { x: -1, y: -1 }, usize::MIN);
    for p1 in points {
        let mut slopes: HashMap<DirectedSlope, Point> = HashMap::new();

        for p2 in points.iter().filter(|p2| *p2 != p1) {
            slopes.insert(DirectedSlope::from((p1, p2)), p2.clone());
        }
        if slopes.len() > current_best.1 {
            current_best = (p1.clone(), slopes.len());
        }
    }
    current_best
}

fn laser_sorted_points(points: &Vec<Point>) -> Vec<Point> {
    let (station_point, _) = best_point(points);

    // group points by slope from station_point
    let mut points_by_slope: HashMap<DirectedSlope, Vec<Point>> = HashMap::new();
    for p in points.iter() {
        if *p == station_point {
            continue;
        }
        points_by_slope
            .entry(DirectedSlope::from((&station_point, &*p)))
            .or_insert(Vec::new())
            .push(p.clone());
    }

    // sort points
    let mut sorted_tuples: Vec<(DirectedSlope, Vec<Point>)> = points_by_slope.into_iter().collect();
    sorted_tuples.sort_by(|(ds1, _), (ds2, _)| ds1.cmp(ds2));
    sorted_tuples.iter_mut().for_each(|(_, ps)| {
        ps.sort_by(|p1, p2| {
            Point::distance_squared(&station_point, p1)
                .cmp(&Point::distance_squared(&station_point, p2))
                .reverse()
        })
    });

    // collect sorted points
    let mut laser_sorted_points: Vec<Point> = Vec::new();
    let mut removed_points = true;
    while removed_points {
        removed_points = false;
        for (_, ps_rev) in sorted_tuples.iter_mut() {
            match ps_rev.pop() {
                Some(p) => {
                    removed_points = true;
                    laser_sorted_points.push(p)
                }
                None => (),
            }
        }
    }

    laser_sorted_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![".#..#", ".....", "#####", "....#", "...##"].join("\n");

        let expected_output = (Point { x: 3, y: 4 }, 8);
        let actual_output = best_point(&string_to_points(&input));

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test2() {
        let input = vec![
            "......#.#.", // ......#.#.
            "#..#.#....", // #..#.#....
            "..#######.", // ..#######.
            ".#.#.###..", // .#.#.###..
            ".#..#.....", // .c..#.....
            "..#....#.#", // ..c....#.#
            "#..#....#.", // #..c....#.
            ".##.#..###", // .##.#..###
            "##...#..#.", // c#...#..#.
            ".#....####", // .#....####
        ]
        .join("\n");

        let expected_output = (Point { x: 5, y: 8 }, 33);
        let actual_output = best_point(&string_to_points(&input));

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn part1() {
        let input = read_to_string("./data/input.txt").unwrap();

        let expected_output = (Point { x: 20, y: 19 }, 284);
        let actual_output = best_point(&string_to_points(&input));

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test3() {
        let input = vec![
            ".#....#####...#..",
            "##...##.#####..##",
            "##...#...#.#####.",
            "..#.....#...###..",
            "..#.#.....#....##",
        ]
        .join("\n");

        let expected_first_point = Point { x: 8, y: 1 };
        let expected_second_point = Point { x: 9, y: 0 };
        let expected_third_point = Point { x: 9, y: 1 };
        let expected_fourth_point = Point { x: 10, y: 0 };
        let expected_fifth_point = Point { x: 9, y: 2 };
        let expected_ninth_point = Point { x: 15, y: 1 };

        let expected_tenth_point = Point { x: 12, y: 2 };
        let expected_eleventh_point = Point { x: 13, y: 2 };
        let expected_twelfth_point = Point { x: 14, y: 2 };
        let expected_thirteenth_point = Point { x: 15, y: 2 };

        let sorted_points = laser_sorted_points(&string_to_points(&input));

        assert_eq!(expected_first_point, sorted_points[0]);
        assert_eq!(expected_second_point, sorted_points[1]);
        assert_eq!(expected_third_point, sorted_points[2]);
        assert_eq!(expected_fourth_point, sorted_points[3]);
        assert_eq!(expected_fifth_point, sorted_points[4]);
        assert_eq!(expected_ninth_point, sorted_points[8]);

        assert_eq!(expected_tenth_point, sorted_points[9]);
        assert_eq!(expected_eleventh_point, sorted_points[10]);
        assert_eq!(expected_twelfth_point, sorted_points[11]);
        assert_eq!(expected_thirteenth_point, sorted_points[12]);
    }

    #[test]
    fn test4() {
        let input = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        let input = input.join("\n");
        let points = string_to_points(&input);

        let expected_1st = Point { x: 11, y: 12 };
        let expected_2nd = Point { x: 12, y: 1 };
        let expected_3rd = Point { x: 12, y: 2 };
        let expected_10th = Point { x: 12, y: 8 };
        let expected_20th = Point { x: 16, y: 0 };
        let expected_50th = Point { x: 16, y: 9 };
        let expected_100th = Point { x: 10, y: 16 };
        let expected_199th = Point { x: 9, y: 6 };
        let expected_200th = Point { x: 8, y: 2 };
        let expected_201st = Point { x: 10, y: 9 };
        let expected_299th = Point { x: 11, y: 1 };

        let (station_point, _) = best_point(&points);
        assert_eq!(station_point, Point { x: 11, y: 13 });

        let sorted_points = laser_sorted_points(&points);
        assert_eq!(expected_1st, sorted_points[0]);
        assert_eq!(expected_2nd, sorted_points[1]);
        assert_eq!(expected_3rd, sorted_points[2]);
        assert_eq!(expected_10th, sorted_points[9]);
        assert_eq!(expected_20th, sorted_points[19]);
        assert_eq!(expected_50th, sorted_points[49]);
        assert_eq!(expected_100th, sorted_points[99]);
        assert_eq!(expected_199th, sorted_points[198]);
        assert_eq!(expected_200th, sorted_points[199]);
        assert_eq!(expected_201st, sorted_points[200]);
        assert_eq!(expected_299th, sorted_points[298]);
    }

    #[test]
    fn part2() {
        let input = read_to_string("./data/input.txt").unwrap();
        let points = string_to_points(&input);
        let sorted_points = laser_sorted_points(&points);
        let point_200 = sorted_points[199].clone();

        let expected_result = 404;
        let actual_result = point_200.x * 100 + point_200.y;

        assert_eq!(expected_result, actual_result);
    }
}
