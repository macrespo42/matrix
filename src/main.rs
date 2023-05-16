use std::{io, println};
mod mandatory;
mod types;
use crate::mandatory::linear_combination::linear_combination;
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
        1 => ex01(),
        _ => println!("This exercice does not exist or are not implemented yet 🙄"),
    };
}

fn ex00() {
    println!("\nExercise 00 - Add, Subtract and Scale\n");
    println!("-------------------------------------");
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.add(&v);
    println!("Add [2, 3] to [5, 7]");
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
    let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
    let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
    println!("Add {u} \nwith\n{v}");
    u.add(&v);
    println!("expected: [\n[8,6]\n[1,6]\n]\n");
    println!("got: {u}");
    println!("-------------------------------------");
    let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
    let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
    println!("Subtract {u} \nwith\n{v}");
    u.sub(&v);
    println!("expected: [\n[-6,-2]\n[5,2]\n]\n");
    println!("got: {u}");
    println!("-------------------------------------");
    let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
    println!("Scale {u}\nby 2");
    u.scl(2.);
    println!("expected [\n[2, 4]\n[6,8]\n]\n]");
    println!("got: {u}");
    println!("-------------------------------------");
}

fn ex01() {
    let e1 = Vector::from(&[1., 0., 0.]);
    let e2 = Vector::from(&[0., 1., 0.]);
    let e3 = Vector::from(&[0., 0., 1.]);
    println!("\nExercise 01 - Linear combination\n");
    println!("-------------------------------------");
    println!("expected: [10, -2, 0.5]");
    println!(
        "got: {}",
        linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5])
    );
    println!("-------------------------------------");

    let v1 = Vector::from(&[1., 2., 3.]);
    let v2 = Vector::from(&[0., 10., -100.]);

    println!("expected: [10, 0, 230]");
    println!("got: {}", linear_combination::<f32>(&[v1, v2], &[10., -2.]));
    println!("-------------------------------------");
}
