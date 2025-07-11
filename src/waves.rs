use std::result::Result;
use std::ops::Sub;
use std::ops::Div;

#[derive(Debug, PartialEq)]
pub struct Wave {
    values: Vec<i8>,
}

impl Wave {
    pub fn new(values: Vec<i8>) -> Self {
        return Wave { values };
    }

    pub fn merge(&self, other: &Self) -> Self {
        let new_values = [&self.values[..], &other.values[..]].concat();
        return Wave::new(new_values);
    }

    pub fn overlap(&self, other: &Self) -> Result<Self, &'static str> {
        let len_self = self.values.len();
        let len_other = other.values.len();
        if len_self != len_other {
            return Err("values don't have the same length");
        }
        return Ok(Wave::new(vec![0]));
    }

    pub fn sample_left_pad(self, ammount: i32) -> Self {
        return Wave::new(vec![ammount as i8]);
    }
}

impl Sub for Wave {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        return self.merge(&other);
    }
}

impl Div for Wave {
    type Output = Result<Self, &'static str>;

    fn div(self, other: Self) -> Self::Output {
        return self.overlap(&other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenation() {
        let x: Wave = Wave::new(vec![1, 2, 3]);
        let y: Wave = Wave::new(vec![4, 5, 6]);
        let z = x.merge(&y);
        assert_eq!(
            z,
            Wave::new(vec![1, 2, 3, 4, 5, 6])
        );
        let w = x - y;
        assert_eq!(z, w);
    }

    #[test]
    fn test_overlapping() {
        let x: Wave = Wave::new(vec![0]);
        let y: Wave = Wave::new(vec![0]);
        let z: Wave = x.overlap(&y).unwrap();
        assert_eq!(z, Wave::new(vec![0]));
        let w = x / y;
        assert_eq!(z, w.unwrap());
    }
}

