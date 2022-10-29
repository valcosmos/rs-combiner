fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

// struct String {
//   vec: Vec<u8>

// }

#[derive(Debug)]
pub struct Args {
    pub image_1: get_nth_arg(1),
    pub image_2: get_nth_arg(2),
    pub output: get_nth_arg(3),
}

impl Args {
    pub fn new() -> Self {
        Args {
            image_1: String::new(),
            image_2: String::new(),
            output: String::new(),
        }
    }
}
