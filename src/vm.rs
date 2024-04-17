pub fn preprocess() -> u32 {
    print!("Preprocess");
    1
}

pub fn prove() -> u32 {
    print!("Prove");
    2
}

pub fn verify() -> u32 {
    print!("verify");
    3
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e() {
        preprocess();
        prove();
        let res = verify();
        assert_eq!(res, 3);
    }
}