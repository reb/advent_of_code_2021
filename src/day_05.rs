/// --- Day 5: Hydrothermal Venture ---
///
/// You come across a field of hydrothermal vents on the ocean floor! These
/// vents constantly produce large, opaque clouds, so it would be best to avoid
/// them if possible.
///
/// They tend to form in lines; the submarine helpfully produces a list of
/// nearby lines of vents (your puzzle input) for you to review. For example:
///
/// 0,9 -> 5,9
/// 8,0 -> 0,8
/// 9,4 -> 3,4
/// 2,2 -> 2,1
/// 7,0 -> 7,4
/// 6,4 -> 2,0
/// 0,9 -> 2,9
/// 3,4 -> 1,4
/// 0,0 -> 8,8
/// 5,5 -> 8,2
///
/// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2
/// where x1,y1 are the coordinates of one end the line segment and x2,y2 are
/// the coordinates of the other end. These line segments include the points at
/// both ends. In other words:
///
///   - An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
///   - An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
///
/// For now, only consider horizontal and vertical lines: lines where either x1
/// = x2 or y1 = y2.
///
/// So, the horizontal and vertical lines from the above list would produce the
/// following diagram:
///
/// .......1..
/// ..1....1..
/// ..1....1..
/// .......1..
/// .112111211
/// ..........
/// ..........
/// ..........
/// ..........
/// 222111....
///
/// In this diagram, the top left corner is 0,0 and the bottom right corner is
/// 9,9. Each position is shown as the number of lines which cover that point or
/// . if no line covers that point. The top-left pair of 1s, for example, comes
/// from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9
/// -> 5,9 and 0,9 -> 2,9.
///
/// To avoid the most dangerous areas, you need to determine the number of
/// points where at least two lines overlap. In the above example, this is
/// anywhere in the diagram with a 2 or larger - a total of 5 points.
///
/// Consider only horizontal and vertical lines. At how many points do at least
/// two lines overlap?
use std::cmp::Ordering;

const INPUT: &str = include_str!("../input/day_05");

pub fn run() {
    println!("Not implemented yet");
    unimplemented!();
}

type Coordinates = (u32, u32);
struct HydrothermalVent {
    end_a: Coordinates,
    end_b: Coordinates,
}

impl HydrothermalVent {
    fn horizontal_or_vertical(&self) -> bool {
        let (x_a, y_a) = self.end_a;
        let (x_b, y_b) = self.end_b;

        x_a == x_b || y_a == y_b
    }

    fn iter(&self) -> HydrothermalVentIter {
        HydrothermalVentIter {
            current: Some(self.end_a),
            end: self.end_b,
        }
    }
}

struct HydrothermalVentIter {
    current: Option<Coordinates>,
    end: Coordinates,
}

impl Iterator for HydrothermalVentIter {
    type Item = Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.current;
        let (end_x, end_y) = self.end;
        self.current = self.current.and_then(|(x, y)| {
            if x == end_x && y == end_y {
                None
            } else {
                Some((
                    match x.cmp(&end_x) {
                        Ordering::Less => x + 1,
                        Ordering::Equal => x,
                        Ordering::Greater => x - 1,
                    },
                    match y.cmp(&end_y) {
                        Ordering::Less => y + 1,
                        Ordering::Equal => y,
                        Ordering::Greater => y - 1,
                    },
                ))
            }
        });

        last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydrothermal_vent_horizontal_or_vertical() {
        // 0,9 -> 5,9
        assert!(HydrothermalVent {
            end_a: (0, 9),
            end_b: (5, 9)
        }
        .horizontal_or_vertical());
        // 8,0 -> 0,8
        assert!(!HydrothermalVent {
            end_a: (8, 0),
            end_b: (0, 8)
        }
        .horizontal_or_vertical());
        // 9,4 -> 3,4
        assert!(HydrothermalVent {
            end_a: (9, 4),
            end_b: (3, 4)
        }
        .horizontal_or_vertical());
        // 2,2 -> 2,1
        assert!(HydrothermalVent {
            end_a: (2, 2),
            end_b: (2, 1)
        }
        .horizontal_or_vertical());
        // 7,0 -> 7,4
        assert!(HydrothermalVent {
            end_a: (7, 0),
            end_b: (7, 4)
        }
        .horizontal_or_vertical());
        // 6,4 -> 2,0
        assert!(!HydrothermalVent {
            end_a: (6, 4),
            end_b: (2, 0)
        }
        .horizontal_or_vertical());
        // 0,9 -> 2,9
        assert!(HydrothermalVent {
            end_a: (0, 9),
            end_b: (2, 9)
        }
        .horizontal_or_vertical());
        // 3,4 -> 1,4
        assert!(HydrothermalVent {
            end_a: (3, 4),
            end_b: (1, 4)
        }
        .horizontal_or_vertical());
        // 0,0 -> 8,8
        assert!(!HydrothermalVent {
            end_a: (0, 0),
            end_b: (8, 8)
        }
        .horizontal_or_vertical());
        // 5,5 -> 8,2
        assert!(!HydrothermalVent {
            end_a: (5, 5),
            end_b: (8, 2)
        }
        .horizontal_or_vertical());
    }

    #[test]
    fn test_hydrothermal_vent_iter_1() {
        // An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
        let vent = HydrothermalVent {
            end_a: (1, 1),
            end_b: (1, 3),
        };
        let mut vent_iter = vent.iter();

        assert_eq!(vent_iter.next(), Some((1, 1)));
        assert_eq!(vent_iter.next(), Some((1, 2)));
        assert_eq!(vent_iter.next(), Some((1, 3)));
        assert_eq!(vent_iter.next(), None);
    }

    #[test]
    fn test_hydrothermal_vent_iter_2() {
        // An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
        let vent = HydrothermalVent {
            end_a: (9, 7),
            end_b: (7, 7),
        };
        let mut vent_iter = vent.iter();

        assert_eq!(vent_iter.next(), Some((9, 7)));
        assert_eq!(vent_iter.next(), Some((8, 7)));
        assert_eq!(vent_iter.next(), Some((7, 7)));
        assert_eq!(vent_iter.next(), None);
    }
}
