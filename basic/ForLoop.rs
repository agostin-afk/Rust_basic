use std::io;

fn main(){
    let mut num= String::new();
    io::stdin().read_line(&mut num).expect("Erro ao ler o terminal.");
    let num:usize = num.trim().parse().expect("Erro ao trocar o tipo.");
    let mut arr:Vec<usize> = Vec::with_capacity(num);
    for number in 1..=num {
        // let mut num_ :i32 = number.parse();
        arr.push(number);
    }
    println!("Array: {:?}", arr);
}