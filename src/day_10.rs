/// --- Day 10: Syntax Scoring ---
///
/// You ask the submarine to determine the best route out of the deep-sea cave,
/// but it only replies:
///
/// Syntax error in navigation subsystem on line: all of them
///
/// All of them?! The damage is worse than you thought. You bring up a copy of
/// the navigation subsystem (your puzzle input).
///
/// The navigation subsystem syntax is made of several lines containing chunks.
/// There are one or more chunks on each line, and chunks contain zero or more
/// other chunks. Adjacent chunks are not separated by any delimiter; if one
/// chunk stops, the next chunk (if any) can immediately start. Every chunk must
/// open and close with one of four legal pairs of matching characters:
///
///   - If a chunk opens with (, it must close with ).
///   - If a chunk opens with [, it must close with ].
///   - If a chunk opens with {, it must close with }.
///   - If a chunk opens with <, it must close with >.
///
/// So, () is a legal chunk that contains no other chunks, as is []. More
/// complex but valid chunks include ([]), {()()()}, <([{}])>,
/// [<>({}){}[([])<>]], and even (((((((((()))))))))).
///
/// Some lines are incomplete, but others are corrupted. Find and discard the
/// corrupted lines first.
///
/// A corrupted line is one where a chunk closes with the wrong character - that
/// is, where the characters it opens and closes with do not form one of the
/// four legal pairs listed above.
///
/// Examples of corrupted chunks include (], {()()()>, (((()))}, and
/// <([]){()}[{}]). Such a chunk can appear anywhere within a line, and its
/// presence causes the whole line to be considered corrupted.
///
/// For example, consider the following navigation subsystem:
///
/// [({(<(())[]>[[{[]{<()<>>
/// [(()[<>])]({[<{<<[]>>(
/// {([(<{}[<>[]}>{[]{[(<()>
/// (((({<>}<{<{<>}{[]{[]{}
/// [[<[([]))<([[{}[[()]]]
/// [{[{({}]{}}([{[{{{}}([]
/// {<[[]]>}<{[{[{[]{()[[[]
/// [<(<(<(<{}))><([]([]()
/// <{([([[(<>()){}]>(<<{{
/// <{([{{}}[<[[[<>{}]]]>[]]
///
/// Some of the lines aren't corrupted, just incomplete; you can ignore these
/// lines for now. The remaining five lines are corrupted:
///
///   - {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
///   - [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
///   - [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
///   - [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
///   - <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
///
/// Stop at the first incorrect closing character on each corrupted line.
///
/// Did you know that syntax checkers actually have contests to see who can get
/// the high score for syntax errors in a file? It's true! To calculate the
/// syntax error score for a line, take the first illegal character on the line
/// and look it up in the following table:
///
///   - ): 3 points.
///   - ]: 57 points.
///   - }: 1197 points.
///   - >: 25137 points.
///
/// In the above example, an illegal ) was found twice (2*3 = 6 points), an
/// illegal ] was found once (57 points), an illegal } was found once (1197
/// points), and an illegal > was found once (25137 points). So, the total
/// syntax error score for this file is 6+57+1197+25137 = 26397 points!
///
/// Find the first illegal character in each corrupted line of the navigation
/// subsystem. What is the total syntax error score for those errors?
///
/// --- Part Two ---
///
/// Now, discard the corrupted lines. The remaining lines are incomplete.
///
/// Incomplete lines don't have any incorrect characters - instead, they're
/// missing some closing characters at the end of the line. To repair the
/// navigation subsystem, you just need to figure out the sequence of closing
/// characters that complete all open chunks in the line.
///
/// You can only use closing characters (), ], }, or >), and you must add them
/// in the correct order so that only legal pairs are formed and all chunks end
/// up closed.
///
/// In the example above, there are five incomplete lines:
///
///   - [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
///   - [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
///   - (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
///   - {<[[]]>}<{[{[{[]{()[[[] - Complete by adding ]]}}]}]}>.
///   - <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
///
/// Did you know that autocomplete tools also have contests? It's true! The
/// score is determined by considering the completion string
/// character-by-character. Start with a total score of 0. Then, for each
/// character, multiply the total score by 5 and then increase the total score
/// by the point value given for the character in the following table:
///
///   - ): 1 point.
///   - ]: 2 points.
///   - }: 3 points.
///   - >: 4 points.
///
/// So, the last completion string above - ])}> - would be scored as follows:
///
///   - Start with a total score of 0.
///   - Multiply the total score by 5 to get 0, then add the value of ] (2) to
///     get a new total score of 2.
///   - Multiply the total score by 5 to get 10, then add the value of ) (1) to
///     get a new total score of 11.
///   - Multiply the total score by 5 to get 55, then add the value of } (3) to
///     get a new total score of 58.
///   - Multiply the total score by 5 to get 290, then add the value of > (4) to
///     get a new total score of 294.
///
/// The five lines' completion strings have total scores as follows:
///
///   - }}]])})] - 288957 total points.
///   - )}>]}) - 5566 total points.
///   - }}>}>)))) - 1480781 total points.
///   - ]]}}]}]}> - 995444 total points.
///   - ])}> - 294 total points.
///
/// Autocomplete tools are an odd bunch: the winner is found by sorting all of
/// the scores and then taking the middle score. (There will always be an odd
/// number of scores to consider.) In this example, the middle score is 288957
/// because there are the same number of scores smaller and larger than it.
///
/// Find the completion string for each incomplete line, score the completion
/// strings, and sort the scores. What is the middle score?
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_10");

pub fn run() {
    let navigation_subsystem = INPUT;

    let autocompletions = navigation_subsystem
        .lines()
        .map(autocomplete_line)
        .collect();

    let total_syntax_error_score = calculate_error_score(&autocompletions);
    println!(
        "The total syntax error score for the first illegal character errors is: {}",
        total_syntax_error_score
    );

    let autocomplete_middle_score = calculate_middle_score(&autocompletions);
    println!(
        "The middle score for the autocompletions is: {}",
        autocomplete_middle_score
    );
}

fn calculate_error_score(autocompletions: &Vec<Result<String, char>>) -> u32 {
    autocompletions
        .iter()
        .filter_map(|r| r.as_ref().err())
        .map(line_error_score)
        .sum()
}

fn line_error_score(error: &char) -> u32 {
    match error {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn calculate_middle_score(autocompletions: &Vec<Result<String, char>>) -> u64 {
    let mut scores = autocompletions
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .map(autocompletion_score)
        .collect::<Vec<_>>();
    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}

fn autocompletion_score(autocompletion: &String) -> u64 {
    autocompletion.chars().fold(0, |total, closer| {
        total * 5
            + match closer {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
    })
}

fn autocomplete_line(line: &str) -> Result<String, char> {
    lazy_static! {
        static ref OPENERS: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')]
            .into_iter()
            .collect();
        static ref CLOSERS: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
            .into_iter()
            .collect();
    }

    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            closer => {
                if OPENERS.get(&closer).map(|opener| *opener) != stack.pop() {
                    return Err(closer);
                }
            }
        }
    }

    // autocomplete the line using the remaining stack
    Ok(stack
        .iter()
        .filter_map(|opener| CLOSERS.get(opener))
        .rev()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn autocompletions() -> Vec<Result<String, char>> {
        vec![
            Ok("}}]])})]".to_string()),
            Ok(")}>]})".to_string()),
            Err('}'),
            Ok("}}>}>))))".to_string()),
            Err(')'),
            Err(']'),
            Ok("]]}}]}]}>".to_string()),
            Err(')'),
            Err('>'),
            Ok("])}>".to_string()),
        ]
    }

    #[test_case("[({(<(())[]>[[{[]{<()<>>" => Ok("}}]])})]".to_string())  ; "1")]
    #[test_case("[(()[<>])]({[<{<<[]>>("   => Ok(")}>]})".to_string())    ; "2")]
    #[test_case("{([(<{}[<>[]}>{[]{[(<()>" => Err('}')                    ; "3")]
    #[test_case("(((({<>}<{<{<>}{[]{[]{}"  => Ok("}}>}>))))".to_string()) ; "4")]
    #[test_case("[[<[([]))<([[{}[[()]]]"   => Err(')')                    ; "5")]
    #[test_case("[{[{({}]{}}([{[{{{}}([]"  => Err(']')                    ; "6")]
    #[test_case("{<[[]]>}<{[{[{[]{()[[[]"  => Ok("]]}}]}]}>".to_string()) ; "7")]
    #[test_case("[<(<(<(<{}))><([]([]()"   => Err(')')                    ; "8")]
    #[test_case("<{([([[(<>()){}]>(<<{{"   => Err('>')                    ; "9")]
    #[test_case("<{([{{}}[<[[[<>{}]]]>[]]" => Ok("])}>".to_string())      ; "10")]
    fn test_autocomplete_line(line: &str) -> Result<String, char> {
        autocomplete_line(line)
    }

    #[test]
    fn test_calculate_error_score() {
        assert_eq!(calculate_error_score(&autocompletions()), 26397);
    }

    #[test_case(&"}}]])})]".to_string()  => 288957  ; "1")]
    #[test_case(&")}>]})".to_string()    => 5566    ; "2")]
    #[test_case(&"}}>}>))))".to_string() => 1480781 ; "3")]
    #[test_case(&"]]}}]}]}>".to_string() => 995444  ; "4")]
    #[test_case(&"])}>".to_string()      => 294     ; "5")]
    fn test_line_autocompletion_score(autocompletion: &String) -> u32 {
        autocompletion_score(autocompletion)
    }

    #[test]
    fn test_calculate_middle_score() {
        assert_eq!(calculate_middle_score(&autocompletions()), 288957);
    }
}
