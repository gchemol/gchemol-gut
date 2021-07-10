// [[file:../gut.note::*utils.rs][utils.rs:1]]
use crate::prelude::*;

/// Sleep a few seconds
pub fn sleep(t: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(t));
}

/// Parse a list of numbers from a readable string.
///
/// "2-5"   ==> vec![2, 3, 4, 5]
/// "1,3-5" ==> vec![1, 3, 4, 5]
/// "1 3,5" ==> vec![1, 3, 4, 5]
///
pub fn parse_numbers_human_readable(s: &str) -> Result<Vec<usize>> {
    let list_or: Result<Vec<_>> = s
        .trim()
        .replace(",", " ")
        .split_whitespace()
        .map(|s| parse_numbers_field(s))
        .collect();
    let list = list_or
        .with_context(|| format!("invalid {}", s))?
        .concat()
        .into_iter()
        .sorted()
        .dedup()
        .collect();
    Ok(list)
}

/// "2"     ==> vec![2]
/// "2-5"   ==> vec![2, 3, 4, 5]
fn parse_numbers_field(s: &str) -> Result<Vec<usize>> {
    match s.parse() {
        Ok(n) => Ok(vec![n]),
        Err(_) => {
            if s.contains("-") {
                let nn: Result<Vec<_>> = s.split("-").map(|k| k.parse::<usize>().context("atom list range")).collect();
                match nn?.as_slice() {
                    [a, b] => Ok((*a..=*b).collect()),
                    rest => bail!("invalid atom list range: {:?}", rest),
                }
            } else {
                bail!("invalid atom list format: {:?}", s);
            }
        }
    }
}

#[test]
fn test_parse_numbers() {
    let x = parse_numbers_human_readable("2,5").unwrap();
    assert_eq!(vec![2, 5], x);

    let x = parse_numbers_human_readable("2-5").unwrap();
    assert_eq!(vec![2, 3, 4, 5], x);

    let x = parse_numbers_human_readable("2-5,1,3").unwrap();
    assert_eq!(vec![1, 2, 3, 4, 5], x);

    let x = parse_numbers_human_readable("5-2,1,3").unwrap();
    assert_eq!(vec![1, 3], x);

    let _ = parse_numbers_human_readable("a-2,1,3").unwrap_err();
}
// utils.rs:1 ends here
