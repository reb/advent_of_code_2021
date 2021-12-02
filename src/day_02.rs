/// --- Day 2: Dive! ---
///
/// Now, you need to figure out how to pilot this thing.
///
/// It seems like the submarine can take a series of commands like forward 1,
/// down 2, or up 3:
///
///     forward X increases the horizontal position by X units.
///     down X increases the depth by X units.
///     up X decreases the depth by X units.
///
/// Note that since you're on a submarine, down and up affect your depth, and so
/// they have the opposite result of what you might expect.
///
/// The submarine seems to already have a planned course (your puzzle input).
/// You should probably figure out where it's going. For example:
///
/// forward 5
/// down 5
/// forward 8
/// up 3
/// down 8
/// forward 2
///
/// Your horizontal position and depth both start at 0. The steps above would
/// then modify them as follows:
///
///     forward 5 adds 5 to your horizontal position, a total of 5.
///     down 5 adds 5 to your depth, resulting in a value of 5.
///     forward 8 adds 8 to your horizontal position, a total of 13.
///     up 3 decreases your depth by 3, resulting in a value of 2.
///     down 8 adds 8 to your depth, resulting in a value of 10.
///     forward 2 adds 2 to your horizontal position, a total of 15.
///
/// After following these instructions, you would have a horizontal position of
/// 15 and a depth of 10. (Multiplying these together produces 150.)
///
/// Calculate the horizontal position and depth you would have after following
/// the planned course. What do you get if you multiply your final horizontal
/// position by your final depth?
use regex::Regex;

const INPUT: &str = include_str!("../input/day_02");

pub fn run() {
    let instructions = parse_instructions(INPUT);

    let (horizontal, depth) = follow_instructions(&instructions);
    println!(
        "The final horizontal position multiplied by final depth is: {}",
        horizontal * depth
    );
}

#[derive(Debug, PartialEq)]
struct Instruction {
    action: Action,
    units: i32,
}

#[derive(Debug, PartialEq)]
enum Action {
    Forward,
    Down,
    Up,
}

fn follow_instructions(instructions: &Vec<Instruction>) -> (i32, i32) {
    instructions
        .iter()
        .fold((0, 0), |(horizontal, depth), instruction| {
            let (d_horizontal, d_depth) = instruction_delta(instruction);
            (horizontal + d_horizontal, depth + d_depth)
        })
}

fn instruction_delta(instruction: &Instruction) -> (i32, i32) {
    match instruction {
        Instruction {
            action: Action::Forward,
            units,
        } => (*units, 0),
        Instruction {
            action: Action::Down,
            units,
        } => (0, *units),
        Instruction {
            action: Action::Up,
            units,
        } => (0, -*units),
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(convert_to_instruction).collect()
}

fn convert_to_instruction(line: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(down|forward|up) ([0-9])").unwrap();
    }
    RE.captures(line).and_then(|groups| {
        groups
            .get(1)
            .and_then(|a| convert_to_action(a.as_str()))
            .and_then(|action| {
                groups
                    .get(2)
                    .and_then(|u| u.as_str().parse().ok())
                    .and_then(|units| Some(Instruction { action, units }))
            })
    })
}

fn convert_to_action(action: &str) -> Option<Action> {
    match action {
        "forward" => Some(Action::Forward),
        "down" => Some(Action::Down),
        "up" => Some(Action::Up),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let input = "\
            down 5\n\
            forward 8\n\
            up 3\n";

        let expected = vec![
            Instruction {
                action: Action::Down,
                units: 5,
            },
            Instruction {
                action: Action::Forward,
                units: 8,
            },
            Instruction {
                action: Action::Up,
                units: 3,
            },
        ];

        assert_eq!(parse_instructions(input), expected);
    }

    #[test]
    fn test_follow_instructions() {
        let instructions = vec![
            Instruction {
                action: Action::Forward,
                units: 5,
            },
            Instruction {
                action: Action::Down,
                units: 5,
            },
            Instruction {
                action: Action::Forward,
                units: 8,
            },
            Instruction {
                action: Action::Up,
                units: 3,
            },
            Instruction {
                action: Action::Down,
                units: 8,
            },
            Instruction {
                action: Action::Forward,
                units: 2,
            },
        ];

        assert_eq!(follow_instructions(&instructions), (15, 10));
    }
}
