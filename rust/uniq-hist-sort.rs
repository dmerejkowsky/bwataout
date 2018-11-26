use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn process(input: &Vec<String>) -> Vec<&String> {
    let mut seen = HashSet::new();
    let mut res = vec![];
    for line in input.into_iter().rev() {
        if seen.contains(&line) {
            continue;
        }
        res.push(line);
        seen.insert(line);
    }
    res.into_iter().rev().collect()
}

fn main() {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let mut hist = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        hist.push(line);
    }
    let out = process(&hist);
    for line in out {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorting_input() {
        let input = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "two".to_string(),
            "four".to_string(),
        ];
        let actual = process(&input);
        let expected = vec!["one", "three", "two", "four"];
        assert_eq!(actual, expected);
    }

}
