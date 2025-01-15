//define an enum to store multiple types in one vector

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //creating a new empty vector and adding elements using the push 
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(31);
    numbers.push(32);

    println!("Initial vector: {:?}", numbers);

    //creating a vector with the initial values using the vec! macro
    let mut scores = vec![95,53,54];
    println!("Scores: {:?}", scores);

    // accessing the elements using the indexing and get method
    let second_score = &scores[1];
    println!("The second score is : {second_score}");

    match scores.get(3) {
        Some(score) => println!("The fourth score is : {score}"),
        None => println!("There is no fourth score."),
    }

    // attempting to access an out of bounds index with both method 
    // uncommenting the next line would gives panci mode
    // let invalid_index = &scores[10];
    match scores.get(10) {
        Some(score) => println!("The score at index 10 is:{score}"),
        None => println!("No score at index 10"),

        // we should use the match .get method to avoide the  compile time error
        // because it handels the every case
    }
    
    // demostrating borrowing rules with push and refrences
    let first = &scores[0];
    println!("The first score is : {first}");
    scores.push(90);//this works because first is no longer used after this point
    println!("Update scores: {:?}", scores);

    //iterating over a vector immutably
    println!("Scores:");
    for score in &scores{
        println!("{score}");
    }

    //iterating over a vector mutably and modifying elements
    for score in &mut scores {
        *score += 5; //adding 5 to each score
    }
    println!("Boosted scores: {:?}", scores);

    // using an enum to store multiple types in one vector
    let row = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(43.2),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => print!("Int: {value}"),
            SpreadsheetCell::Float(value) => println!("Float: {value}"),
            SpreadsheetCell::Text(value) => print!("Text: {value}"),
        }
    }

    // demonstrating vector going out of scope an being dropped
{
        let temp_vec = vec![1,2,3];
        println!("Temporary vecotrs are: {:?}", temp_vec);
    }// temp_vec is dropped here 
}
