use std::thread;
use std::time::Duration;
// o codigo abaixo está encadeado.
//fn main(){
//    thread::spawn(|| {
//        for i in 1..10{
//            println!("oi numero {i} criado em uma thread!");
//            thread::sleep(Duration::from_millis(1));
//        }
//    });
//    for i in 1..5{
//        println!("oi numero {i} criado na funcao main!");
//        thread::sleep(Duration::from_millis(1));
//        
//    }
//}

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("oi numero {i} criado na thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("oi numero {i} criado na main");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}