// use std::io;

fn main() {
    let hello: Vec<i32> = (0..10).collect();

    fn do_stuff(val: &Vec<i32>) {
        println!("{}", val.len());
    }

    do_stuff(&hello);
}
