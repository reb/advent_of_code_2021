/// --- Day 4: Giant Squid ---
///
/// You're already almost 1.5km (almost a mile) below the surface of the ocean,
/// already so deep that you can't see any sunlight. What you can see, however,
/// is a giant squid that has attached itself to the outside of your submarine.
///
/// Maybe it wants to play bingo?
///
/// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
/// Numbers are chosen at random, and the chosen number is marked on all boards
/// on which it appears. (Numbers may not appear on all boards.) If all numbers
/// in any row or any column of a board are marked, that board wins. (Diagonals
/// don't count.)
///
/// The submarine has a bingo subsystem to help passengers (currently, you and
/// the giant squid) pass the time. It automatically generates a random order in
/// which to draw numbers and a random set of boards (your puzzle input). For
/// example:
///
/// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
///
/// 22 13 17 11  0
///  8  2 23  4 24
/// 21  9 14 16  7
///  6 10  3 18  5
///  1 12 20 15 19
///
///  3 15  0  2 22
///  9 18 13 17  5
/// 19  8  7 25 23
/// 20 11 10 24  4
/// 14 21 16 12  6
///
/// 14 21 17 24  4
/// 10 16 15  9 19
/// 18  8 23 26 20
/// 22 11 13  6  5
///  2  0 12  3  7
///
/// After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no
/// winners, but the boards are marked as follows (shown here adjacent to each
/// other to save space):
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
///
/// After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are
/// still no winners:
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
///
/// Finally, 24 is drawn:
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
///
/// At this point, the third board wins because it has at least one complete row
/// or column of marked numbers (in this case, the entire top row is marked: 14
/// 21 17 24 4).
///
/// The score of the winning board can now be calculated. Start by finding the
/// sum of all unmarked numbers on that board; in this case, the sum is 188.
/// Then, multiply that sum by the number that was just called when the board
/// won, 24, to get the final score, 188 * 24 = 4512.
///
/// To guarantee victory against the giant squid, figure out which board will
/// win first. What will your final score be if you choose that board?
///
/// --- Part Two ---
///
/// On the other hand, it might be wise to try a different strategy: let the
/// giant squid win.
///
/// You aren't sure how many bingo boards a giant squid could play at once, so
/// rather than waste time counting its arms, the safe thing to do is to figure
/// out which board will win last and choose that one. That way, no matter which
/// boards it picks, it will win for sure.
///
/// In the above example, the second board is the last to win, which happens
/// after 13 is eventually called and its middle column is completely marked. If
/// you were to keep playing until this point, the second board would have a sum
/// of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.
///
/// Figure out which board will win last. Once it wins, what would its final
/// score be?
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_04");

pub fn run() {
    let (numbers, mut boards) = load_bingo_game(INPUT);

    let mut first_found = false;
    for number in numbers.iter() {
        let last_one = boards.len() == 1;
        boards = boards
            .into_iter()
            .filter_map(|mut board| {
                board.mark(number);
                if board.bingo() {
                    if !first_found {
                        first_found = true;
                        println!(
                            "The first board that will win has a final score of: {}",
                            board.score() * number
                        );
                    }
                    if last_one {
                        println!(
                            "The last board that will win has a final score of: {}",
                            board.score() * number
                        );
                    }
                    None
                } else {
                    Some(board)
                }
            })
            .collect();
    }
}

type BingoNumber = u32;
type Coordinates = (u8, u8);

#[derive(Debug, PartialEq)]
struct BingoBoard {
    numbers: HashMap<Coordinates, BingoNumber>,
    marked: HashMap<Coordinates, bool>,
}

impl BingoBoard {
    fn new(numbers: HashMap<Coordinates, BingoNumber>) -> BingoBoard {
        BingoBoard {
            numbers,
            marked: HashMap::new(),
        }
    }

    fn mark(&mut self, number: &BingoNumber) {
        for (position, _) in self.numbers.iter().filter(|(_, n)| n == &number) {
            self.marked.insert(*position, true);
        }
    }

    fn bingo(&self) -> bool {
        // check rows
        let mut row = 0;
        while self.numbers.get(&(row, 0)).is_some() {
            if self
                .numbers
                .keys()
                .filter(|(x, _)| x == &row)
                .all(|pos| *self.marked.get(pos).unwrap_or(&false))
            {
                return true;
            }
            row += 1;
        }

        // check columns
        let mut column = 0;
        while self.numbers.get(&(0, column)).is_some() {
            if self
                .numbers
                .keys()
                .filter(|(_, y)| y == &column)
                .all(|pos| *self.marked.get(pos).unwrap_or(&false))
            {
                return true;
            }
            column += 1;
        }

        return false;
    }

    fn score(&self) -> BingoNumber {
        self.numbers
            .iter()
            .filter(|(pos, _)| !*self.marked.get(pos).unwrap_or(&false))
            .map(|(_, number)| number)
            .sum()
    }
}

fn load_bingo_game(input: &str) -> (Vec<BingoNumber>, Vec<BingoBoard>) {
    let mut blocks = input.split("\n\n");

    let numbers = blocks
        .next()
        .unwrap()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();
    let boards = blocks.map(parse_board).collect();

    (numbers, boards)
}

fn parse_board(board: &str) -> BingoBoard {
    BingoBoard::new(
        board
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .enumerate()
                    .map(|(y, n)| ((x as u8, y as u8), n))
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bingo_board_1() -> BingoBoard {
        // 22 13 17 11  0
        //  8  2 23  4 24
        // 21  9 14 16  7
        //  6 10  3 18  5
        //  1 12 20 15 19
        BingoBoard::new(
            [
                ((0, 0), 22),
                ((0, 1), 13),
                ((0, 2), 17),
                ((0, 3), 11),
                ((0, 4), 0),
                ((1, 0), 8),
                ((1, 1), 2),
                ((1, 2), 23),
                ((1, 3), 4),
                ((1, 4), 24),
                ((2, 0), 21),
                ((2, 1), 9),
                ((2, 2), 14),
                ((2, 3), 16),
                ((2, 4), 7),
                ((3, 0), 6),
                ((3, 1), 10),
                ((3, 2), 3),
                ((3, 3), 18),
                ((3, 4), 5),
                ((4, 0), 1),
                ((4, 1), 12),
                ((4, 2), 20),
                ((4, 3), 15),
                ((4, 4), 19),
            ]
            .iter()
            .cloned()
            .collect(),
        )
    }

    fn bingo_board_2() -> BingoBoard {
        //  3 15  0  2 22
        //  9 18 13 17  5
        // 19  8  7 25 23
        // 20 11 10 24  4
        // 14 21 16 12  6
        BingoBoard::new(
            [
                ((0, 0), 3),
                ((0, 1), 15),
                ((0, 2), 0),
                ((0, 3), 2),
                ((0, 4), 22),
                ((1, 0), 9),
                ((1, 1), 18),
                ((1, 2), 13),
                ((1, 3), 17),
                ((1, 4), 5),
                ((2, 0), 19),
                ((2, 1), 8),
                ((2, 2), 7),
                ((2, 3), 25),
                ((2, 4), 23),
                ((3, 0), 20),
                ((3, 1), 11),
                ((3, 2), 10),
                ((3, 3), 24),
                ((3, 4), 4),
                ((4, 0), 14),
                ((4, 1), 21),
                ((4, 2), 16),
                ((4, 3), 12),
                ((4, 4), 6),
            ]
            .iter()
            .cloned()
            .collect(),
        )
    }

    fn bingo_board_3() -> BingoBoard {
        // 14 21 17 24  4
        // 10 16 15  9 19
        // 18  8 23 26 20
        // 22 11 13  6  5
        //  2  0 12  3  7
        BingoBoard::new(
            [
                ((0, 0), 14),
                ((0, 1), 21),
                ((0, 2), 17),
                ((0, 3), 24),
                ((0, 4), 4),
                ((1, 0), 10),
                ((1, 1), 16),
                ((1, 2), 15),
                ((1, 3), 9),
                ((1, 4), 19),
                ((2, 0), 18),
                ((2, 1), 8),
                ((2, 2), 23),
                ((2, 3), 26),
                ((2, 4), 20),
                ((3, 0), 22),
                ((3, 1), 11),
                ((3, 2), 13),
                ((3, 3), 6),
                ((3, 4), 5),
                ((4, 0), 2),
                ((4, 1), 0),
                ((4, 2), 12),
                ((4, 3), 3),
                ((4, 4), 7),
            ]
            .iter()
            .cloned()
            .collect(),
        )
    }

    #[test]
    fn test_load_bingo_game() {
        let input = "\
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
        \n\
        22 13 17 11  0\n\
         8  2 23  4 24\n\
        21  9 14 16  7\n\
         6 10  3 18  5\n\
         1 12 20 15 19\n\
        \n\
         3 15  0  2 22\n\
         9 18 13 17  5\n\
        19  8  7 25 23\n\
        20 11 10 24  4\n\
        14 21 16 12  6\n\
        \n\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7\n";

        let expected_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let expected_boards = vec![bingo_board_1(), bingo_board_2(), bingo_board_3()];

        assert_eq!(load_bingo_game(input), (expected_numbers, expected_boards));
    }

    #[test]
    fn test_bingo_board_mark() {
        let mut bingo_board = bingo_board_1();

        let mut expected_marked = HashMap::new();
        expected_marked.insert((1, 2), true);

        bingo_board.mark(&23);

        assert_eq!(bingo_board.marked, expected_marked);
    }

    #[test]
    fn test_bingo_board_bingo() {
        let mut bingo_board_1 = bingo_board_1();
        let mut bingo_board_2 = bingo_board_2();
        let mut bingo_board_3 = bingo_board_3();

        for n in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21].iter() {
            // none of these should be bingo
            bingo_board_1.mark(n);
            assert!(!bingo_board_1.bingo());
            bingo_board_2.mark(n);
            assert!(!bingo_board_2.bingo());
            bingo_board_3.mark(n);
            assert!(!bingo_board_3.bingo());
        }

        let n = &24;
        bingo_board_1.mark(n);
        assert!(!bingo_board_1.bingo());
        bingo_board_2.mark(n);
        assert!(!bingo_board_2.bingo());
        // finally bingo
        bingo_board_3.mark(n);
        println!("{:?}", bingo_board_3.marked);
        println!("{:?}", bingo_board_3.numbers);
        assert!(bingo_board_3.bingo());
    }

    #[test]
    fn test_bingo_board_score() {
        let mut bingo_board = bingo_board_3();

        // prepare the board
        for n in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24].iter() {
            bingo_board.mark(n);
        }

        assert_eq!(bingo_board.score(), 188);
    }
}
