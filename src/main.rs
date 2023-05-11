use std::io;
mod types;
use crate::types::{Matrix, Vector};

fn main() {
    println!("Welcome to the matrix ! Choose the exercice you want to check 💊:");
    println!("Available :");
    for n in 0..15 {
        println!(" - {n}")
    }

    let mut exercice = String::from("");

    io::stdin()
        .read_line(&mut exercice)
        .expect("Failed to read line");

    let exercice: u32 = match exercice.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Bad user input"),
    };

    match exercice {
        0 => ex00(),
        _ => println!("This exercice does not exist or are not implemented yet 🙄"),
    };
    // let mut u = Vector::from(&[2., 3.]);
    // u.scl(2.);
    // println!("{}", u);
    // // [4.0]
    // // [6.0]
    // let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
    // let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
    // u.add(&v);
    // println!("{}", u);
    // // [8.0, 6.0]
    // // [1.0, 6.0]
    // let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
    // let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
    // u.sub(&v);
    // println!("{}", u);
    // // [-6.0, -2.0]
    // // [5.0, 2.0]
    // let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
    // u.scl(2.);
    // println!("{}", u);
    // // [2.0, 4.0]
    // // [6.0, 8.0]
}

fn ex00() {
    println!("\nExercise 00 - Add, Subtract and Scale\n");
    println!("-------------------------------------");
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.add(&v);
    println!("Add [2, 3] with [5, 7]");
    println!("expected: [7, 10]");
    println!("got: {u}");
    println!("-------------------------------------");
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.sub(&v);
    println!("Subtract [2, 3], with [5, 7]");
    println!("expected: [-3, -4]");
    println!("got: {u}");
    println!("-------------------------------------");
    let mut u = Vector::from(&[2., 3.]);
    u.scl(2.);
    println!("Scale [2, 3] by 2");
    println!("expected: [4, 6]");
    println!("got: {u}");
    println!("-------------------------------------");
}
