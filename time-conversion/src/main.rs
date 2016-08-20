pub fn time_conversion<'a>(input: &str) -> Result<String, &'a str> {
    if input.len() != 10 {
        return Err("Input is not of form hh:mm:ss(AM|PM)");
    }
    let (hh, mm, ss, am_or_pm) = (&input[0..2], &input[3..5],
                                  &input[6..8], &input[8..10]);
    let hh_out = hh.parse::<isize>().unwrap() + match (hh, am_or_pm) {
        ("12", "AM") => -12,
        (_, "AM") | ("12", "PM") => 0,
        _ => 12,
    };

    Ok(format!("{:02}:{:02}:{:02}", hh_out, mm, ss))
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

const FAKE_INPUT: &'static str = "07:05:45PM";

fn main() {
    input_to_fn_to_stdout(time_conversion, InputType::Stdin);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time_conversion() {
        let pairs = [
            (
                "07:05:45PM",
                "19:05:45",
            ),
            (
                "07:05:45AM",
                "07:05:45",
            ),
            (
                "12:05:45AM",
                "00:05:45",
            ),
            (
                "12:05:45PM",
                "12:05:45",
            ),
        ];
        for &(input, expected) in &pairs {
            assert_eq!(expected, time_conversion(input).unwrap());
        }
    }
}
