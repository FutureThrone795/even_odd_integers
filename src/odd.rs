use std::{fmt::{Display, Write}, ops::Add};
use crate::util::u64_to_char_unchecked;
use crate::even::Even;

#[derive(Clone, Copy)]
pub struct Odd {
    pub data: u64
}

impl Add for Odd {
    type Output = Even;
    fn add(self, other: Odd) -> Even { 
        return Even { data: self.data + other.data + 1 }
    }
}

impl TryFrom<u64> for Odd {
    type Error = ();
    fn try_from(num: u64) -> Result<Odd, ()> { 
        if num % 2 == 0 {
            return Err(()); 
        }
        return Ok(
            Odd {
                data: num / 2
            }
        );
    }
}

impl Display for Odd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data == 0 {
            f.write_char('0')?;
            return Ok(());
        }

        let mut k = 1;
        while k * 5 <= self.data {
            k *= 10;
        }
        while k > 0 {
            let n = ((self.data * 2 + 1) / k) % 10;
            f.write_char(u64_to_char_unchecked(n))?;
            k /= 10;
        }
        
        return Ok(());
    }
}