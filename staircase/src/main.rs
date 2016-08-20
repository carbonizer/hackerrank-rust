pub fn staircase<'a>(stdin: &str) -> Result<String, &'a str> {
    use std::str::{FromStr};
    if stdin.len() == 0 {
        return Err("No input");
    }
    let num: usize = usize::from_str(&stdin).unwrap();
    let mut lines = vec![String::new(); num];
    for (i, line) in lines.iter_mut().enumerate() {
        let n = i + 1;
        for _ in 0..(num - n) {
            line.push(' ');
        }
        for _ in 0..n {
            line.push('#');
        }
    }

    Ok(lines.join("\n"))
}

enum InputType {
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
            match std::env::args().nth(1) {
                Some(path) => {
                    match File::open(path) {
                        Ok(unwrapped_fp) => fp = Some(unwrapped_fp),
                        Err(error) => panic!("While opening custom path: {}", error)
                    }
                }
                None => panic!("Expected arg 1 to be file path")
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

const FAKE_INPUT: &'static str = "3 5 10";

fn main() {
    input_to_fn_to_stdout(staircase, InputType::Stdin);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_staircase() {
        let pairs = [
            ("6", concat!(
                "     #\n",
                "    ##\n",
                "   ###\n",
                "  ####\n",
                " #####\n",
                "######",
            )),
        ];
        for &(input, expected) in &pairs {
            assert_eq!(expected, staircase(input).unwrap());
        }
    }
}
