use std::io::{self, BufRead};
use std::cmp::{min, max};
use std::ops::{Add};

fn main() {
    let stdin = io::stdin();
    let paths: Vec<Path> = stdin.lock().lines()
        .map(|s| s.unwrap())
        .map(|s| Path::from(s))
        .collect();
    let (path1, path2) = (paths.get(0).unwrap(), paths.get(1).unwrap());

    let (mut best_point, mut best_distance): (Option<Point>, Option<i32>) = (None, None);
    let (mut distance1, mut distance2) = (0, 0);
    for ls1 in path1.line_segments.iter() {
        distance2 = 0;
        for ls2 in path2.line_segments.iter() {
            if let Some(Point{ x, y }) = ls1.intersection(ls2) {
                if x == 0 && y == 0 { continue; }
                let new_distance = distance1 + distance2 +
                    ls1.length_along(&Point{x, y}).unwrap() + ls2.length_along(&Point{x, y}).unwrap();
                match (&best_point, &best_distance) {
                    (None, None) => { best_point = Some(Point{ x, y }); best_distance = Some(new_distance); }
                    (Some(Point{ x:_, y:_ }), Some(d)) => {
                        if new_distance < *d {
                            best_point = Some(Point{ x, y });
                            best_distance = Some(new_distance);
                        }
                    }
                    (_, _) => panic!("impossible")
                }
            }
            distance2 += ls2.size();
        }
        distance1 += ls1.size();
    }

    println!(
        "{:?}, sum manhattan: {:?}",
        best_point,
        best_distance
    );
}

#[derive(Clone)]
enum Direction {
    Right, Left, Up, Down
}
#[derive(Clone)]
struct PathSegment {
    direction: Direction,
    size: u16
}
impl From<&str> for PathSegment {
    fn from(s: &str) -> PathSegment {
        PathSegment{
            direction: match &s[0..1] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("oh no")
            },
            size: (&s[1..]).parse().unwrap()
        }
    }
}

#[derive(Debug, Clone)]
struct Point { x: i32, y: i32 }
impl From<PathSegment> for Point {
    fn from(path_seg: PathSegment) -> Point {
        match path_seg.direction {
            Direction::Left => Point{ x: -(path_seg.size as i32), y: 0 },
            Direction::Right => Point{ x: (path_seg.size as i32), y: 0 },
            Direction::Up => Point{ x:0, y: (path_seg.size as i32) },
            Direction::Down => Point{ x:0, y: -(path_seg.size as i32) } 
        }
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point{ x: self.x+other.x, y: self.y+other.y }
    }
}
impl Point {
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
enum LRstart { Left, Right }
#[derive(Debug)]
enum UDstart { Up, Down }
#[derive(Debug)]
enum LineSegment {
    LeftRight{y: i32, x_small: i32, x_big: i32, start: LRstart},
    UpDown{x: i32, y_small: i32, y_big: i32, start: UDstart}
}
impl LineSegment {
    fn new(p1: Point, path_segment: PathSegment) -> LineSegment {
        use Direction::*;
        use LineSegment::*;
        let p2: Point = match path_segment.direction {
            Right => Point{ x: p1.x + (path_segment.size as i32), y: p1.y },
            Left => Point{ x: p1.x - (path_segment.size as i32), y: p1.y },
            Up => Point{ x: p1.x, y: p1.y + (path_segment.size as i32) },
            Down => Point{ x: p1.x, y: p1.y - (path_segment.size as i32) }
        };
        match (p1.x == p2.x, p1.y == p2.y) {
            (true, false) => UpDown{
                x: p1.x, y_small: min(p1.y, p2.y), y_big: max(p1.y, p2.y),
                start: if p1.y < p2.y { UDstart::Down } else { UDstart::Up }
            },
            (false, true) => LeftRight{
                y: p1.y, x_small: min(p1.x, p2.x), x_big: max(p1.x, p2.x),
                start: if p1.x < p2.x { LRstart::Left } else { LRstart::Right }
            },
            (true, true) => panic!("points are the same"),
            (false, false) => panic!("points don't share an x or y")
        }
    }
    fn size(&self) -> i32 {
        use LineSegment::*;
        match self {
            LeftRight{ y:_, x_small, x_big, start:_ } => x_big - x_small,
            UpDown{ x:_, y_small, y_big, start:_ } => y_big - y_small

        }
    }
    fn intersection(&self, other: &LineSegment) -> Option<Point> {
        use LineSegment::*;
        match (self, other) {
            (LeftRight{ y:y1, x_small:xs1, x_big:xb1, start:_  }, LeftRight{ y:y2, x_small:xs2, x_big:xb2, start:_}) =>
                if y1 != y2 { None } else {
                    if min(xb1,xb2) < max(xs1,xs2) { None } else {
                        if min(xb1,xb2).abs() < max(xs1,xs2).abs() {
                            Some(Point{ y:*y1, x:min(*xb1,*xb2) })
                        } else {
                            Some(Point{ y:*y1, x:max(*xs1,*xs2) })
                        }
                    }
                },
            (UpDown{ x:x1, y_small:ys1, y_big:yb1, start:_ }, UpDown{ x:x2, y_small:ys2, y_big:yb2, start:_ }) =>
                if x1 != x2 { None } else {
                    if min(yb1,yb2) < max(ys1,ys2) { None } else {
                        if min(yb1,yb2).abs() < max(ys1,ys2).abs() {
                            Some(Point{ x:*x1, y:min(*yb1,*yb2) })
                        } else {
                            Some(Point{ x:*x1, y:max(*ys1,*ys2) })
                        }
                    }
                },
            (LeftRight{ y, x_small, x_big, start:_ }, UpDown{ x, y_small, y_big, start:_ }) =>
                if x < x_small || x > x_big || y < y_small || y > y_big { None } else {
                    Some(Point{ x:*x, y:*y })
                }
            (UpDown{ x, y_small, y_big, start:_ }, LeftRight{ y, x_small, x_big, start:_ }) =>
                if x < x_small || x > x_big || y < y_small || y > y_big { None } else {
                    Some(Point{ x:*x, y:*y })
                }
        }
    }
    fn length_along(&self, p: &Point) -> Option<i32> {
        use LineSegment::*;
        match self {
            LeftRight{y, x_small, x_big, start} => {
                use LRstart::*;
                if p.y != *y || p.x < *x_small || p.x > *x_big { None }
                else { match start { Left => Some(p.x - x_small), Right => Some(x_big - p.x) } }
            },
            UpDown{x, y_small, y_big, start} => {
                use UDstart::*;
                if p.x != *x || p.y < *y_small || p.y > *y_big { None }
                else { match start { Down => Some(p.y - y_small), Up => Some(y_big - p.y) } }
            }
        }
    }
}

#[derive(Debug)]
struct Path{
    line_segments: Vec<LineSegment>
}
impl From<&str> for Path {
    fn from(s: &str) -> Path {
        Path{ line_segments: s
            .split(",")
            .map(|s| PathSegment::from(s))
            .fold((Point{x:0, y:0}, Vec::new()), |(xy, mut line_segs), path_seg| {
                 let line_seg = LineSegment::new(xy.clone(), path_seg.clone());
                 line_segs.push(line_seg);
                 (xy + Point::from(path_seg), line_segs)
            }).1
        }
    }
}
impl From<String> for Path {
    fn from(s: String) -> Path {
        Path::from(s.as_str())
    }
}
