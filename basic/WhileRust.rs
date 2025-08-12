use std::io;
fn main() {
    println!("Informe um numero: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Erro ao ler o terminal.");
    let num: i32 = num.trim().parse().expect("Erro ao converter");
    let mut x = 0;
    while x < num {
        println!("Number_x: {x}\nNumber: {num}");
        x += 1;
    }
    println!("Fim :)")
}
