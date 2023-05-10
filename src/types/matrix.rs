use std::fmt;

pub struct Matrix<K> {
    pub positions: Vec<Vec<K>>,
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
