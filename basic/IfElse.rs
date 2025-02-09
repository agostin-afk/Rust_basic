use std::io;
fn main(){
    println!("Inser a number: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Erro na leitura.");
    let num:i32 = num.trim().parse().expect("Erro na atribuição");
    if num > 10 {
        println!("Your number is greater than 10.");
    } 
    else if num < 10{
        println!("this is a small number.");
    }
    else{
        println!("Your number is 10!");
    }
}