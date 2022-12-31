// Just some code I wrote when watching https://youtu.be/gwVmE8gcqrg?t=127

const PRINT_DOUBLES: bool = true;
const PRINT_DOUBLE_AND_NOT_SUM: bool = true;
const PRINT_LAST_DIGIT_OF_SUM: bool = true;

fn main() {
    println!("Result: {}", luhn(String::from("453989463779694")));
}

fn luhn(s: String) -> u8 {
    let mut digits = {
        let bytes = s.as_bytes().to_owned();
        let mut digits: Vec<u8> = Vec::new();
        digits.reserve(bytes.len());
        for b in bytes {
            digits.push(byte_to_digit(b));
        }
        digits
    };
    for (index, digit) in digits.iter_mut().rev().enumerate() {
        // going through every other index
        if index % 2 == 0 {
            let mut double = (*digit as u64) * 2;
            if PRINT_DOUBLES {
                println!("Doubled {} to get {}", *digit, double);
            }
            if double >= 10 {
                double = digit_sum(double);
            }
            *digit = double as u8;
        }
    }
    let sum: u8 = digits.iter().sum();
    if PRINT_DOUBLE_AND_NOT_SUM {
        println!("Got sum of all digits, doubled and not: {}", sum);
    }
    let sum_digits = num_to_digits_vec(sum as u64);
    let last_sum_digit = sum_digits[sum_digits.len() - 1];
    if PRINT_LAST_DIGIT_OF_SUM {
        println!("Last digit of sum: {}", last_sum_digit)
    }
    if last_sum_digit == 0 {
        0
    } else {
        10 - last_sum_digit
    }
}

fn byte_to_digit(b: u8) -> u8 {
    b - 48
}

fn num_to_digits_vec(n: u64) -> Vec<u8> {
    let s = n.to_string();
    let v1 = s.as_bytes().to_vec();
    let mut v2: Vec<u8> = Vec::new();
    v2.reserve(v1.len());
    for b in v1 {
        v2.push(byte_to_digit(b));
    }
    v2
}

fn digit_sum(n: u64) -> u64 {
    let sum: u8 = num_to_digits_vec(n).iter().sum();
    sum as u64
}

/*fn digit_vec_to_u64(v: Vec<u8>) -> Option<u64> {
    match String::from_utf8(v) {
        Ok(s) => match s.parse() {
            Ok(n) => Some(n),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
*/

#[test]
fn test() {
    assert_eq!(luhn(String::from("453989463779694")), 5);
}
