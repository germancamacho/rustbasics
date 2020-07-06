use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinar el número!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop {
    let mut guess = String::new();
    println!("Indique un número: ");

    io::stdin()
            .read_line(&mut guess)
            .expect("Fallo la lectura de la linea");
    
    let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
    };


        println!("Yo guessed {} ",guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Muy pequeño"),
            Ordering::Greater => println!("Muy grande!"),
            Ordering::Equal => {println!("Ganaste!"); break;}
        }
    
    }


}
