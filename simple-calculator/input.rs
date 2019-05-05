use std::io;

pub fn get_int() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

   let n: i32 = input.trim().parse().unwrap();
   return n;
}