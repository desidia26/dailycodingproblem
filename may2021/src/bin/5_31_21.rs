
// Good morning! Here's your coding interview problem for today.

// This problem was asked by Airbnb.

// Given a list of integers, write a function that returns the largest sum 
// of non-adjacent numbers. Numbers can be 0 or negative.

// For example, [2, 4, 6, 2, 5] should return 13, since
// we pick 2, 6, and 5. [5, 1, 1, 5] should return 10, since we pick 5 and 5.

// Follow-up: Can you do this in O(N) time and constant space?

use std::collections::HashMap;
use std::cmp;
fn main() {
    let arr1: Vec<i32> = vec![2, 4, 6, 2, 5];
    let arr2: Vec<i32> = vec![5, 1, -5, 1, 5];
    println!("{}", find_largest_non_adjacent_sum(arr2));
}

fn find_largest_non_adjacent_sum(input_arr: Vec<i32>) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut index = input_arr.len() - 1;
    loop {
        let mut new_sum = cmp::max(input_arr[index] + second, first);
        second = first;
        first = new_sum;
        if(index == 0) {
            break;
        }
        else {
            index -= 1;
        }
    }
    first
}