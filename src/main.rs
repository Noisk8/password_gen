use std::char::from_u32;
use rand;
use rand::{Rng, thread_rng};


fn main() {
    
    let password_length: i32 =15;

    let mut resultado: String =  String::new();


    for _ in 0..password_length{

        let number :u32 =thread_rng().gen_range(48..122);
        let ch :char = from_u32(number).unwrap();
        resultado.push(ch);
    }
    println!("{}", resultado);
}

