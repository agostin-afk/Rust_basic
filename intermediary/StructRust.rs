struct User{
    active: bool,
    username: String,
    email: String,
    pssw: u32,
    apresentacao: fn(&str) -> &str,
}
fn hello(name: &str) -> &str {
    println!("Olá, {name}!");
    name
}
fn main(){
    let user1 = User {
        active: true,
        username: String::from("Agosto"),
        email: String::from("agosto@gmail.com"),
        pssw: 454545,
        apresentacao: hello,
    };
    println!("Your name {}", user1.username);
    println!("{}", (user1.apresentacao)(&user1.username));
}