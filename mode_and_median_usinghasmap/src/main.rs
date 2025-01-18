use std::collections::HashMap;
// use std::io::Read;
// use std::io::BufRead;

fn find_median_and_mode(mut integers: Vec<i32>) -> (f64, i32) {
    // Sorting the vector
    integers.sort();

    // Calculating the median
    let median = match integers.len() {
        even if even % 2 == 0 => {
            let mid1 = integers[even / 2 - 1];
            let mid2 = integers[even / 2];
            (mid1 as f64 + mid2 as f64) / 2.0
        },
        odd if odd % 2 != 0 => {
            integers[odd / 2] as f64
        },
        _ => 0.0,
    };

    // Calculating the mode using a HashMap
    let mut frequencies = HashMap::new();
    for &integer in &integers {
        *frequencies.entry(integer).or_insert(0) += 1;
    }

    let mut mode: i32 = integers[0];
    let mut max_count = 0;
    for (&integer, &count) in &frequencies {
        if count > max_count {
            mode = integer;
            max_count = count;
        }
    }

    (median, mode)
}

fn main() {
    // println!("Enter a list of the integers seperated by spaces");
    // let mut values = String::new()
    let integers = vec![2,4,5,5,6,2,65,22,4,52,5,25,265,2,42,422,42,42,42,42,42,52,52,5];
    let (median, mode) = find_median_and_mode(integers);
    // we passed the direct values of the integers here
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
