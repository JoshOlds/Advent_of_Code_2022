use std::fmt::Debug;
use std::{str::FromStr};


/// Prints a pretty header for puzzle
pub fn print_puzzle_header(day_num: i32){
    println!("--------------------\n       Day {}     \n--------------------", day_num);
}

/// Generic function that splits a string into a vector and converts to generic types
pub fn split_string_to_vector<T: FromStr>(input: &str, delimiter: &str) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
{
    let in_vector: Vec<&str> = input.split(delimiter).collect();
    let mut out_vector: Vec<T> = Vec::with_capacity(in_vector.len());
    for string in in_vector.iter() {
        out_vector.push(string.parse::<T>().unwrap());
    }
    out_vector
}
