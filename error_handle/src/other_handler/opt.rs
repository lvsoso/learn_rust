pub fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

pub fn give_princess(gift: Option<&str>) {
    // 使用 `unwrap`，当接收到 `None` 时返回一个 `panic`。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
    super::opt::pp(inside);
}

pub fn pp(msg : &str){
    println!("{}", msg);
}