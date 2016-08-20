pub fn compare_the_triplets<'a>(input: &str) -> Result<String, &'a str> {
    use std::str::{FromStr};
    if input.len() == 0 {
        return Err("No input");
    }
    let num_groups = input.lines()
       .map(|line| {
           // Split line into three numbers
           line.split(' ')
               .map(|a| usize::from_str(&a).unwrap())
               .collect::<Vec<_>>()
       })
       .collect::<Vec<_>>();

    let (alice, bob) = (&num_groups[0], &num_groups[1]);

    let totals = alice.into_iter()
        .zip(bob.into_iter())
        .fold((0, 0), |(acc_a, acc_b), (a, b)| {
            (
                if a > b {
                    acc_a + 1
                } else {
                    acc_a
                },
                if b > a {
                    acc_b + 1
                } else {
                    acc_b
                },
            )
        });
    Ok(format!("{} {}", totals.0, totals.1))
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

const FAKE_INPUT: &'static str = "1\n5 2 1";

fn main() {
    input_to_fn_to_stdout(compare_the_triplets, InputType::Stdin);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_the_triplets() {
        let pairs = [
            (
                "5 6 7\n\
                 3 6 10",
                "1 1",
            ),
        ];
        for &(input, expected) in &pairs {
            assert_eq!(expected, compare_the_triplets(input).unwrap());
        }
    }
}
