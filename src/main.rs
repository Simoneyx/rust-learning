//prova 
#![allow(dead_code)]
mod player;
mod weapon;
use crate::player::Giocatore;
use crate::player::movement::Movimenti;
use crate::player::state::StatoGiocatore;
use crate::weapon::Arma;


fn main() {
    let _giu: StatoGiocatore = StatoGiocatore::Move(Movimenti::Down);
    let _attacco: StatoGiocatore=StatoGiocatore::Attack(Arma::Arco);
    let giocatori: Vec<Giocatore>=vec![Giocatore { nome:String::from("ciao"), vita: 100, stato: _giu },Giocatore{nome:String::from("Simone"), vita: 100, stato: _attacco },Giocatore::new()];
    print!("{:?}",&giocatori);
}
