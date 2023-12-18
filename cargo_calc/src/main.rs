// Add to Cargo.toml
// rprompt = "1.0.5"

fn main() {
    //Taking user input
    let a = rprompt::prompt_reply_stdout("Enter first number: ").unwrap();
    let b = rprompt::prompt_reply_stdout("Enter second number: ").unwrap();
    let cal =
        rprompt::prompt_reply_stdout("[1] Add [2] Subtract [3] Multiply [4] Divide: ").unwrap();

    //Turning input from string to int
    let a: f64 = a.parse().unwrap();
    let b: f64 = b.parse().unwrap();
    let cal: f64 = cal.parse().unwrap();

    let mut ans = 0.0;

    if cal == 1. {
        ans += a + b;
    } else if cal == 2. {
        ans = a - b;
    } else if cal == 3. {
        ans = a * b;
    } else if cal == 4. {
        ans = a / b;
    }

    println!("The answer is {}", ans)
}
