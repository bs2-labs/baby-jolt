mod vm;

fn main() {
    println!("Hello, baby jolt!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e() {
        vm::preprocess();
        vm::prove();
        let res = vm::verify();
        assert_eq!(res, 3);
    }
}