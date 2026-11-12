use std::ops::{Add, Mul, Sub};

use crate::even::Even;
use crate::odd::Odd;

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Add<Even> for Odd {
    type Output = Odd;

    fn add(self, rhs: Even) -> Odd { 
        return Odd { data: self.data + rhs.data };
    }
}

impl Add<Odd> for Even {
    type Output = Odd;

    fn add (self, rhs: Odd) -> Odd {
        return rhs + self;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
 
impl Sub<Even> for Odd {
    type Output = Odd;

    fn sub(self, rhs: Even) -> Odd {
        return Odd { data: self.data - rhs.data }
    }
}

impl Sub<Odd> for Even {
    type Output = Odd;

    fn sub(self, rhs: Odd) -> Odd {
        return Odd { data: self.data - rhs.data }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Mul<Even> for Odd {
    type Output = Even;

    fn mul(self, rhs: Even) -> Even {
        return Even { data: self.data * rhs.data * 2 + rhs.data }
    }
}

impl Mul<Odd> for Even {
    type Output = Even;

    fn mul(self, rhs: Odd) -> Even {
        return rhs * self;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////