use std::env;

pub fn print_args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // for i in env::args() {
    //     println!("{}", i);
    // }
}

// main.rs

// mod test_args;
// fn main() {
//     test_args::main();
// }

// include!("../src/test_args.rs");
// fn main() {
//     print_args();
// }