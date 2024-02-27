pub fn add(left: usize, right: usize) -> usize {
    left + right
}


macro_rules! check_divide_by_zero {
    () => {
        
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
