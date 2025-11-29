//prova enum
use std::io;
enum Movimenti{
    UP,
    DOWN,
    LEFT,
    RIGHT
}
enum Arma {
    SPADA,
    ARCO
}
enum StatoGiocatore{
    IDLE,
    MOVE(Movimenti),
    ATTACK(Arma)
}

struct Giocatore{
    
}

fn main() {
    let provolone = StatoGiocatore::MOVE(Movimenti::DOWN);
}
