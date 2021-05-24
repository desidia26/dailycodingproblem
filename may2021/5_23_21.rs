// Good morning! Here's your coding interview problem for today.

// This problem was recently asked by Google.

// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.

// For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

// Bonus: Can you do this in one pass?
use std::collections::HashSet;

fn main() {
  let arr = [10, 15, 2, 1];
  println!("This array contains additivies {}", contains_additives(&arr, 17));
}

fn contains_additives(arr: &[i32], goal: i32) -> bool {
  let mut num_map = HashSet::<i32>::new();
  for member in arr {
    let remain = goal - *member;
    if num_map.contains(&remain){
      return true;
    }
    num_map.insert(*member);
  }
  false
}

// Time O(n)