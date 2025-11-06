//primo programma per vedere un po la sintassi di rust
use std::env;
use std::io;

fn main() {
    let mut _s=String::new();
    let args:Vec<String> = env::args().collect();
    let mut _bho: &str;
    println!("{:?}",args);
    if args.len() > 1 {
        for _bho in args{
            print!("{} ",_bho);
        }
        
    }
    else{
        println!("scrivi qualcosa:");
        io::stdin()
            .read_line(&mut _s)
            .expect("non hai scritto nulla");
        println!("{}",_s);
    }
    
}
