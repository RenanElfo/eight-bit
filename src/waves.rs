use std::ops::Div;
use std::ops::Sub;
use std::result::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wave {
    values: Vec<i16>,
    amplitude: f64,
    sampling_frequency: f64,
}

impl Wave {
    pub fn new(values: Vec<i16>) -> Self {
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

    pub fn sample_right_pad(&mut self, ammount: usize) {
        self.values.resize(self.values.len() + ammount, 0);
    }

    pub fn sample_left_pad(&mut self, ammount: usize) {
        self.sample_right_pad(ammount);
        self.values.rotate_right(ammount);
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
        assert_eq!(z, Wave::new(vec![1, 2, 3, 4, 5, 6]));
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

    #[test]
    fn test_right_padding() {
        let mut x: Wave = Wave::new(vec![1, 2, 3]);
        x.sample_right_pad(2);
        assert_eq!(x, Wave::new(vec![1, 2, 3, 0, 0]));
    }

    #[test]
    fn test_left_padding() {
        let mut x: Wave = Wave::new(vec![1, 2, 3]);
        x.sample_left_pad(2);
        assert_eq!(x, Wave::new(vec![0, 0, 1, 2, 3]));
    }
}
