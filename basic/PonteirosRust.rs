use std::io;
fn main(){
    println!("Insert your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Erro ao ler o terminal.");
    let name = name.trim();
    ola(&name.to_string());
    println!("Seu nome é mesmo {name} ?")
}
fn ola(stg: &String){
    println!("Olá, {stg}");

}
