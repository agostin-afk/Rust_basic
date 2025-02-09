fn main(){
    enum IpaddrKind{
        V4,
        V6,
    }
    struct IpAddr{
        kind: IpaddrKind,
        address: String,
    }
    let home = IpAddr{
        kind: IpaddrKind::V4,
        address: String::from("000.0.0.0"),
    };

}