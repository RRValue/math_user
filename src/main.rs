use math_lingo::*;

fn main() {
    println!("Evaluate equation \"one plus two\" = {}", math_lingo!(one plus two));
    println!("Evaluate equation \"one plus nine\" = {}", math_lingo!(one plus nine));
    println!("Evaluate equation \"1 plus three\" = {}", math_lingo!(1 plus three));
    println!("Evaluate equation \"nine plus 100\" = {}", math_lingo!(nine plus 100));
    println!("Evaluate equation \"1.11 plus 2.22\" = {}", math_lingo!(1.11 plus 2.22));
}
