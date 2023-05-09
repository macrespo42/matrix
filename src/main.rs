mod types;
use crate::types::{Matrix, Vector};

fn main() {
    let vec1 = vec![1., 2., 3., 4., 5., 6.];
    let mut my_vec: Vec<Vec<f32>> = Vec::new();

    for _i in 0..3 {
        let row_vec = vec![2.; 2];
        my_vec.push(row_vec);
    }
    let ma = Matrix::<f32> { positions: my_vec };
    let ve = Vector::<f32> { positions: vec1 };

    println!("Size of vector : {}", ve.size());
    println!("vector: {ve}");

    println!("--------------------");

    println!("Size of matrix: {} : {}", ma.shape().0, ma.shape().1);
    println!("matrix: \n{}", ma);
    if ma.is_square() {
        println!("matrix is square");
    } else {
        println!("matrix is not square");
    }

    let reshaped_vec = Vector::<f32> {
        positions: ma.reshape_to_vector(),
    };
    println!("Matrix reshaped to vector : {reshaped_vec}");

    let reshaped_matrix = Matrix::<f32> {
        positions: ve.reshape_to_matrix(2, 3),
    };
    println!("Vector reshape to matrix {reshaped_matrix}");
    println!("--------------------------------------------");
}
