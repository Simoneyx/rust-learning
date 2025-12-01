//prova enum
#[derive(Debug)]
enum Movimenti{
    Up,
    Down,
    Left,
    Right
}
#[derive(Debug)]
enum Arma {
    Spada,
    Arco
}
#[derive(Debug)]
enum StatoGiocatore{
    Idle,
    Move(Movimenti),
    Attack(Arma)
}
#[derive(Debug)]
struct Giocatore{
    nome:String,
    vita:i8,
    stato:StatoGiocatore
}

fn main() {
    let _giu: StatoGiocatore = StatoGiocatore::Move(Movimenti::Down);
    let _attacco: StatoGiocatore=StatoGiocatore::Attack(Arma::Arco);
    let giocatori=[Giocatore { nome:String::from("ciao"), vita: 100, stato: _giu },Giocatore{nome:String::from("Simone"), vita: 100, stato: _attacco }];
    print!("{:?}",&giocatori);
}
