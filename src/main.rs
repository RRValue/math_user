use math_lingo::*;

fn main() {
    println!("Evaluate equation \"one plus two\" = {}", math_lingo!(one plus two));
    println!("Evaluate equation \"one plus nine\" = {}", math_lingo!(one plus nine));
    println!("Evaluate equation \"1 plus three\" = {}", math_lingo!(1 plus three));
    println!("Evaluate equation \"nine plus 100\" = {}", math_lingo!(nine plus 100));
    println!("Evaluate equation \"1.11 plus 2.22\" = {}", math_lingo!(1.11 plus 2.22));
    println!("Evaluate equation \"1 through 3\" = {}", math_lingo!(1 through 3));
}

#[cfg(test)]
mod tests {
    use math_lingo::*;

    #[test]
    fn test_equations() {
        assert_eq!(math_lingo!(one plus two), 3.0);
        assert_eq!(math_lingo!(one plus nine), 10.0);
        assert_eq!(math_lingo!(1 plus three), 4.0);
        assert_eq!(math_lingo!(nine plus 100), 109.0);
        assert_eq!(math_lingo!(1.11 plus 2.22), 3.33);
        assert_eq!(math_lingo!(1 through 3), 1.0 / 3.0);
    }
}