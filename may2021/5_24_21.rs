// Good morning! Here's your coding interview problem for today.

// This problem was asked by Uber.

// Given an array of integers, return a new array such that each element at index i of the new array
// is the product of all the numbers in the original array except the one at i.

// For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. 
// If our input was [3, 2, 1], the expected output would be [2, 3, 6].

// Follow-up: what if you can't use division?

fn main() {
  let input_vec_ref = &vec![1,2,3,4,5];
  let mut result = procuctize_with_division(input_vec_ref);
  println!("Result with division: {:?}", result);
  result = procuctize_without_division(input_vec_ref);
  println!("Result without division: {:?}", result);

}

fn procuctize_with_division(input_vec: &[u32]) -> Vec<u32> {
  let mut total: u32 = 1;
  let mut result: Vec<u32> = Vec::with_capacity(input_vec.len());
  for &number in input_vec {
    total *= number;
  }
  for &number in input_vec {
    result.push(total/number);
  }
  result
}

fn procuctize_without_division(input_vec: &[u32]) -> Vec<u32> {
  let mut result: Vec<u32> = vec![1; input_vec.len()];
  let mut temp: u32 = 1;
  let mut index: usize = 0;
  while index < input_vec.len() {
    result[index] = temp;
    temp *= input_vec[index];
    index += 1;
  }
  index = input_vec.len() - 1;
  temp = 1;
  while index >= 1 {
    result[index] *= temp;
    temp *= input_vec[index];
    index -= 1;
  }
  result[index] *= temp;
  result
}