pub fn new_year_chaos<'a>(input: &str) -> Result<String, &'a str> {
    if input.len() == 0 {
        return Err("Input Missing");
    }
    Ok(input.lines().skip(1).enumerate().filter_map(|(i, line)| {
        match i%2 {
            1 => {
                // Convert position strings to numbers
                let positions: Vec<_> = line.split(' ')
                    .map(|a| a.parse::<isize>().unwrap())
                    .collect();

                // Determine how many bribes each person is the line gave
                let bribe_counts: Vec<_> = positions.iter().enumerate().map(|(i, orig_pos)| {
                    let ii = i as isize;
                    let curr_pos = ii + 1;
                    let delta = orig_pos - curr_pos;

                    // If closer to the front of the line, delta is the number of bribes.  Note that
                    // you can ignore the situation of two bribes then n receives because at least
                    // one of those would have to undo a previous bribe.
                    if delta > 0 {
                        delta
                    }
                    // Although no farther forward, the person may have bribed once then
                    // received n times where n is (|delta| + 1). If this is the case, all of the
                    // next n people in front must have previously been behind.
                    else if ii > -delta
                        && positions[(ii - (-delta + 1)) as usize..i].iter().all(|c| c > orig_pos) {
                        1
                    }
                    // Otherwise, there was no bribing by this person
                    else {
                        0
                    }
                }).collect();

                // Check that no one bribed more than two times
                if bribe_counts.iter().any(|&c| c > 2){
                    Some("Too chaotic".to_string())
                } else {
                    // Sum counts
                    Some(bribe_counts.iter().fold(0, |acc, c| acc + c).to_string())
                }
            }
            _ => None,
        }
    }).collect::<Vec<_>>().join("\n"))
}

pub enum InputType {
    None,
    Stdin,
    File,
    Const,
}

fn input_to_fn_to_stdout<'a, T>(f: fn(&str) -> Result<T, &'a str>, it: InputType)
    where T: std::fmt::Display {
    use std::io::{Read};
    use std::fs::{File};

    let mut input = String::new();
    let mut fp: Option<File> = None;

    match it {
        InputType::Stdin => {
            fp = File::open("/dev/stdin").ok();
        }

        InputType::File => {
            if let Some(path) = std::env::args().nth(1) {
                match File::open(path) {
                    Ok(unwrapped_fp) => fp = Some(unwrapped_fp),
                    Err(error) => panic!("While opening custom path: {}", error)
                }
            } else {
                panic!("Expected arg 1 to be file path");
            }
        }

        InputType::Const => {
            input.push_str(&FAKE_INPUT);
        }

        InputType::None => {}
    }

    if let Some(mut unwrapped) = fp {
        match unwrapped.read_to_string(&mut input) {
            Ok(_) => {}
            Err(error) => panic!("While reading from input: {}", error)
        }
    }

    match f(&input) {
        Ok(rv) => {
            println!("{}", rv);
        }
        Err(error) => panic!("While running func\nInput: {:?}\nError: {}", &input, error),
    }
}

const FAKE_INPUT: &'static str = "0 3 4 2";

fn main() {
    input_to_fn_to_stdout(new_year_chaos, InputType::Stdin);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_year_chaos() {
        let pairs = [
            (
                "2\n\
                 5\n\
                 2 1 5 3 4\n\
                 5\n\
                 2 5 1 3 4",
                "3\n\
                 Too chaotic",
            ),
            (
                "1\n\
                 5\n\
                 1 2 5 4 3",
                "3",
            ),
        ];
        for &(input, expected) in &pairs {
            assert_eq!(expected, new_year_chaos(input).unwrap());
        }
    }
}
