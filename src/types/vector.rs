use std::{fmt, panic, println, result};

#[derive(Clone)]
pub struct Vector<K> {
    pub positions: Vec<K>,
}

impl<K: std::ops::Add<Output = K> + Copy> std::ops::Add for Vector<K> {
    type Output = Vector<K>;

    fn add(self, other: Vector<K>) -> Vector<K> {
        let mut result = self;
        for (point, &other_point) in result.positions.iter_mut().zip(other.positions.iter()) {
            *point = *point + other_point;
        }
        result
    }
}

impl<K: std::ops::Sub<Output = K> + Copy> std::ops::Sub for Vector<K> {
    type Output = Vector<K>;

    fn sub(self, other: Vector<K>) -> Vector<K> {
        let mut result = self;
        for (point, &other_point) in result.positions.iter_mut().zip(other.positions.iter()) {
            *point = *point - other_point;
        }
        result
    }
}

impl<K: std::ops::Mul<f32, Output = K> + Copy> std::ops::Mul<f32> for Vector<K> {
    type Output = Vector<K>;

    fn mul(self, scalar: f32) -> Vector<K> {
        let mut result = self;
        for point in result.positions.iter_mut() {
            *point = *point * scalar;
        }
        result
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[").expect("can't write in stdout");
        for (i, pos) in self.positions.iter().enumerate() {
            if i != 0 {
                write!(f, ", ",).expect("can't write in stdout");
            }
            write!(f, "{}", pos).expect("can't write in stdout");
        }
        write!(f, "]")
    }
}

impl<
        K: Copy + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::Mul<Output = K>,
    > Vector<K>
{
    pub fn size(&self) -> usize {
        self.positions.len()
    }

    pub fn reshape_to_matrix(&self, col_size: usize, row_size: usize) -> Vec<Vec<K>> {
        let mut matrix: Vec<Vec<K>> = Vec::new();

        let mut row_inserted = 0;
        let mut start_slice: usize = 0;
        let mut end_slice: usize = col_size;

        while row_inserted < row_size {
            let mut row: Vec<K> = Vec::new();
            row.extend_from_slice(&self.positions[start_slice..end_slice]);
            start_slice += col_size;
            end_slice += col_size;
            row_inserted += 1;
            matrix.push(row);
        }
        matrix
    }

    pub fn from(vec: &[K]) -> Self {
        Vector {
            positions: vec.to_vec(),
        }
    }

    fn is_same_size(&self, v: &Vector<K>) {
        if self.positions.len() != v.positions.len() {
            panic!("Can't add two Vectors of different sizes");
        }
    }

    pub fn add(&mut self, v: &Vector<K>) {
        self.is_same_size(v);
        for (index, points) in v.positions.iter().enumerate() {
            self.positions[index] = self.positions[index] + points.clone();
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        self.is_same_size(v);
        for (index, points) in v.positions.iter().enumerate() {
            self.positions[index] = self.positions[index] - points.clone();
        }
    }

    pub fn scl(&mut self, a: K) {
        for point in self.positions.iter_mut() {
            *point = *point * a;
        }
    }

    pub fn dot(&self, v: Vector<K>) -> K {
        if self.positions.len() != v.positions.len() {
            panic!("vectors must have the same size for dot product");
        }
        let mut result = self.positions[0] * v.positions[0];
        for i in 1..self.positions.len() {
            result = result + (self.positions[i] * v.positions[i])
        }
        result
    }
}

impl<K: Clone + std::ops::Mul<K, Output = K> + Into<f32> + std::fmt::Display> Vector<K> {
    pub fn norm_1(&self) -> f32 {
        let mut result = self.abs(self.positions[0].clone());
        for index in 1..self.positions.len() {
            result += self.abs(self.positions[index].clone());
        }
        result
    }

    pub fn norm(&self) -> f32 {
        let mut result: f32 = self.abs(self.positions[0].clone()).powf(2.);
        for index in 1..self.positions.len() {
            result += self.abs(self.positions[index].clone()).powf(2.);
        }
        // square root of result Newton-Raphson algorithm
        if result < 0. {
            return f32::NAN;
        }

        let mut guess = result;
        let mut prev_guess = 0.;
        let mut guess_result = prev_guess - guess;

        if guess_result < 0. {
            guess_result *= -1.;
        }
        while guess_result > 0.00000001 {
            prev_guess = guess;
            guess = 0.5 * (guess + result / guess);
            guess_result = prev_guess - guess;
            if guess_result < 0. {
                guess_result *= -1.;
            }
        }

        guess
    }

    pub fn norm_inf(&self) -> f32 {
        let mut result: f32 = self.abs(self.positions[0].clone());
        for index in 1..self.positions.len() {
            let elt: f32 = self.abs(self.positions[index].clone());
            if elt > result {
                result = elt;
            }
        }
        result
    }

    fn abs(&self, val: K) -> f32 {
        if val.clone().into() < 0. {
            return val.clone().into() * -1.;
        }
        val.into()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn vector_add() {
        let mut u = Vector::from(&[2., 3.]);
        let v = Vector::from(&[5., 7.]);
        u.add(&v);
        assert_eq!(7.0, u.positions[0]);
        assert_eq!(10.0, u.positions[1]);
    }

    #[test]
    fn vector_sub() {
        let mut u = Vector::from(&[2., 3.]);
        let v = Vector::from(&[5., 7.]);
        u.sub(&v);
        assert_eq!(-3.0, u.positions[0]);
        assert_eq!(-4.0, u.positions[1]);
    }

    #[test]
    fn vector_scale() {
        let mut u = Vector::from(&[2., 3.]);
        u.scl(2.);
        assert_eq!(4.0, u.positions[0]);
        assert_eq!(6.0, u.positions[1]);
    }

    #[test]
    fn dot_basics() {
        let u = Vector::from(&[0., 0.]);
        let v = Vector::from(&[1., 1.]);
        assert_eq!(0.0, u.dot(v));
        let u = Vector::from(&[1., 1.]);
        let v = Vector::from(&[1., 1.]);
        assert_eq!(2., u.dot(v));
        let u = Vector::from(&[-1., 6.]);
        let v = Vector::from(&[3., 2.]);
        assert_eq!(9., u.dot(v));
    }

    #[test]
    fn norms_test_basics() {
        let u = Vector::from(&[0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.0);
        assert_eq!(u.norm(), 0.0);
        assert_eq!(u.norm_inf(), 0.0);

        let u = Vector::from(&[1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.0);
        assert_eq!(u.norm(), 3.7416573);
        assert_eq!(u.norm_inf(), 3.);

        let u = Vector::from(&[-1., -2.]);
        assert_eq!(u.norm_1(), 3.0);
        assert_eq!(u.norm(), 2.236067977);
        assert_eq!(u.norm_inf(), 2.);
    }

    #[test]
    fn norms_test_hards() {
        let u = Vector::from(&[0.]);
        assert_eq!(u.norm_1(), 0.);
        assert_eq!(u.norm(), 0.);
        assert_eq!(u.norm_inf(), 0.);

        let u = Vector::from(&[1.]);
        assert_eq!(u.norm_1(), 1.);
        assert_eq!(u.norm(), 1.);
        assert_eq!(u.norm_inf(), 1.);

        let u = Vector::from(&[0., 0.]);
        assert_eq!(u.norm_1(), 0.);
        assert_eq!(u.norm(), 0.);
        assert_eq!(u.norm_inf(), 0.);

        let u = Vector::from(&[1., 0.]);
        assert_eq!(u.norm_1(), 1.);
        assert_eq!(u.norm(), 1.);
        assert_eq!(u.norm_inf(), 1.);

        let u = Vector::from(&[2., 1.]);
        assert_eq!(u.norm_1(), 3.);
        assert_eq!(u.norm(), 2.236067977);
        assert_eq!(u.norm_inf(), 2.);

        let u = Vector::from(&[4., 2.]);
        assert_eq!(u.norm_1(), 6.);
        assert_eq!(u.norm(), 4.472135955);
        assert_eq!(u.norm_inf(), 4.);

        let u = Vector::from(&[-4., -2.]);
        assert_eq!(u.norm_1(), 6.);
        assert_eq!(u.norm(), 4.472135955);
        assert_eq!(u.norm_inf(), 4.);
    }
}
