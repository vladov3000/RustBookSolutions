
fn f_to_c_temp(f_temp: i32) -> i32 {
    (f_temp - 32) * 5 / 9
}

fn fibonacci(n: i32) -> i32 {
    if n < 3 { return 1; }

    let mut fib = (1, 1);
    for _ in 3..n {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.0 + fib.1
}

fn christmas() {
    println!("Christmas is too much writing")
}

fn main() {
    println!("32F is {}C", f_to_c_temp(32));
    println!("0F is {}C", f_to_c_temp(0));

    println!("{}", "=".repeat(20));

    for i in 1..8 {
        println!("fibonacci for n={} is {}", i, fibonacci(i));
    }

    println!("{}", "=".repeat(20));

    christmas();
}
