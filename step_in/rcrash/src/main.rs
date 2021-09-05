pub mod array;
pub mod avg;
pub mod closure;
pub mod enums;
pub mod expression;
pub mod function;
pub mod generic;
pub mod let_suggar;
pub mod mod2;
pub mod module;
pub mod pub_control;
pub mod pub_struct;
pub mod random_num;
pub mod slice;
pub mod structs;
pub mod tuple;
pub mod use_bind_super_self;

const A_CONST: i32 = 42;

fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    println!("The value of A_CONST is {}", A_CONST);

    println!("mod2::MESSAGE is {}", mod2::MESSAGE);
    println!("mod2::mod2_a::MESSAGE is {}", mod2::mod2_a::MESSAGE);
}
