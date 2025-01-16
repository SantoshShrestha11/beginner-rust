use std::collections::HashMap;

fn find_median_and_mode(mut integers: Vec<i32>) -> (f64, i32) {
    // Sorting the vector
    integers.sort();

    // Calculating the median
    let median = match integers.len() {
        0 => 0.0,
        even if even % 2 == 0 => {
            let mid1 = integers[even / 2 - 1];
            let mid2 = integers[even / 2];
            (mid1 as f64 + mid2 as f64) / 2.0
        },
        odd => integers[integers.len() / 2] as f64, 
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
            mode = *integer;
            max_count = count;
        }
    }

    (median, mode)
}

fn main() {
    let integers = vec![1, 2, 3, 1, 2, 31, 1, 34, 4, 5];
    let (median, mode) = find_median_and_mode(integers);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
