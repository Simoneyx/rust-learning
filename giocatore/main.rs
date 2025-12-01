//prova enum
use std::io;
#[derive(Debug)]
enum Movimenti{
    UP,
    DOWN,
    LEFT,
    RIGHT
}
#[derive(Debug)]
enum Arma {
    SPADA,
    ARCO
}
#[derive(Debug)]
enum StatoGiocatore{
    IDLE,
    MOVE(Movimenti),
    ATTACK(Arma)
}
#[derive(Debug)]
struct Giocatore{
    nome:String,
    vita:i8,
    stato:StatoGiocatore
}

fn main() {
    let _giu: StatoGiocatore = StatoGiocatore::MOVE(Movimenti::DOWN);
    let _attacco: StatoGiocatore=StatoGiocatore::ATTACK(Arma::ARCO);
    let giocatori=[Giocatore { nome:String::from("ciao"), vita: 100, stato: _giu },Giocatore{nome:String::from("Simone"), vita: 100, stato: _attacco }];
    print!("{:?}",giocatori);

}
