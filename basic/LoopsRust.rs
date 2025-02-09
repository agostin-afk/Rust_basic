use std::io;
fn main(){
    println!("Insert a number: ");
    let answer:u32 = 45;
    let mut attempt:u32 = 0;
    loop{
        let mut num=String::new();
        io::stdin().read_line(&mut num).expect("Erro ao ler o arquivo!");
        let num:u32 = num.trim().parse().expect("Erro ao converter");
        if num == answer{
            println!("You are right!");
            break
        }else if num < answer {
            println!("More!");
        }
        else{
            println!("Less!");
        }
        attempt +=1;
    }
    println!("attempt: {attempt}");
}