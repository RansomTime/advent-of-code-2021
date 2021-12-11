mod inputs;
fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> i32 {
    let mut res = 0;
    for line in inputs::input().lines() {
        res += get_string_score(line);
    }

    res

}

fn part_2() -> i64 {
    let mut scores = vec![];
    for line in inputs::input().lines() {
        match get_line_score(line) {
            0 => (),
            score => scores.push(score),
        }
    }
    scores.sort_unstable();

    scores[(scores.len()/2)]
}


fn get_closing_symbol(symbol: char) -> char {
    match symbol {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' | ']' | '}' | '>' => panic!("get_closing() called on a closing symbol. {}", symbol),
        _ => panic!("Invalid symbol: {}", symbol)
    }
}

fn get_string_score(string: &str) -> i32 {
    fn invalid_symbol_score(symbol: char) -> i32 {
        match symbol {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }
    let mut seen: Vec<char> =  vec![];
    for token in string.chars() {
        match token {
            '(' | '[' | '{' | '<' => seen.push(token),
            ')' | ']' | '}' | '>' => {
                match seen.last() {
                    None => panic!("I don't think this should happen, seen closing token with no opening tokens on stack"), // seen closing token with no opening tokens
                    Some(opener) => {
                        if token == get_closing_symbol(*opener) {
                            seen.pop().unwrap();
                        } else {
                            return invalid_symbol_score(token); // seen wrong closing token.
                        }
                    }
                }
            },
            _ => panic!("invalid token {}", token),
        }
    }
    0
}

fn complete_string(string: &str) -> Vec<char> {
    let mut seen: Vec<char> =  vec![];
    for token in string.chars() {
        match token {
            '(' | '[' | '{' | '<' => seen.push(token),
            ')' | ']' | '}' | '>' => {
                match seen.last() {
                    None => panic!("I don't think this should happen, seen closing token with no opening tokens on stack"), // seen closing token with no opening tokens
                    Some(opener) => {
                        if token == get_closing_symbol(*opener) {
                            seen.pop().unwrap();
                        } else {
                            return vec![]; // seen wrong closing token. So string score will be 0
                            // return empty vector so we can score this 0
                        }
                    }
                }
            },
            _ => panic!("invalid token {}", token),
        }
    }
    let mut res: Vec<char> = vec![];
    loop {
        match seen.pop() {
            None => return res,
            Some(token) => res.push(get_closing_symbol(token)),
        };
    }
}

fn get_line_score(line: &str) -> i64 {
    fn invalid_symbol_score(symbol: char) -> i64 {
        match symbol {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("invalid symbol {}", symbol),
        }
    }
    //Start with a total score of 0.
    let mut res = 0;
    //Then, for each character, multiply the total score by 5 
    // and then increase the total score by the point value given for the character in the following table:
    for symbol in complete_string(line) {
        res *= 5;
        res += invalid_symbol_score(symbol);
    }
    res
}

#[cfg(test)]
mod test {
    use crate::inputs;
    use crate::*;

    #[test]
    fn find_closing_symbol() {
        assert_eq!(get_closing_symbol('('), ')');
        assert_eq!(get_closing_symbol('<'), '>');
    }

    #[test]
    fn parse_string() {
        fn invalid_symbol_score(symbol: char) -> i32 { // hoist for clearer debug
            match symbol {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        }
        assert_eq!(get_string_score("{([(<{}[<>[]}>{[]{[(<()>"), invalid_symbol_score('}'));
        assert_eq!(get_string_score("[[<[([]))<([[{}[[()]]]"), invalid_symbol_score(')'));
        assert_eq!(get_string_score("[{[{({}]{}}([{[{{{}}([]"), invalid_symbol_score(']'));
        assert_eq!(get_string_score("[<(<(<(<{}))><([]([]()"), invalid_symbol_score(')'));
        assert_eq!(get_string_score("<{([([[(<>()){}]>(<<{{"), invalid_symbol_score('>'));
        assert_eq!(get_string_score("<{([([[(<>()){}]>(<<{{"), invalid_symbol_score('>'));
        assert_eq!(get_string_score("[({(<(())[]>[[{[]{<()<>>"), 0);
    }

    #[test]
    fn part_1_test() {
        let mut res = 0;
        for line in inputs::test().lines() {
            res += get_string_score(line);
        }

        assert_eq!(res, 26397);
    }

    #[test]
    fn complete_lines() {
        assert_eq!(complete_string("{([(<{}[<>[]}>{[]{[(<()>"), vec![]);
        assert_eq!(complete_string("[({(<(())[]>[[{[]{<()<>>").into_iter().collect::<String>(), String::from("}}]])})]"));
        assert_eq!(complete_string("{<[[]]>}<{[{[{[]{()[[[]").into_iter().collect::<String>(), String::from("]]}}]}]}>"));
    }

    

    #[test]
    fn line_score() {
        assert_eq!(get_line_score("{([(<{}[<>[]}>{[]{[(<()>"), 0);
        assert_eq!(get_line_score("[({(<(())[]>[[{[]{<()<>>"), 288957);
    }

    #[test]
    fn part_2_test() {
        let mut scores = vec![];
        for line in inputs::test().lines() {
            match get_line_score(line) {
                0 => (),
                score => scores.push(score),
            }
        }
        scores.sort_unstable();

        assert_eq!(scores[(scores.len()/2)], 288957);
    }
}