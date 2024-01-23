/// Advent of Code 2023
///
/// Not really a fan, but let's start and see where it goes
///
/// Part 1: using String API and an extension to Option<T>
///
/// Part 2: don't think the spec is clear; custom 'find' 
/// functions used here

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

/// Reads lines from a file and provides iterators
///
#[derive(Debug)]
pub struct InfiniteLinesReader {
    /// The actual lines read from the file
    lines: Vec<String>,
}

impl InfiniteLinesReader {
    /// Init with lines from a file
    pub fn init(fname: &str) -> Result<Self, Error> {
        // open given file name and read all lines in it
        let f = File::open(fname)?;

        let mut reader = BufReader::new(f);

        let mut lines: Vec<String> = Vec::new();

        let mut buffer = String::new();

        let mut eof = false;
        while !eof {
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    eof = true;
                }
                Ok(_) => {
                    // keep a copy of each line
                    lines.push(buffer.trim_end_matches("\n").to_string());
                    buffer.clear();
                }
                Err(_error) => {
                    return Err(_error);
                }
            }
        }

        Ok(InfiniteLinesReader { lines })
    }

    /// Endlessly provide input lines
    pub fn cycle(&self) -> impl Iterator<Item = &String> {
        self.lines.iter().cycle()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.lines.iter()
    }
    
    pub fn length(&self) -> usize {
        self.lines.len()
    }
}

pub struct PagedIterator<I> {
    page_length: usize,
    page_number: usize,
    line_number: usize,
    iter: I,
}

impl<I> PagedIterator<I> {
    pub fn init(iter: I, page_length: usize) -> PagedIterator<I> {
        PagedIterator { page_length, page_number: 1, line_number: 0, iter }
    }
}

impl<I> Iterator for PagedIterator<I> where I: Iterator {
    type Item = (usize, usize, <I as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            y => {
                self.line_number += 1;
                if self.line_number > self.page_length {
                    self.page_number += 1;
                    self.line_number = 1;
                }
                Some((self.page_number, self.line_number, y?))
            }
        }
    }
}

/// See [extend option with a fallible map method repo](https://github.com/golddranks/try_map)
pub trait DiscardMapExt<T, U, E> {
    /// The function f yields a Result, if it fails we just return None
    fn disc_map<F>(self, f: F) -> Option<U> where F: FnOnce(T) -> Result<U, E>;
}

impl <T, U, E> DiscardMapExt<T, U, E> for Option<T> {
    fn disc_map<F>(self, f: F) -> Option<U> where F: FnOnce(T) -> Result<U, E> {
        match self {
            Some(x) => {
                f(x).ok()
            },
            None => None,
        }
    }
}

pub fn solve(fname: &str) -> Result<usize, Error> {

    let reader = InfiniteLinesReader::init(fname)?;
    let mut lines = PagedIterator::init(reader.iter(), reader.length());

    let mut rx = 0usize;

    while let Some((p, n, cv)) = lines.next() {
        println!("# processing input line {}::{} {}", p, n, cv);
        let digits: Vec<&str> = cv.matches(|x| char::is_ascii_digit(&x)).collect();

        let first = digits.first().disc_map(|digit| digit.parse::<usize>());
        let last = digits.last().disc_map(|digit| digit.parse::<usize>());

        let decoded = match (first, last) {
            (Some(a), Some(b)) => 10 * a + b,
            _ => 0,
        };

        println!("# decoded as {:?} ... {:?} = {}", first, last, decoded);

        rx += decoded;
    }
    
    println!("# result {:?}", rx);

    Ok(rx)
}

/// Finds the first needle in the source string, and returns the associated
/// value
///
pub fn translate_first(source: &str, needles: &[(&str, usize)]) -> Option<usize> {
    let mut n = 0usize;
    while n < source.len() {
        for nd in needles {
            if source[n..].starts_with(nd.0) {
                return Some(nd.1);
            }
        }
        n += 1;
    }
    None
}

/// Finds the last needle in the source string, and returns the associated
/// value
///
pub fn translate_last(source: &str, needles: &[(&str, usize)]) -> Option<usize> {
    let mut n = source.len();
    while n > 0 {
        for nd in needles {
            if source[..n].ends_with(nd.0) {
                return Some(nd.1);
            }
        }
        n -= 1;
    }
    None
}

pub fn ext_solve(fname: &str) -> Result<usize, Error> {

    let reader = InfiniteLinesReader::init(fname)?;
    let mut lines = PagedIterator::init(reader.iter(), reader.length());
    
    let needles = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), 
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6),
        ("7", 7), ("8", 8), ("9", 9), 
    ];

    let mut rx = 0usize;

    while let Some((p, n, cv)) = lines.next() {
        println!("# processing input line {}::{} {}", p, n, cv);
        let first = translate_first(cv, &needles);
        let last = translate_last(cv, &needles);

        let decoded = match (first, last) {
            (Some(a), Some(b)) => 10 * a + b,
            _ => 0,
        };

        println!("# decoded as {:?} ... {:?} = {}", first, last, decoded);

        rx += decoded;
    }
    
    println!("# result {:?}", rx);

    Ok(rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let irl = InfiniteLinesReader::init("data/sample.txt").expect("failed to read input file");
        let mut pi = PagedIterator::init(irl.iter(), 2);
        
        assert_eq!(pi.next().unwrap(), (1, 1, &String::from("1abc2")));
        assert_eq!(pi.next().unwrap(), (1, 2, &String::from("pqr3stu8vwx")));
        assert_eq!(pi.next().unwrap(), (2, 1, &String::from("a1b2c3d4e5f")));
        assert_eq!(pi.next().unwrap(), (2, 2, &String::from("treb7uchet")));
        assert_eq!(pi.next(), None);
    }

    #[test]
    fn sample() {
        let rx = solve("data/sample.txt").expect("failed to solve input puzzle");
        assert_eq!(rx, 142);
    }
    
    // #[test]
    fn puzzle() {
        let _rx = solve("data/input.txt").expect("failed to solve input puzzle");
    }

    #[test]
    fn ext_sample() {
        let rx = ext_solve("data/ext_sample.txt").expect("failed to solve input puzzle");
        assert_eq!(rx, 281);
    }

    #[test]
    fn ext_puzzle() {
        let _rx = ext_solve("data/input.txt").expect("failed to solve input puzzle");
    }
}
