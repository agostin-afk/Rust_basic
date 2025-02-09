fn main(){
    let mut count:i32 = 0;
    'counting: loop{
        println!("count = {count}\n");
        let mut aux = 10;
        loop{
            println!("aux: {aux}");
            if aux == 9{
                break;
            }
            if count == 2{
                break 'counting;
            }
            aux -=1; 
        }
        count +=1;
    }
}