use std::io;

fn main() {
    let mut notas_1: Vec<i32> = vec::new();
    let mut numeros = vec![45, 32, 12, 41, 0];
    println!("Informe numeros para adicionar no vetor: {:?}", numeros);
    
    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Erro ao ler o terminal.");
        let num = num.trim();
        
        if num == "-1" {
            break;
        }
        let num: i32 = num.parse().expect("Erro ao converter o tipo");
        numeros.push(num);
        println!("novo vector: {:?}", numeros);
        println!("Novo numero: ");
    }
}