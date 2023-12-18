use std::io;

fn main() {
    fn sum(n: i32) -> i32 {
        if n == 1 {
            println!("1");
            return n;
        }

        print!("{} + ", n);
        return n + sum(n - 1);
    }
    let total = sum(5);
    println!("total = {} ", total);

    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("number - {}", name);
        }
        Err(e) => {
            println!("error program - {}", e);
        }
    }
}
