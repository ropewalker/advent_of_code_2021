use crate::day05::SegmentOrientation::*;
use crate::day05::TriangleOrientation::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering::*;
use std::collections::HashSet;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(point_str: &str) -> Self {
        let mut iter = point_str.split(',');

        Self {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum SegmentOrientation {
    Horizontal = 0,
    Vertical = 1,
    Descending = 2,
    Ascending = 3,
}

#[derive(Clone, Debug, PartialEq)]
struct Segment {
    start: Point,
    end: Point,
    orientation: SegmentOrientation,
}

impl From<&str> for Segment {
    fn from(segment_str: &str) -> Self {
        let mut iter = segment_str.split(" -> ");

        let start: Point = iter.next().unwrap().into();
        let end: Point = iter.next().unwrap().into();

        let orientation = Self::calculate_orientation(&start, &end);

        Self {
            start,
            end,
            orientation,
        }
    }
}

impl Segment {
    fn calculate_orientation(start: &Point, end: &Point) -> SegmentOrientation {
        if start.y == end.y {
            Horizontal
        } else if start.x == end.x {
            Vertical
        } else if end.y - start.y == end.x - start.x {
            Descending
        } else {
            Ascending
        }
    }

    fn into_points(self) -> Vec<Point> {
        let mut points = Vec::new();

        let x_increment = -(self.start.x.cmp(&self.end.x) as i32);
        let y_increment = -(self.start.y.cmp(&self.end.y) as i32);

        let mut point = Point {
            x: self.start.x,
            y: self.start.y,
        };

        loop {
            points.push(point.to_owned());

            if point == self.end {
                break;
            }

            point = Point {
                x: point.x + x_increment,
                y: point.y + y_increment,
            };
        }

        points
    }
}

fn point_inside_box(point: &Point, box_diagonal: &Segment) -> bool {
    point.x <= box_diagonal.start.x.max(box_diagonal.end.x)
        && point.x >= box_diagonal.start.x.min(box_diagonal.end.x)
        && point.y <= box_diagonal.start.y.max(box_diagonal.end.y)
        && point.y >= box_diagonal.start.y.min(box_diagonal.end.y)
}

#[derive(Clone, Debug, PartialEq)]
enum TriangleOrientation {
    Clockwise,
    Counterclockwise,
    Collinear,
}

fn triangle_orientation(segment: &Segment, point: &Point) -> TriangleOrientation {
    match ((segment.end.y - segment.start.y) * (point.x - segment.end.x))
        .cmp(&((segment.end.x - segment.start.x) * (point.y - segment.end.y)))
    {
        Greater => Counterclockwise,
        Less => Clockwise,
        Equal => Collinear,
    }
}

fn intersect_non_collinear(segment0: &Segment, segment1: &Segment) -> Option<Point> {
    let (segment0, segment1) = if segment0.orientation < segment1.orientation {
        (segment0, segment1)
    } else {
        (segment1, segment0)
    };

    match (&segment0.orientation, &segment1.orientation) {
        (Horizontal, Vertical) => Some(Point {
            x: segment1.start.x,
            y: segment0.start.y,
        }),
        (Horizontal, Descending) => Some(Point {
            x: segment1.start.x + (segment0.start.y - segment1.start.y),
            y: segment0.start.y,
        }),
        (Horizontal, Ascending) => Some(Point {
            x: segment1.start.x - (segment0.start.y - segment1.start.y),
            y: segment0.start.y,
        }),
        (Vertical, Descending) => Some(Point {
            x: segment0.start.x,
            y: segment1.start.y + (segment0.start.x - segment1.start.x),
        }),
        (Vertical, Ascending) => Some(Point {
            x: segment0.start.x,
            y: segment1.start.y - (segment0.start.x - segment1.start.x),
        }),
        (Descending, Ascending) => {
            let double_x =
                segment1.start.y - segment0.start.y + segment1.start.x + segment0.start.x;
            let double_y =
                segment1.start.x - segment0.start.x + segment1.start.y + segment0.start.y;

            if double_x % 2 == 0 {
                Some(Point {
                    x: double_x / 2,
                    y: double_y / 2,
                })
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn intersect(segment0: &Segment, segment1: &Segment) -> HashSet<Point> {
    let mut overlap_points = HashSet::new();

    let segment1_start_to_segment0 = triangle_orientation(segment0, &segment1.start);
    let segment1_end_to_segment0 = triangle_orientation(segment0, &segment1.end);
    let segment0_start_to_segment1 = triangle_orientation(segment1, &segment0.start);
    let segment0_end_to_segment1 = triangle_orientation(segment1, &segment0.end);

    if segment1_start_to_segment0 != segment1_end_to_segment0
        && segment0_start_to_segment1 != segment0_end_to_segment1
    {
        if let Some(point) = intersect_non_collinear(segment0, segment1) {
            overlap_points.insert(point);
            return overlap_points;
        }
    }

    if segment1_start_to_segment0 == Collinear && point_inside_box(&segment1.start, segment0) {
        overlap_points.insert(segment1.start.to_owned());
    }

    if segment1_end_to_segment0 == Collinear && point_inside_box(&segment1.end, segment0) {
        overlap_points.insert(segment1.end.to_owned());
    }

    if segment0_start_to_segment1 == Collinear && point_inside_box(&segment0.start, segment1) {
        overlap_points.insert(segment0.start.to_owned());
    }

    if segment0_end_to_segment1 == Collinear && point_inside_box(&segment0.end, segment1) {
        overlap_points.insert(segment0.end.to_owned());
    }

    if overlap_points.len() > 1 {
        let mut iter = overlap_points.iter();
        let start = iter.next().unwrap().to_owned();
        let end = iter.next().unwrap().to_owned();

        let orientation = Segment::calculate_orientation(&start, &end);

        overlap_points.extend(
            Segment {
                start,
                end,
                orientation,
            }
            .into_points(),
        )
    }

    overlap_points
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Segment> {
    input.lines().map(|l| l.into()).collect()
}

#[aoc(day5, part1)]
fn part1(segments: &[Segment]) -> usize {
    let mut overlap_points = HashSet::new();

    let segments: Vec<&Segment> = segments
        .iter()
        .filter(|&s| s.orientation == Horizontal || s.orientation == Vertical)
        .collect();

    for (i, segment0) in segments.iter().enumerate() {
        for segment1 in segments.iter().skip(i + 1) {
            overlap_points.extend(intersect(segment0, segment1));
        }
    }

    overlap_points.len()
}

#[aoc(day5, part2)]
fn part2(segments: &[Segment]) -> usize {
    let mut overlap_points = HashSet::new();

    for (i, segment0) in segments.iter().enumerate() {
        for segment1 in segments.iter().skip(i + 1) {
            overlap_points.extend(intersect(segment0, segment1));
        }
    }

    overlap_points.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 12);
    }
}
