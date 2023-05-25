use std::fmt;

#[derive(Clone)]
pub struct Matrix<K> {
    pub positions: Vec<Vec<K>>,
}

impl<K: std::ops::Add<Output = K> + Copy> std::ops::Add for Matrix<K> {
    type Output = Matrix<K>;

    fn add(self, other: Matrix<K>) -> Matrix<K> {
        let mut result = self;
        for (row, &ref other_row) in result.positions.iter_mut().zip(other.positions.iter()) {
            for (column, &other_column) in row.iter_mut().zip(other_row.iter()) {
                *column = *column + other_column;
            }
        }
        result
    }
}

impl<K: std::ops::Sub<Output = K> + Copy> std::ops::Sub for Matrix<K> {
    type Output = Matrix<K>;

    fn sub(self, other: Matrix<K>) -> Matrix<K> {
        let mut result = self;
        for (row, &ref other_row) in result.positions.iter_mut().zip(other.positions.iter()) {
            for (column, &other_column) in row.iter_mut().zip(other_row.iter()) {
                *column = *column - other_column;
            }
        }
        result
    }
}

impl<K: std::ops::Mul<f32, Output = K> + Copy> std::ops::Mul<f32> for Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, scalar: f32) -> Matrix<K> {
        let mut result = self;
        for row in result.positions.iter_mut() {
            for value in row.iter_mut() {
                *value = *value * scalar;
            }
        }
        result
    }
}

impl<
        K: Copy + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::Mul<Output = K>,
    > Matrix<K>
{
    fn column_size(&self) -> usize {
        self.positions[0].len()
    }

    fn row_size(&self) -> usize {
        self.positions.len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.column_size(), self.row_size())
    }

    pub fn is_square(&self) -> bool {
        self.positions.len() == self.positions[0].len()
    }

    pub fn reshape_to_vector(&self) -> Vec<K> {
        let mut returned_vector: Vec<K> = Vec::new();
        for row in self.positions.iter() {
            returned_vector.extend(row.clone());
        }
        returned_vector
    }

    pub fn from(matrix: &[&[K]]) -> Self {
        let mut positions = Vec::with_capacity(matrix.len());
        for row in matrix {
            positions.push(row.to_vec());
        }
        Matrix { positions }
    }

    fn same_size(&self, v: &Matrix<K>) {
        if self.positions.len() != v.positions.len() {
            panic!("Matrix do not have same size!");
        }
        for (index, point) in self.positions.iter().enumerate() {
            if point.len() != v.positions[index].len() {
                panic!("Matrix rows do not have same size!");
            }
        }
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        self.same_size(v);
        for (row_index, row) in v.positions.iter().enumerate() {
            for (col_index, column) in row.iter().enumerate() {
                self.positions[row_index][col_index] =
                    self.positions[row_index][col_index] + column.clone();
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        self.same_size(v);
        for (row_index, row) in v.positions.iter().enumerate() {
            for (col_index, column) in row.iter().enumerate() {
                self.positions[row_index][col_index] =
                    self.positions[row_index][col_index] - column.clone();
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for row in self.positions.iter_mut() {
            for column in row.iter_mut() {
                *column = *column * a;
            }
        }
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n").expect("can't write in stdout");
        for row in self.positions.iter() {
            write!(f, "[").expect("can't write in stdout");
            for (index, point) in row.iter().enumerate() {
                write!(f, "{}", point).expect("can't write in stdout");
                if index < row.len() - 1 {
                    write!(f, ",").expect("can't write in stdout");
                }
            }
            write!(f, "]\n").expect("can't write in stdout");
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_add() {
        let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
        u.add(&v);
        assert_eq!(Vec::from([8.0, 6.0]), u.positions[0]);
        assert_eq!(Vec::from([1.0, 6.0]), u.positions[1]);
    }

    #[test]
    fn matrix_sub() {
        let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
        u.sub(&v);
        assert_eq!(Vec::from([-6.0, -2.0]), u.positions[0]);
        assert_eq!(Vec::from([5.0, 2.0]), u.positions[1]);
    }

    #[test]
    fn matrix_scale() {
        let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        u.scl(2.);
        println!("{}", u);
        assert_eq!(Vec::from([2.0, 4.0]), u.positions[0]);
        assert_eq!(Vec::from([6.0, 8.0]), u.positions[1]);
    }
}
