pub fn add(left: usize, right: usize) -> usize {
    left + right
}


//*----*//
//* This is not very nice rusty  *//
//* *//
macro_rules! float64 {
    ($arg:expr) => {
        //check if has . in the string
        if $arg.contains(".") {
            $arg.parse::<f64>().unwrap()
        } else {
            $arg.parse::<i64>().unwrap() as f64
        }
    };
}
pub fn float64(arg:i64)->f64{
    //do g
    if arg.to_string().contains(".") {
       return arg.to_string().parse::<f64>().unwrap();
    } else {
        return arg as f64
    }
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
