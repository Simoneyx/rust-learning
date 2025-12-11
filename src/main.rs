// vector test
use std::io;
fn main() {
    let mut input:String =String::new();
    io::stdin()
        .read_line(&mut input )
        .expect("error");
    println!("enter some numbers: ");
    let mut v:Vec<&str>  =input.split(' ').collect();
    let last_element:usize = v.len()-1;
    v[last_element]=v[last_element].trim();
    println!("{v:?}")
}
