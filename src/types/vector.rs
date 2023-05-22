use std::fmt;

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
}

#[cfg(test)]
mod tests {
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
}
