struct Rectangle{
    wid: u32,
    hei: u32,
}
impl Rectangle{
    fn get_area(&self) -> u32{
        self.wid * self.hei
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.wid > other.wid && self.hei > other.hei
    }
}
fn main(){
    let box1 = Rectangle{
        wid: 45,
        hei: 45,
    };
    let box2 = Rectangle{
        wid: 10,
        ..box1
    };
    let box3 = Rectangle{
        wid: 100,
        hei: 100
    };
    let box4 = Rectangle{
        wid: 10,
        hei: 10
    };
    println!("size box1: {}\n", box1.get_area());
    println!("size box2: {}\n", box2.get_area());
    println!("size box3: {}\n", box3.get_area());
    println!("size box4: {}\n", box4.get_area());
    println!("A box1 cabe a box2?: {}", box1.can_hold(&box2));
    println!("A box1 cabe a box4?: {}", box1.can_hold(&box4));

}