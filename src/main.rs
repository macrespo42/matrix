use std::io;
mod mandatory;
mod types;
use crate::mandatory::cosine::angle_cos;
use crate::mandatory::linear_combination::linear_combination;
use crate::mandatory::linear_interpolation::lerp;
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
        0 => {
            ex00();
        }
        1 => {
            ex01();
        }
        2 => {
            ex02();
        }
        3 => ex03(),
        4 => {
            println!("\nExercise 04 - Norm");
            println!("-------------------------------------");
            let u = Vector::from(&[0., 0., 0.]);
            ex04(u, ["0.", "0.", "0."]);
            let u = Vector::from(&[1., 2., 3.]);
            ex04(u, ["6.0", "3.7416573", "3."]);
            let u = Vector::from(&[-1., -2.]);
            ex04(u, ["3.0", "2.236067977", "2."]);
            let u = Vector::from(&[0.]);
            ex04(u, ["0.", "0.", "0."]);
            let u = Vector::from(&[1.]);
            ex04(u, ["1.", "1.", "1."]);
            let u = Vector::from(&[0., 0.]);
            ex04(u, ["0.", "0.", "0."]);
            let u = Vector::from(&[1., 0.]);
            ex04(u, ["1.", "1.", "1."]);
            let u = Vector::from(&[2., 1.]);
            ex04(u, ["3.", "2.236067977", "2."]);
            let u = Vector::from(&[4., 2.]);
            ex04(u, ["6.", "4.472135955", "4"]);
            let u = Vector::from(&[-4., -2.]);
            ex04(u, ["6.", "4.472135955", "4"]);
        }
        5 => {
            println!("\nExercise 05 - Cosine\n");
            println!("-------------------------------------");
            let u = Vector::from(&[1., 0.]);
            let v = Vector::from(&[1., 0.]);
            ex05(&u, &v, 1.0);

            let u = Vector::from(&[1., 0.]);
            let v = Vector::from(&[0., 1.]);
            ex05(&u, &v, 0.0);

            let u = Vector::from(&[-1., 1.]);
            let v = Vector::from(&[1., -1.]);
            ex05(&u, &v, -1.0000001);

            let u = Vector::from(&[2., 1.]);
            let v = Vector::from(&[4., 2.]);
            ex05(&u, &v, 1.0);

            let u = Vector::from(&[1., 2., 3.]);
            let v = Vector::from(&[4., 5., 6.]);
            ex05(&u, &v, 0.9746319);
        }
        _ => println!("This exercice does not exist or are not implemented yet 🙄"),
    };
}

fn ex00() {
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
}

fn ex01() {
    let e1 = Vector::from(&[1., 0., 0.]);
    let e2 = Vector::from(&[0., 1., 0.]);
    let e3 = Vector::from(&[0., 0., 1.]);
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
}

fn ex02() {
    println!("lerp(0., 1. 0.)");
    println!("expected: 0");
    println!("got: {}", lerp(0., 1., 0.));
    println!("-------------------------------------");
    println!("lerp(0., 1., 1.)");
    println!("expected: 1");
    println!("got: {}", lerp(0., 1., 1.));
    println!("-------------------------------------");
    println!("lerp(0., 1., 0.5)");
    println!("expected: 0.5");
    println!("got: {}", lerp(0., 1., 0.5));
    println!("-------------------------------------");
    println!("lerp(21., 42., 0.3)");
    println!("expected: 27.3");
    println!("got: {}", lerp(21., 42., 0.3));
    println!("-------------------------------------");
    println!("lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3)");
    println!("expected: [2.6, 1.3]");
    println!(
        "got: {}",
        lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3)
    );
    println!("-------------------------------------");
    println!(
        "lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20.,10.], [30., 40.]]), 0.5)"
    );
    println!("expected:[[11., 5.5], [16.5, 22.]]");
    println!(
        "got: {}",
        lerp(
            Matrix::from(&[&[2., 1.], &[3., 4.]]),
            Matrix::from(&[&[20., 10.], &[30., 40.]]),
            0.5,
        )
    );
}

fn ex03() {
    println!("\nExercise 03 - Dot product\n");
    println!("-------------------------------------");
    let u = Vector::from(&[0., 0.]);
    let v = Vector::from(&[1., 1.]);
    println!("dot product of : {u} and {v}");
    println!("expected: 0");
    println!("got: {}", u.dot(v));
    println!("-------------------------------------");
    let u = Vector::from(&[1., 1.]);
    let v = Vector::from(&[1., 1.]);
    println!("dot product of : {u} and {v}");
    println!("expected: 2");
    println!("got: {}", u.dot(v));
    println!("-------------------------------------");
    let u = Vector::from(&[-1., 6.]);
    let v = Vector::from(&[3., 2.]);
    println!("expected: 9");
    println!("got: {}", u.dot(v));
}

fn ex04<K>(u: Vector<K>, expected: [&str; 3])
where
    K: Clone,
    K: Into<f32>,
    K: std::ops::Mul<K, Output = K>,
    K: std::fmt::Display,
{
    println!("taxicab norm of vector: {u}");
    println!("expected: {}", expected[0]);
    println!("got {}", u.norm_1());
    println!("-------------------------------------");
    println!("euclidian norm of vector: {u}");
    println!("expected: {}", expected[1]);
    println!("got {}", u.norm());
    println!("-------------------------------------");
    println!("l-infinity norm of vector: {u}");
    println!("expected: {}", expected[2]);
    println!("got {}", u.norm_inf());
    println!("-------------------------------------");
}

fn ex05<K>(u: &Vector<K>, v: &Vector<K>, expected: f32)
where
    K: Copy,
    K: Into<f32>,
    K: std::ops::Add<K, Output = K>,
    K: std::ops::Sub<K, Output = K>,
    K: std::ops::Mul<K, Output = K>,
    K: std::fmt::Display,
{
    println!("cosines of {v} and {u}");
    println!("expected: {expected}");
    println!("got: {}", angle_cos::<K>(&u, &v));
}
