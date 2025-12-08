pub mod movement;
pub mod state;

use state::StatoGiocatore;
#[derive(Debug)]
pub struct Giocatore{
    pub nome:String,
    pub vita:i8,
    pub stato:StatoGiocatore
}