use std::fmt::{Display, Write};
use std::ops::{Add, Mul, Sub};
use crate::util::u64_to_char_unchecked;

#[derive(Clone, Copy)]
pub struct Even {
    pub data: u64
}

impl Add for Even {
    type Output = Even;
    fn add(self, rhs: Even) -> Even { 
        return Even { data: self.data + rhs.data };
    }
}

impl Sub for Even {
    type Output = Even;
    fn sub(self, rhs: Self) -> Even {
        return Even { data: self.data - rhs.data };
    }
}

impl Mul for Even {
    type Output = Even;
    fn mul(self, rhs: Self) -> Even {
        return Even { data: self.data * rhs.data * 2 };
    }
}

impl TryFrom<u64> for Even {
    type Error = ();
    fn try_from(num: u64) -> Result<Even, ()> { 
        if num % 2 == 1 {
            return Err(()); 
        }
        return Ok(
            Even {
                data: num / 2
            }
        );
    }
}

impl Display for Even {
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
            let n = (self.data * 2 / k) % 10;
            f.write_char(u64_to_char_unchecked(n))?;
            k /= 10;
        }
        
        return Ok(());
    }
}