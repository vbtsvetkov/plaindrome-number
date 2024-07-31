fn main() {
    println!("Hello, world!");
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    
    if x < 10 {
        return true;
    }
    let mut digits: Vec<u8> = vec![];
    let mut value = x;

    while value != 0 {
        digits.push((value % 10) as u8);
        value = value / 10;
    }
    
    let mut min: usize = 0;
    let mut max = digits.len() - 1;

    while min <= max {
        if digits[min] != digits[max] {
            return false;
        }

        min+=1;
        max-=1;
    }

    true
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn pass_121_true() {
        assert_eq!(is_palindrome(121), true);    
    }

    #[test]
    fn pass_minus_121_false() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn pass_10_false() {
        assert_eq!(is_palindrome(10), false);
    }
}

