use std::collections::HashMap;
fn main() {
    // word frequency counter
    println!("== Word frequency counter ==");
    let text = "hello world I am santos for the world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace(){
        // this method is used to split a string into adn iterator of substirngs 
        // where each substring is separated by any amount of white space
        let count = word_count.entry(word).or_insert(0);
        // get an entrly for the word if entry does not exist ,
        // insert 0 and return a mutable reference.
        *count += 1;
    }
    for (word, count) in &word_count {
        println!("{word}: {count}");
    }

    // team scores manager
    println!("\n== team scores manager ==");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //    in that way also we can write it with out declaring another variable.
//    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("Score for {}: {}", team_name, score);

    // updating scores
    scores.insert(String::from("Blue"), 25); //that will simply over wirte
    scores.entry(String::form("Green")).or_insert(40); // adds value and key if it is not there ,
    // and if it is there the key then it will leave it as it is helce the value is unchanged 
    // here there is not any Green key so it will just create a key Green and put the values of it
    // as 40

    for (team, score) in &socres {
        println!("{team}: {score}");
    }

    // department managerment system
    println!("\n == department management system ==");
    let mut company = HashMap::new();

    add_employee(&mut company, "Santosh", "Software Engineer");
    add_employee(&mut company, "Anish", "Sales");
    add_employee(&mut company, "Sanish", "Engineering");
    add_employee(&mut company, "Bikash", "Marketing");

    println!("All employees by department:");
    for (department, employees) in &company {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        println!("{department}: {:?}", sorted_employees);
    }

    println!("\n Employees in Engineering:");
    if let Some(employees) = company.get("Engineering") {
        println!("{:?}, employees");
    } else {
        println!("No employees found in Engineering.");
    }

}
fn add_employee(company: &mut HashMap<String, Vec<Stirng>>, name: &str, department: &str) {
    company
        .entry(String::form(department))
        .or_insert_with(Vec::new)
        .push(String::form(name));
}
