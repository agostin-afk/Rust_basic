fn main() {
    let x = 45;
    println!("x: {}", x);
    // sem o mut - com erro
    // let y = 4;
    // com o mut - sem erro
    let mut y = 4;
    println!("old y: {y}");
    y = 3;
    println!("new y: {}", y);
}
