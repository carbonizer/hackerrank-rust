pub fn kangaroo<'a>(input: &str) -> Result<&'a str, &'a str> {
    if input.len() == 0 {
        return Err("Input Missing");
    }
    let nums = input.split(' ').map(|a| a.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let (x1, v1, x2, v2) = (nums[0], nums[1], nums[2], nums[3]);

    static FALSE_OK_MSG: Result<&'static str, &'static str> = Ok("NO");
    static TRUE_OK_MSG: Result<&'static str, &'static str> = Ok("YES");

    struct Hops {
        curr: isize,
        rate: isize,
    }

    impl Iterator for Hops {
        type Item = isize;
        fn next(&mut self) -> Option<isize> {
            self.curr += self.rate;
            Some(self.curr)
        }
    }

    // x1 < x2, so v1 > v2 to have a chance to land on the same spot
    if v1 <= v2 {
        return FALSE_OK_MSG;
    }

    // Create iterators of the hops the kangaroos will take
    let hops1 = Hops{ curr: x1, rate: v1 };
    let hops2 = Hops{ curr: x2, rate: v2 };

    // Compare locations of kangaroos
    for (loc1, loc2) in hops1.zip(hops2) {
        match (loc1 - loc2).signum() {
            -1 => continue,
            0 => break,
            1 | _ => return FALSE_OK_MSG,
        }
    }

    TRUE_OK_MSG
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
    input_to_fn_to_stdout(kangaroo, InputType::Stdin);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kangaroo() {
        let pairs = [
            (
                "0 3 4 2",
                "YES",
            ),
            (
                "0 2 5 3",
                "NO",
            ),
        ];
        for &(input, expected) in &pairs {
            assert_eq!(expected, kangaroo(input).unwrap());
        }
    }
}
