//prova struct
use std::io;

#[derive(Debug)]
struct Rectangle{
    base:f64,
    altezza:f64,
}
impl Rectangle {
    fn area(&self) -> f64 {
        self.base * self.altezza
    }
    fn perimetro(&self) -> f64 {
        2.0 * (self.base + self.altezza)
    }
}
fn main() {
    let mut base = String::new();
    let mut altezza = String::new();
    let errore= String::from("Errore di input");

    println!("inserisci base rettangolo:");
    io::stdin().read_line(&mut base).expect(errore.as_str());
    println!("inserisci altezza rettangolo:");
    io::stdin().read_line(&mut altezza).expect(errore.as_str());

    let rect1= Rectangle{
        base: base.trim().parse().expect(errore.as_str()),
        altezza: altezza.trim().parse().expect(errore.as_str()),
    };

    println!("L'area del rettangolo ({:?})e': {},il perimetro e':{}",rect1, rect1.area(),rect1.perimetro());
}
