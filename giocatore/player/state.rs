use crate::player::movement::Movimenti;
use crate::weapon::Arma;

#[derive(Debug)]
pub enum StatoGiocatore{
    Idle,
    Move(Movimenti),
    Attack(Arma)
}