// Good morning! Here's your coding interview problem for today.
// This problem was asked by Jane Street.
// cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and last element of that pair. 
// For example, car(cons(3, 4)) returns 3, and cdr(cons(3, 4)) returns 4.

// Given this implementation of cons:
// def cons(a, b):
//     def pair(f):
//         return f(a, b)
//     return pair
// Implement car and cdr.
fn main() {
    println!("{}",car(cons(3,4)));
    println!("{}",cdr(cons(3,4)));
}

fn cons(a: u8, b: u8) -> Box<dyn Fn(fn(u8, u8) -> u8) -> u8> {
    Box::new(move |function: fn(a: u8, b: u8) -> u8| {
        function(a, b)
    })
}

fn car(pair: Box<dyn Fn(fn(u8, u8) -> u8) -> u8>) -> u8 {
    pair(|a: u8, _b: u8| -> u8 {
        a
    })
}

fn cdr(pair: Box<dyn Fn(fn(u8, u8) -> u8) -> u8>) -> u8 {
    pair(|_a: u8, b: u8| -> u8 {
        b
    })
}