mod args;
use args::Args;

fn main() {
    let args = Args::new();
    println!("{:?}", args);
}

// impl String {
//     fn new()->Self{
//         String{
//             vec:Vec::new(),
//         }
//     }
// }
