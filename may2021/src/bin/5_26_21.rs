// Good morning! Here's your coding interview problem for today.

// This problem was asked by Stripe.

// Given an array of integers, find the first missing positive integer in
// linear time and constant space. In other words, find the lowest positive
// integer that does not exist in the array. The array can contain duplicates
// and negative numbers as well.

// For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0] 
// should give 3.

// You can modify the input array in-place.

use std::collections::HashSet;
use std::iter::FromIterator;

fn main () {
    println!("{}", find_first_missing_pos_int(vec![3, 4, -1, 1, 2]));
}

fn find_first_missing_pos_int(input_array: Vec<i32>) -> u32 {
    let hash: HashSet<i32> = HashSet::from_iter(input_array.iter().cloned());
    let mut i: i32 = 1;
    loop {
        match hash.get(&i) {
            Some(_) => {},
            None => {break}
        }
        i += 1;
    }
    i as u32
}