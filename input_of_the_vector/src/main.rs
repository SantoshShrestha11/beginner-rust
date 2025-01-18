use std::{collections::HashMap, io};
fn main() {
    println!("Enter some integers leaving one space");
    let mut input  = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to take the input form you ");
    let ints: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("failed to parse"))
        .collect();
    println!("The sum of the integers is {:#?}",sum(ints.clone()));
    // we can use the &sum here as it does not need to modify the original val so 
    // ya that works but need to use the debug trait
    
    let (median, mode) = find_median_and_mode(ints.clone());// to perform certain operation we n
    // should pass the referance to the original val but here the original val is not mutable so we
    // are just sending the clone so it can be modified properly same goes for the first one 
    println!("Median is : {}", median);
    println!("Mode is : {}", mode);
}
fn sum(integers: Vec<i32>)  -> i32 {
    integers.iter().sum()
}
fn find_median_and_mode(ints: Vec<i32>) -> (f64,i32) {
    let median = match ints.len() {
        even if even % 2 == 0 => {
            let mid1 = ints[even / 2 -1];
            let mid2 = ints[even / 2];
        (mid1 as f64 + mid2 as f64) / 2.0
        },
        odd if odd % 2 != 0 => {
            ints[odd /2] as f64
        },
        _ => 0.0,
    };

    let mut frequencies = HashMap::new();
    for &int in &ints {
        *frequencies.entry(int).or_insert(0) += 1;
    }

    let mut mode: i32 = ints[0];
    let mut max_count = 0;
    for (&int, &count) in &frequencies {
        if count > max_count {
            mode = int;
            max_count = count;

        }
    }
    (median, mode)
}
