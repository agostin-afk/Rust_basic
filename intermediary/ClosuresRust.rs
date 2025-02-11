#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor{
    Red,
    Blue
}
struct Inventory{
    shirt: Vec<ShirtColor>,
}
impl Inventory{
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirt{
            match color{
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            };
        }
        if num_blue > num_red{
            ShirtColor::Blue
        }
        else{
            ShirtColor::Red
        }
    }
}
fn main(){
    let store = Inventory {
        shirt: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The User with preference: {:?} \ngets: {:?}",
        user_pref1, giveaway1
    );
}