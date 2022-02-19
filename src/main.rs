pub mod solutions;

fn main() {
    println!("Hello, LeetCode!");
    use solutions::problem5::Problem5;
    dbg!(solutions::problem5::ManacherAlgorithm::longest_palindrome(String::from("weassktkssaxcxw")));
}
