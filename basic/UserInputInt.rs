use std::io;
fn main(){
    println!("Guess a number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Erro!");
    let guess:i32 = guess.trim().parse().expect("Please, insert a valid Number!");
    println!("Your number: {}", guess);

}