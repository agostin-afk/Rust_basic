use std::io;
fn main(){
    let mut code= String::new();
    io::stdin().read_line(&mut code).expect("Erro na hora de ler o terminal");
    let code:i32 = code.trim().parse().expect("Erro na atribuicao");
    let result:i32 = hello_world(code);
    println!("{result}");
    
}
fn hello_world(x:i32) -> i32{
    println!("Hello, World!!");
    println!("Your code: {x}");
    return x;
}