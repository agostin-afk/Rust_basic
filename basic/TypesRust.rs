fn main(){
    let x:i32 = -45;
    let y:u32 = 45;
    let z:f32 = -4.5;
    let letra:char = 'A';
    let nome:String = String::from("Agosto");
    let tup:(u32, i32,char) = (45, -45, 'A');
    let tup_simple = (999, 666, 333);
    let variable_tupl = (x, y, z);
    let arr:[i32; 5] =[1,2,3,4,5];
    let arr_simple = [5,6,7,8,9];
    let single_value_array = [45;5]; 
    // para get em uma tupla = tup.index ex: tup.1
    // para get em um array = array[index] ex: arr[3]
    println!("\n{x}\n{y}\n{z}\n{letra}\n{nome}\n{tup:?}\n{tup_simple:?}\n{variable_tupl:?}\n{arr:?}\n{arr_simple:?}\n{single_value_array:?}");
}