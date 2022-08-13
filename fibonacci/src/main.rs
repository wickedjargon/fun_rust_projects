fn main() {
    const START: u32 = 0;
    const STOP: u32 = 10;

    println!("fib recursive {}..{}:", START, STOP);
    for i in START..STOP {
        println!("{}", fib_recursive(i));
    }
    println!("");
    println!("fib iterative {}..{}:", START, STOP);
    for i in START..STOP {
        println!("{}", fib_iterative(i));
    }
}

fn fib_recursive(x: u32) -> u32 {
    if x == 0 {
        return 1;
    } else if x == 1 {
        return 1;
    } else {
        return fib_recursive(x - 1) + fib_recursive(x - 2);
    }
}

fn fib_iterative(x: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _i in 0..x {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}
