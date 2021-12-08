/// --- Day 8: Seven Segment Search ---
///
/// You barely reach the safety of the cave when the whale smashes into the cave
/// mouth, collapsing it. Sensors indicate another exit to this cave at a much
/// greater depth, so you have no choice but to press on.
///
/// As your submarine slowly makes its way through the cave system, you notice
/// that the four-digit seven-segment displays in your submarine are
/// malfunctioning; they must have been damaged during the escape. You'll be in
/// a lot of trouble without them, so you'd better figure out what's wrong.
///
/// Each digit of a seven-segment display is rendered by turning on or off any
/// of seven segments named a through g:
///
///   0:      1:      2:      3:      4:
///  aaaa    ....    aaaa    aaaa    ....
/// b    c  .    c  .    c  .    c  b    c
/// b    c  .    c  .    c  .    c  b    c
///  ....    ....    dddd    dddd    dddd
/// e    f  .    f  e    .  .    f  .    f
/// e    f  .    f  e    .  .    f  .    f
///  gggg    ....    gggg    gggg    ....
///
///   5:      6:      7:      8:      9:
///  aaaa    aaaa    aaaa    aaaa    aaaa
/// b    .  b    .  .    c  b    c  b    c
/// b    .  b    .  .    c  b    c  b    c
///  dddd    dddd    ....    dddd    dddd
/// .    f  e    f  .    f  e    f  .    f
/// .    f  e    f  .    f  e    f  .    f
///  gggg    gggg    ....    gggg    gggg
///
/// So, to render a 1, only segments c and f would be turned on; the rest would
/// be off. To render a 7, only segments a, c, and f would be turned on.
///
/// The problem is that the signals which control the segments have been mixed
/// up on each display. The submarine is still trying to display numbers by
/// producing output on signal wires a through g, but those wires are connected
/// to segments randomly. Worse, the wire/segment connections are mixed up
/// separately for each four-digit display! (All of the digits within a display
/// use the same connections, though.)
///
/// So, you might know that only signal wires b and g are turned on, but that
/// doesn't mean segments b and g are turned on: the only digit that uses two
/// segments is 1, so it must mean segments c and f are meant to be on. With
/// just that information, you still can't tell which wire (b/g) goes to which
/// segment (c/f). For that, you'll need to collect more information.
///
/// For each display, you watch the changing signals for a while, make a note of
/// all ten unique signal patterns you see, and then write down a single four
/// digit output value (your puzzle input). Using the signal patterns, you
/// should be able to work out which pattern corresponds to which digit.
///
/// For example, here is what you might see in a single entry in your notes:
///
/// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
/// cdfeb fcadb cdfeb cdbaf
///
/// (The entry is wrapped here to two lines so it fits; in your notes, it will
/// all be on a single line.)
///
/// Each entry consists of ten unique signal patterns, a | delimiter, and
/// finally the four digit output value. Within an entry, the same wire/segment
/// connections are used (but you don't know what the connections actually are).
/// The unique signal patterns correspond to the ten different ways the
/// submarine tries to render a digit using the current wire/segment
/// connections. Because 7 is the only digit that uses three segments, dab in
/// the above example means that to render a 7, signal lines d, a, and b are on.
/// Because 4 is the only digit that uses four segments, eafb means that to
/// render a 4, signal lines e, a, f, and b are on.
///
/// Using this information, you should be able to work out which combination of
/// signal wires corresponds to each of the ten digits. Then, you can decode the
/// four digit output value. Unfortunately, in the above example, all of the
/// digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and
/// are more difficult to deduce.
///
/// For now, focus on the easy digits. Consider this larger example:
///
/// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
/// fdgacbe cefdb cefbgd gcbe
/// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
/// fcgedb cgb dgebacf gc
/// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
/// cg cg fdcagb cbg
/// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
/// efabcd cedba gadfec cb
/// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
/// gecf egdcabf bgf bfgea
/// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
/// gebdcfa ecba ca fadegcb
/// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
/// cefg dcbef fcge gbcadfe
/// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
/// ed bcgafe cdgba cbgef
/// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
/// gbdfcae bgc cg cgb
/// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
/// fgae cfgab fg bagce
///
/// Because the digits 1, 4, 7, and 8 each use a unique number of segments, you
/// should be able to tell which combinations of signals correspond to those
/// digits. Counting only digits in the output values (the part after | on each
/// line), in the above example, there are 26 instances of digits that use a
/// unique number of segments (highlighted above).
///
/// In the output values, how many times do digits 1, 4, 7, or 8 appear?
///
/// --- Part Two ---
///
/// Through a little deduction, you should now be able to determine the
/// remaining digits. Consider again the first example above:
///
/// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
/// cdfeb fcadb cdfeb cdbaf
///
/// After some careful analysis, the mapping between signal wires and segments
/// only make sense in the following configuration:
///
///  dddd
/// e    a
/// e    a
///  ffff
/// g    b
/// g    b
///  cccc
///
/// So, the unique signal patterns would correspond to the following digits:
///
///   - acedgfb: 8
///   - cdfbe: 5
///   - gcdfa: 2
///   - fbcad: 3
///   - dab: 7
///   - cefabd: 9
///   - cdfgeb: 6
///   - eafb: 4
///   - cagedb: 0
///   - ab: 1
///
/// Then, the four digits of the output value can be decoded:
///
///   - cdfeb: 5
///   - fcadb: 3
///   - cdfeb: 5
///   - cdbaf: 3
///
/// Therefore, the output value for this entry is 5353.
///
/// Following this same process for each entry in the second, larger example
/// above, the output value of each entry can be determined:
///
///   - fdgacbe cefdb cefbgd gcbe: 8394
///   - fcgedb cgb dgebacf gc: 9781
///   - cg cg fdcagb cbg: 1197
///   - efabcd cedba gadfec cb: 9361
///   - gecf egdcabf bgf bfgea: 4873
///   - gebdcfa ecba ca fadegcb: 8418
///   - cefg dcbef fcge gbcadfe: 4548
///   - ed bcgafe cdgba cbgef: 1625
///   - gbdfcae bgc cg cgb: 8717
///   - fgae cfgab fg bagce: 4315
///
/// Adding all of the output values in this larger example produces 61229.
///
/// For each entry, determine all of the wire/segment connections and decode the
/// four-digit output values. What do you get if you add up all of the output
/// values?
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_08");

pub fn run() {
    let segment_displays = load_segment_displays(INPUT);

    println!(
        "Digits 1, 4, 7 or 8 appears {} times in the output values",
        count_occurance(&segment_displays, vec![1, 4, 7, 8])
    );

    let total_sum = segment_displays
        .iter()
        .map(|(examples, displays)| {
            displays.iter().fold(0, |number, display| {
                let digit = identify_digit(&examples, display).expect("Could not identify a digit");
                number * 10 + digit
            })
        })
        .sum::<usize>();

    println!("Adding up all the output values gives: {}", total_sum);
}

fn count_occurance(
    segment_displays: &Vec<(SegmentDisplays, SegmentDisplays)>,
    digits_to_count: Vec<usize>,
) -> usize {
    segment_displays
        .iter()
        .map(|(examples, displays)| {
            displays
                .iter()
                .filter_map(|display| identify_digit(&examples, display))
                .filter(|identified_digit| digits_to_count.contains(identified_digit))
                .count()
        })
        .sum()
}

type SegmentDisplay<'a> = &'a str;
type SegmentDisplays<'a> = Vec<SegmentDisplay<'a>>;

fn identify_digit<'a>(
    examples: &SegmentDisplays<'a>,
    display: &SegmentDisplay<'a>,
) -> Option<usize> {
    match display.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        length => {
            // this is harder to identify
            let identified_digits: HashMap<_, _> = examples
                .iter()
                .map(|display| (identify_digit(&vec![], display), display))
                .collect();

            // look if digit 1 completely overlaps
            identified_digits.get(&Some(1)).and_then(|digit_1| {
                match (
                    digit_1.chars().all(|section| display.contains(section)),
                    length,
                ) {
                    (true, 5) => Some(3), // with a length 5 and overlap with digit 1 it's a 3
                    (true, 6) => {
                        // digit 1 overlaps and 6 sections are on, it's a 0 or 9
                        identified_digits.get(&Some(4)).and_then(|digit_4| {
                            if digit_4.chars().all(|section| display.contains(section)) {
                                // if digit 4 fully overlaps, it's a 9
                                Some(9)
                            } else {
                                // if digit 4 doesn't fully overlap, it's a 0
                                Some(0)
                            }
                        })
                    }
                    (false, 5) => {
                        // digit 1 doesn't overlaps and 5 sections are on, it's a 2 or 5
                        identified_digits.get(&Some(4)).and_then(|digit_4| {
                            match digit_4
                                .chars()
                                .filter(|&section| display.contains(section))
                                .count()
                            {
                                2 => Some(2), // 2 sections of digit 4 overlap, it's a 2
                                3 => Some(5), // 3 sections of digit 4 overlap, it's a 5
                                _ => None,
                            }
                        })
                    }
                    (false, 6) => Some(6), // with a length 6 and no overlap of digit 1, it's a 6
                    _ => None,
                }
            })
        }
    }
}

fn load_segment_displays(input: &str) -> Vec<(SegmentDisplays, SegmentDisplays)> {
    input
        .lines()
        .filter_map(|line| {
            line.split('|')
                .tuples()
                .next()
                .map(|(examples, display): (&str, &str)| {
                    (
                        examples.split_whitespace().collect(),
                        display.split_whitespace().collect(),
                    )
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_load_segment_displays() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n";

        let expected_segment_displays = vec![(
            vec![
                "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb",
                "ab",
            ],
            vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"],
        )];

        assert_eq!(load_segment_displays(input), expected_segment_displays);
    }

    #[test_case("cagedb" => Some(0) ; "cagedb, 0")]
    #[test_case("ab" => Some(1) ; "ab, 1")]
    #[test_case("gcdfa" => Some(2) ; "gcdfa, 2")]
    #[test_case("fbcad" => Some(3) ; "fbcad, 3")]
    #[test_case("eafb" => Some(4) ; "eafb, 4")]
    #[test_case("cdfbe" => Some(5) ; "cdfbe, 5")]
    #[test_case("cdfgeb" => Some(6) ; "cdfgeb, 6")]
    #[test_case("dab" => Some(7) ; "dab, 7")]
    #[test_case("acedgfb" => Some(8) ; "acedgfb, 8")]
    #[test_case("cefabd" => Some(9) ; "cefabd, 9")]
    #[test_case("fcadb" => Some(3) ; "fcadb, 3")]
    #[test_case("cdfeb" => Some(5) ; "cdfeb, 5")]
    #[test_case("cdbaf" => Some(3) ; "cdbaf, 3")]
    fn test_identify_digit(display: SegmentDisplay) -> Option<usize> {
        let examples = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        identify_digit(&examples, &display)
    }

    #[test]
    fn test_count_occurances() {
        let segment_displays = vec![
            (vec![], vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]),
            (vec![], vec!["fcgedb", "cgb", "dgebacf", "gc"]),
            (vec![], vec!["cg", "cg", "fdcagb", "cbg"]),
            (vec![], vec!["efabcd", "cedba", "gadfec", "cb"]),
            (vec![], vec!["gecf", "egdcabf", "bgf", "bfgea"]),
            (vec![], vec!["gebdcfa", "ecba", "ca", "fadegcb"]),
            (vec![], vec!["cefg", "dcbef", "fcge", "gbcadfe"]),
            (vec![], vec!["ed", "bcgafe", "cdgba", "cbgef"]),
            (vec![], vec!["gbdfcae", "bgc", "cg", "cgb"]),
            (vec![], vec!["fgae", "cfgab", "fg", "bagce"]),
        ];

        assert_eq!(count_occurance(&segment_displays, vec![1, 4, 7, 8]), 26);
    }
}
