use std::io;

fn main() {
    loop {
        println!("Which mode?\n1: Fibonacci\n2: F to C\n3: C to F");
        let m = read_num();
        if m == 1 {
            println!("Which Fibonacci number?");
            let n = read_num();
            println!("The {}th Fibonacci number is {}", n, fib(n));
        }
        else if m == 2 {
            println!("Fahrenheit?");
            let n = read_num() as f32;
            println!("{} Celsius", f_to_c(n));
        }
        else if m == 3 {
            println!("Celsius?");
            let n = read_num() as f32;
            println!("{} Fahrenheit", c_to_f(n));
        }
    }
}

fn read_num() -> usize {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    n.trim().parse().expect("Not a number.")
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}


fn fib(n: usize) -> usize {
    let mut mem = Vec::new();
    for i in 0..n as usize {
        if [0, 1].contains(&i) {
            mem.push(1)
        } else {
            mem.push(mem[i-1] + mem[i-2])
        }
    }
    return *mem.last().unwrap_or(&0)
}