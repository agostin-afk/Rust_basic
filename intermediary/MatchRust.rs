enum Coin{
    Real(f32),
    Dolar(f32),
    Euro(f32),
}
fn return_value(x: f32, cota: f32) -> f32{
    x/cota
}
fn main(){
    let din=Coin::Real(100.0);
    match din{
        Coin::Real(x) => {
            println!("Seu valor em dolar: {}", return_value(x, 5.0));
        }
        Coin::Dolar(x) => {
            println!("Seu valor em dolar: {}", return_value(x, x));
        }
        Coin::Euro(x) => {
            println!("Seu valor em dolar: {}", return_value(x, 1.2));            
        }

    }
}