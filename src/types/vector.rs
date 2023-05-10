use std::fmt;

pub struct Vector<K> {
    pub positions: Vec<K>,
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
