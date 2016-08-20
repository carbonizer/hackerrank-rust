pub fn save_the_prisoner<'a>(input: &str) -> Result<String, &'a str> {
    use std::str::{FromStr};
    if input.len() == 0 {
        return Err("No input");
    }
    Ok(input.lines()
           // Don't need to know how many lines follow
           .skip(1)
           .map(|line| {
               // Split line into three numbers
               let v = line.split(' ')
                   .map(|a| usize::from_str(&a).unwrap())
                   .collect::<Vec<_>>();

               // Prisoner and start ids are 1-based
               let (n_prisoners, m_sweets, s_id) = (v[0], v[1], v[2]);
//               let (n_prisoners, m_sweets, s_id) = (v[0], v[1], 1);
               // Remainder after candy evenly distributed
               let rem = m_sweets%n_prisoners;
               // The first `- 1` is because we are interested in the last to receive (rather than
               // the first to not receive).  The other `- 1` works with the `+ 1 to handle the fact
               // that ids are 1-based.  Added `n_prisoners` to ensure that remainder is positive.
               let poisoned_id = ((s_id - 1 + rem) + n_prisoners - 1)%n_prisoners + 1;
               poisoned_id.to_string()
           })
           .collect::<Vec<_>>()
           .join("\n")
    )
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

const FAKE_INPUT: &'static str = "1\n5 2 1";

fn main() {
    input_to_fn_to_stdout(save_the_prisoner, InputType::Stdin);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_save_the_prisoner() {
        let pairs = [
            (
                "1\n\
                 5 2 1",
                "2",
            ),
            (
                "1\n\
                 5 7 1",
                "2",
            ),
            (
                "1\n\
                 5 5 2",
                "1",
            ),
            (
                "1\n\
                 5 5 1",
                "5",
            ),
        ];
        for &(input, expected) in &pairs {
            assert_eq!(expected, save_the_prisoner(input).unwrap());
        }
    }
}
