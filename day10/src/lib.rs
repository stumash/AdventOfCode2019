mod grid_parse;
mod point;

use grid_parse::string_to_points;
use num_rational::Ratio;
use point::*;
use std::collections::HashMap;
use std::fs::read_to_string;

fn best_point(points: &Vec<Point>) -> (Point, usize) {
    let mut current_best = (Point { x: -1, y: -1 }, 0);
    for p1 in points {
        let mut slopes: HashMap<DirectedSlope, Point> = HashMap::new();

        for p2 in points.iter().filter(|p2| *p2 != p1) {
            slopes.insert(p1.slope(p2), p2.clone());
        }
        if slopes.len() > current_best.1 {
            current_best = (p1.clone(), slopes.len());
        }
    }
    current_best
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
    fn it_works() {
        let input = read_to_string("./data/input.txt").unwrap();

        let expected_output = (Point { x: 20, y: 19 }, 284);
        let actual_output = best_point(&string_to_points(&input));

        assert_eq!(expected_output, actual_output);
    }
}
