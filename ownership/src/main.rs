fn main() {
    // move_example();
    // take_vs_make();
    references();
}

fn move_example() {
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }
}

fn take_vs_make() {
    let mut s = String::from("hello");
    s = takes_ownership(s);

    let x = 5;
    makes_copy(x);
    // x is still usable
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int)
}

fn references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}