use rand::Rng;
use std::io;

fn generate_password(length:usize) -> String{

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";
    let charset_bytes = charset.as_bytes();

    let mut rng = rand::thread_rng();

   let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset_bytes[idx] as char
        })
        .collect();

    password
}

fn main(){

    println!("Inserisci la lughezza della password : ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Errore nella lettura dell'input");

    let lenght:usize = match input.trim().parse(){

        Ok(num)=>num,
        Err(_)=>{
            println!("Input non valido , usando lunghezza predefinita di 12.");
            12
        }
    };

    let password = generate_password(lenght);

    println!("Passowrd generata : {}",password);
}