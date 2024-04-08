fn main() {
    println!("This shouldnt be executed during test")
}

#[cfg(test)]
mod test {
   #[test]
    fn passed_test() {
        assert_eq!(1 + 5, 6, "passed test {} + {} = {}", 1, 5, 6);
    }

    #[test]
    fn failed_test() {
        assert!(false, "This test failed");
    }
}
