fn main(){
    let mut num:i32 = 0;
    let result:i32 = loop{
        num += 1;
        if num == 45{
            break num *2;
        }
    };

    println!("Result: {result}");
}
