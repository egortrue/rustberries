use num::BigInt;

fn main() {
    let a: &BigInt = &(BigInt::from(2) << 21);
    let b: &BigInt = &((BigInt::from(2) << 21) + 1);

    // Вывод результатов
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);
    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);

    println!("===================================");

    // mybigint
    let mut my_a = mybigint::BigInt::new(23);
    let mut my_b = mybigint::BigInt::new(23);
    my_a.set_bit(22, 1);
    my_b.set_bit(22, 1);
    my_b.set_bit(0, 1);

    // Вывод результатов
    println!("Binary A {:?}", &my_a.to_string());
    println!("Binary B {:?}", &my_b.to_string());
    let addition = my_a + my_b;
    println!("Bin Add: {:?}", addition.to_string());
    println!("Int Add: {:?}", addition.to_i128());

    assert_eq!(format!("{}", addition.to_i128()), format!("{}", a + b))
}

mod mybigint {
    use std::{cmp::max, ops::Add};

    #[derive(Debug)]
    pub struct BigInt {
        bits: Vec<u8>,
    }

    impl BigInt {
        pub fn new(size: usize) -> Self {
            BigInt {
                bits: vec![0; size],
            }
        }

        pub fn to_string(&self) -> String {
            self.bits
                .iter()
                .rev()
                .map(|v| if *v == 1 { "1" } else { "0" })
                .collect::<Vec<&str>>()
                .join("")
        }

        pub fn to_i128(&self) -> i128 {
            i128::from_str_radix(self.to_string().as_str(), 2).unwrap()
        }

        /// Устанавливает у числа I-ый бит (bit) в значение (value)
        pub fn set_bit(&mut self, bit: usize, value: u8) {
            assert!(bit < self.bits.len());
            assert!(value == 0 || value == 1);
            self.bits[bit] = value;
        }
    }

    impl Add for BigInt {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let mut result = Vec::new();
            let mut carry = 0;

            let max_len = max(self.bits.len(), other.bits.len());
            for i in 0..max_len {
                let a = *self.bits.get(i).unwrap_or(&0);
                let b = *other.bits.get(i).unwrap_or(&0);

                let sum = a + b + carry;
                result.push(sum % 2);
                carry = sum / 2;
            }

            if carry > 0 {
                result.push(carry);
            }

            BigInt { bits: result }
        }
    }
}
