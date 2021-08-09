// [[file:../gut.note::*utils.rs][utils.rs:1]]
use crate::prelude::*;

/// Sleep a few seconds
pub fn sleep(t: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(t));
}

/// Make an abbreviation of the long number list. Return the string
/// representation. For example: 1,2,3,6,7,8,9 ==> 1-3,6-9
pub fn abbreviate_numbers_human_readable(numbers: &[usize]) -> Result<String> {
    let n = numbers.len();
    if n == 0 {
        return Ok(String::new());
    }
    // sort the numbers before make abbreviation
    let mut numbers = numbers.to_vec();
    numbers.sort();

    // a little trick
    let num_terminator = numbers[n - 1] + 9;
    numbers.push(num_terminator);

    let mut old = numbers[0];
    let mut abbreviation = old.to_string();
    for w in numbers.windows(2) {
        let pre = w[0];
        let new = w[1];
        if new - pre <= 1 {
            continue;
        }
        if old != pre {
            abbreviation += &format!("-{},{}", pre, new);
        } else {
            abbreviation += &format!(",{}", new);
        }
        old = new;
    }
    let n = abbreviation.len();
    let bits = num_terminator.to_string().len() + 1;
    Ok(abbreviation[0..n - bits].to_string())
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
fn test_parse_and_abbreviate_numbers() {
    let x = parse_numbers_human_readable("2,5").unwrap();
    assert_eq!(vec![2, 5], x);

    let x = parse_numbers_human_readable("2-5").unwrap();
    assert_eq!(vec![2, 3, 4, 5], x);

    let x = parse_numbers_human_readable("2-5,1,3").unwrap();
    assert_eq!(vec![1, 2, 3, 4, 5], x);

    let x = parse_numbers_human_readable("5-2,1,3").unwrap();
    assert_eq!(vec![1, 3], x);

    let _ = parse_numbers_human_readable("a-2,1,3").unwrap_err();

    let x = abbreviate_numbers_human_readable(&[1, 2, 3, 6]).unwrap();
    assert_eq!(x, "1-3,6");
    let x = abbreviate_numbers_human_readable(&[1, 2, 3, 6, 7, 9]).unwrap();
    assert_eq!(x, "1-3,6-7,9");
    let x = abbreviate_numbers_human_readable(&[3, 2, 0, 6, 7, 9]).unwrap();
    assert_eq!(x, "0,2-3,6-7,9");
    let x = abbreviate_numbers_human_readable(&[1, 2, 3, 4, 5]).unwrap();
    assert_eq!(x, "1-5");

    let x = parse_numbers_human_readable("2-5,7-9").unwrap();
    let y = abbreviate_numbers_human_readable(&x).unwrap();
    assert_eq!(y, "2-5,7-9");
}
// utils.rs:1 ends here
