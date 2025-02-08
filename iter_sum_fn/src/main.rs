fn main() {
    let vec1: Vec<i32> = vec![1,2,3,4,4,1,2,1];
    let vec2: Vec<i32> = vec![23,232,423,423,24,53];
    let vec3: Vec<i32> = vec![42,32,52,32,52,32,24];
    let vec4: Vec<i32> = vec![42,423,423,423,2,52,32,32,42];
    let vec5: Vec<i32> = vec![42,423,42,32,32,42,32,52];
    println!("The sume is here: {}", iterator_sum(vec1));
    println!("The sume is here: {}", iterator_sum(vec2));
    println!("The sume is here: {}", iterator_sum(vec3));
    println!("The sume is here: {}", iterator_sum(vec4));
    println!("The sume is here: {}", iterator_sum(vec5));

    //ai is working perfectly fine copilot is amazing
}
fn iterator_sum (vec1: Vec<i32>) -> i32{
    let vec1_iter = vec1.iter();
    let total : i32 = vec1_iter.sum();
    total
}
