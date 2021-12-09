/// --- Day 9: Smoke Basin ---
///
/// These caves seem to be lava tubes. Parts are even still volcanically active;
/// small hydrothermal vents release smoke into the caves that slowly settles
/// like rain.
///
/// If you can model how the smoke flows through the caves, you might be able to
/// avoid it and be that much safer. The submarine generates a heightmap of the
/// floor of the nearby caves for you (your puzzle input).
///
/// Smoke flows to the lowest point of the area it's in. For example, consider
/// the following heightmap:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
///
/// Each number corresponds to the height of a particular location, where 9 is
/// the highest and 0 is the lowest a location can be.
///
/// Your first goal is to find the low points - the locations that are lower
/// than any of its adjacent locations. Most locations have four adjacent
/// locations (up, down, left, and right); locations on the edge or corner of
/// the map have three or two adjacent locations, respectively. (Diagonal
/// locations do not count as adjacent.)
///
/// In the above example, there are four low points, all highlighted: two are in
/// the first row (a 1 and a 0), one is in the third row (a 5), and one is in
/// the bottom row (also a 5). All other locations on the heightmap have some
/// lower adjacent location, and so are not low points.
///
/// The risk level of a low point is 1 plus its height. In the above example,
/// the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk
/// levels of all low points in the heightmap is therefore 15.
///
/// Find all of the low points on your heightmap. What is the sum of the risk
/// levels of all low points on your heightmap?
///
/// --- Part Two ---
///
/// Next, you need to find the largest basins so you know what areas are most
/// important to avoid.
///
/// A basin is all locations that eventually flow downward to a single low
/// point. Therefore, every low point has a basin, although some basins are very
/// small. Locations of height 9 do not count as being in any basin, and all
/// other locations will always be part of exactly one basin.
///
/// The size of a basin is the number of locations within the basin, including
/// the low point. The example above has four basins.
///
/// The top-left basin, size 3:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
///
/// The top-right basin, size 9:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
///
/// The middle basin, size 14:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
///
/// The bottom-right basin, size 9:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
///
/// Find the three largest basins and multiply their sizes together. In the
/// above example, this is 9 * 14 * 9 = 1134.
///
/// What do you get if you multiply together the sizes of the three largest
/// basins?
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_09");

pub fn run() {
    let height_map = load_height_map(INPUT);

    let lowest_points = find_lowest_points(&height_map);
    let total_risk_level = lowest_points
        .iter()
        .map(|(_, p)| (p + 1) as u32)
        .sum::<u32>();

    println!(
        "The sum of the risk levels of all low points is: {}",
        total_risk_level
    );
}

fn find_lowest_points(height_map: &HeightMap) -> Vec<(Coordinates, Height)> {
    height_map
        .iter()
        .filter(|(&position, height)| {
            height_map
                .neighbours(position)
                .all(|neighbour_height| *height < neighbour_height)
        })
        .map(|(position, height)| (*position, *height))
        .collect()
}

type Coordinates = (i32, i32);
type Height = u8;
type HeightMap = HashMap<Coordinates, Height>;

trait HeightMapNeighbours<'a> {
    fn neighbours(&'a self, position: Coordinates) -> HeightMapNeighboursIter<'a>;
}

impl<'a> HeightMapNeighbours<'a> for HeightMap {
    fn neighbours(&self, position: Coordinates) -> HeightMapNeighboursIter {
        HeightMapNeighboursIter::new(self, position)
    }
}

struct HeightMapNeighboursIter<'a> {
    height_map: &'a HeightMap,
    iterator: Box<dyn Iterator<Item = Coordinates>>,
}
impl<'a> HeightMapNeighboursIter<'a> {
    fn new(height_map: &'a HeightMap, (x, y): Coordinates) -> HeightMapNeighboursIter<'a> {
        HeightMapNeighboursIter {
            height_map,
            iterator: { Box::new([(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)].into_iter()) },
        }
    }
}

impl<'a> Iterator for HeightMapNeighboursIter<'a> {
    type Item = &'a Height;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(position) = self.iterator.next() {
            let neighbour = self.height_map.get(&position);
            if neighbour.is_some() {
                return neighbour;
            }
        }
        None
    }
}

fn load_height_map(input: &str) -> HeightMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .enumerate()
                .map(move |(y, height)| ((x as i32, y as i32), height as u8))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn height_map_1() -> HeightMap {
        // 2199943210
        // 3987894921
        // 9856789892
        // 8767896789
        // 9899965678
        [
            // 2199943210
            ((0, 0), 2),
            ((0, 1), 1),
            ((0, 2), 9),
            ((0, 3), 9),
            ((0, 4), 9),
            ((0, 5), 4),
            ((0, 6), 3),
            ((0, 7), 2),
            ((0, 8), 1),
            ((0, 9), 0),
            // 3987894921
            ((1, 0), 3),
            ((1, 1), 9),
            ((1, 2), 8),
            ((1, 3), 7),
            ((1, 4), 8),
            ((1, 5), 9),
            ((1, 6), 4),
            ((1, 7), 9),
            ((1, 8), 2),
            ((1, 9), 1),
            // 9856789892
            ((2, 0), 9),
            ((2, 1), 8),
            ((2, 2), 5),
            ((2, 3), 6),
            ((2, 4), 7),
            ((2, 5), 8),
            ((2, 6), 9),
            ((2, 7), 8),
            ((2, 8), 9),
            ((2, 9), 2),
            // 8767896789
            ((3, 0), 8),
            ((3, 1), 7),
            ((3, 2), 6),
            ((3, 3), 7),
            ((3, 4), 8),
            ((3, 5), 9),
            ((3, 6), 6),
            ((3, 7), 7),
            ((3, 8), 8),
            ((3, 9), 9),
            // 9899965678
            ((4, 0), 9),
            ((4, 1), 8),
            ((4, 2), 9),
            ((4, 3), 9),
            ((4, 4), 9),
            ((4, 5), 6),
            ((4, 6), 5),
            ((4, 7), 6),
            ((4, 8), 7),
            ((4, 9), 8),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_load_height_map() {
        let input = "\
            2199943210\n\
            3987894921\n\
            9856789892\n\
            8767896789\n\
            9899965678\n";

        assert_eq!(load_height_map(input), height_map_1());
    }

    #[test]
    fn test_height_map_neighbours_iter() {
        let height_map = height_map_1();

        let mut neighbours = height_map.neighbours((1, 1));

        assert_eq!(neighbours.next(), Some(&1));
        assert_eq!(neighbours.next(), Some(&8));
        assert_eq!(neighbours.next(), Some(&8));
        assert_eq!(neighbours.next(), Some(&3));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_height_map_neighbours_iter_corner() {
        let height_map = height_map_1();

        let mut neighbours = height_map.neighbours((4, 0));

        assert_eq!(neighbours.next(), Some(&8));
        assert_eq!(neighbours.next(), Some(&8));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_find_lowest_points() {
        let height_map = height_map_1();

        let found_lowest_points = lowest_points(&height_map);

        assert_eq!(found_lowest_points.len(), 4);
        assert!(found_lowest_points.contains(&((0, 1), 1)));
        assert!(found_lowest_points.contains(&((0, 9), 0)));
        assert!(found_lowest_points.contains(&((2, 2), 5)));
        assert!(found_lowest_points.contains(&((4, 6), 5)));
    }
}
