// vector & iterator test
use std::io;
fn main() {
    const E: &str = "Error";
    let mut input: String = String::new();
    println!("enter some numbers: ");
    io::stdin().read_line(&mut input).expect(E);

    let v: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.trim().parse().expect(E))
        .collect();

    println!("max:{}", &v.iter().max().unwrap());
    println!("min:{}", &v.iter().min().unwrap());
    println!("avg:{}", v.iter().sum::<i64>() as f64 / v.len() as f64);
    println!("{:?}", &v);
}
