use math_lingo::*;

fn main() {
    println!("Evaluate equation \"one plus two\" = {}", math_lingo!(one plus two));
    println!("Evaluate equation \"one plus nine\" = {}", math_lingo!(one plus nine));
    println!("Evaluate equation \"1 plus three\" = {}", math_lingo!(1 plus three));
    println!("Evaluate equation \"nine plus 100\" = {}", math_lingo!(nine plus 100));
    println!("Evaluate equation \"1.11 plus 2.22\" = {}", math_lingo!(1.11 plus 2.22));
    println!("Evaluate equation \"3 time 2.1\" = {}", math_lingo!(3 times 2.1));
    println!("Evaluate equation \"0.5 times 1.24\" = {}", math_lingo!(0.5 time 1.24));
    println!("Evaluate equation \"1 through 3\" = {}", math_lingo!(1 through 3));
    println!("Evaluate equation \"1 devided by 3\" = {}", math_lingo!(1 devided by 3));
}

#[cfg(test)]
mod tests {
    use math_lingo::*;

    #[test]
    fn test_equations() {
        assert_eq!(math_lingo!(one plus two), 1.0 + 2.0);
        assert_eq!(math_lingo!(one plus nine), 1.0 + 9.0);
        assert_eq!(math_lingo!(1 plus three), 1.0 + 3.0);
        assert_eq!(math_lingo!(nine plus 100), 9.0 + 100.0);
        assert_eq!(math_lingo!(1.11 plus 2.22), 1.11 + 2.22);
        assert_eq!(math_lingo!(3 time 2.1), 3.0 * 2.1);
        assert_eq!(math_lingo!(0.5 time 1.24), 0.5 * 1.24);
        assert_eq!(math_lingo!(1 through 3), 1.0 / 3.0);
        assert_eq!(math_lingo!(1 devided by 3), 1.0 / 3.0);
    }
}