use std::io;

fn main(){
    println!("informe o numero para x: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Erro!");
    let x:i32 = x.trim().parse().expect("Erro no numero x");
    println!("Informe um numero para y: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Erro!");
    let y:i32 = y.trim().parse().expect("Erro no numero y");
    let sum = x+y;
    let diffe = x - y;
    let mult = x * y;
    let div = x / y;
    let diff_div = x%y;
    println!("x:{x}\ny:{y}\nsum: {sum}\ndiff: {diffe}\nmult: {mult}\ndiv: {div}\ndiffDiv: {diff_div}");
}