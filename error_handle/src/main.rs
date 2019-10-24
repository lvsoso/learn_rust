//https://learnku.com/articles/31161

// mod handler {
//     pub fn give_princess(gift: &str) {
//         if gift == "snake" {
//             panic!("AAAaaaaa!!!!");
//         }

//         println!("I love {}s!!!!!", gift);
//     }
// }

// mod handler {
//     include!("handler.rs");
// }

mod handler;
use handler::give_princess;

mod other_handler;
use other_handler::{opt, give_commoner};

fn main() {
    handler::give_princess("teddy bear");
    give_princess("snake");

    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    opt::give_princess(bird);
    opt::give_princess(nothing);
}
