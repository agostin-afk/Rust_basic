use std::io;

fn main() {
    let mut number = String::new();
    println!("Digite o número de elementos:");
    io::stdin()
        .read_line(&mut number)
        .expect("Erro ao ler o terminal.");
    let num: usize = number
        .trim()
        .parse()
        .expect("Erro na conversão para número.");
    let mut arr: Vec<i32> = Vec::with_capacity(num);
    let mut x = 0;
    while x < num {
        let mut aux = String::new();
        println!("Digite o elemento {}:", x + 1);
        io::stdin()
            .read_line(&mut aux)
            .expect("Erro ao ler o terminal.");
        let aux: i32 = aux.trim().parse().expect("Erro na conversão para número.");
        arr.push(aux);
        x += 1;
    }
    println!("Array: {:?}", arr);
}
