use std::io;

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Sequential Circuit Divider");
        println!("Enter first number (dividend) or 'q' to quit:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().eq_ignore_ascii_case("q") {
            println!("Exiting...");
            break;
        }

        let dividend: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        };

        println!("Enter second number (divisor): ");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let divisor: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        };

        println!("Enter number of bits:");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let n_bits: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        };

        if dividend > 0 && divisor > 0 {
            restoring_div(dividend, divisor, n_bits);
        } else {
            println!("Division contains negative, cannot use restoring division.");
        }

        non_restoring_div(dividend, divisor, n_bits);

        println!("\nPress Enter to continue...");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
    }
}

fn mask(n: i64, n_bits: usize) -> i64 {
    n & ((1 << n_bits) - 1)
}

fn msb(n: i64, n_bits: usize) -> i64 {
    (n >> (n_bits - 1)) & 1
}

fn sign_extend(n: i64, orig_len: u8) -> i64 {
    let sign_bit = 1 << (orig_len - 1);

    if n & sign_bit != 0 {
        n | (!0 << orig_len)
    } else {
        n
    }
}

fn print_hline(n_bits: usize) {
    for _ in 0..n_bits + 1 {
        print!("-");
    }

    println!("-");
}

fn print_add(mut x: i64, mut y: i64, n_bits: usize) {
    x = mask(x, n_bits);
    y = mask(y, n_bits);
    println!("A <- A + M:");
    println!("  {x:0n$b}", n = n_bits);
    println!("+ {y:0n$b}", n = n_bits);
    print_hline(n_bits);
    x += y;
    x = mask(x, n_bits);
    println!("  {x:0n$b}\n", n = n_bits);
}

fn print_sub(mut x: i64, mut y: i64, n_bits: usize) {
    x = mask(x, n_bits);
    y = !y + 1;
    y = mask(y, n_bits);
    println!("A <- A - M:");
    println!("  {x:0n$b}", n = n_bits);
    println!("+ {y:0n$b}", n = n_bits);
    print_hline(n_bits);
    x += y;
    x = mask(x, n_bits);
    println!("  {x:0n$b}\n", n = n_bits);
}

fn restoring_div(dividend: i64, divisor: i64, n_bits: usize) {
    println!("\nRestoring Division");
    let mut a = 0;
    let mut q = dividend;
    let mut m = divisor;

    m = mask(m, n_bits);
    q = mask(q, n_bits);

    for i in 0..n_bits {
        println!("Iteration {itr}:\n", itr = i + 1);
        a = msb(q, n_bits) | a << 1;
        q <<= 1;

        a = mask(a, n_bits + 1);
        q = mask(q, n_bits);

        println!("Shift left AQ:");
        println!("A: {:0n$b} Q: {:0m$b}\n", a, q, n = n_bits + 1, m = n_bits);
        print_sub(a, m, n_bits + 1);
        a -= m;

        if a < 0 {
            println!("A < 0, hence");
            print_add(a, m, n_bits + 1);
            a += m;
            q &= !1;
        } else {
            q |= 1;
        }
    }

    let quotient = sign_extend(q, n_bits as u8);
    let remainder = sign_extend(a, n_bits as u8);

    println!("Q: {quotient} A: {remainder}");
    println!("======================================================")
}

fn non_restoring_div(dividend: i64, divisor: i64, n_bits: usize) {
    println!("\nNon-restoring Division");
    let mut a = 0;
    let mut q = dividend;
    let mut m = divisor;

    m = mask(m, n_bits);
    q = mask(q, n_bits);

    for i in 0..n_bits {
        println!("Iteration {itr}:\n", itr = i + 1);
        a = msb(q, n_bits) | a << 1;
        q <<= 1;

        a = mask(a, n_bits + 1);
        q = mask(q, n_bits);

        println!("Shift left AQ:");
        println!("A: {:0n$b} Q: {:0m$b}\n", a, q, n = n_bits + 1, m = n_bits);

        if msb(a, n_bits + 1) == 1 {
            println!("A < 0, hence");
            print_add(a, m, n_bits + 1);
            a += m;
            if msb(a, n_bits + 1) == 1 {
                q &= !1
            } else {
                q |= 1
            };
        } else {
            println!("A > 0, hence");
            print_sub(a, m, n_bits + 1);
            a -= m;
            if msb(a, n_bits + 1) == 1 {
                q &= !1
            } else {
                q |= 1
            };
        }

        a = mask(a, n_bits + 1);
        println!("A: {:0n$b} Q: {:0m$b}\n", a, q, n = n_bits + 1, m = n_bits);
    }

    if msb(a, n_bits + 1) == 1 {
        println!("Final result of A < 0, restoring...");
        print_add(a, m, n_bits + 1);
        a += m
    }

    a = mask(a, n_bits + 1);

    let quotient = sign_extend(q, n_bits as u8);
    let remainder = sign_extend(a, n_bits as u8);

    println!("Q: {quotient} A: {remainder}");
    println!("======================================================")
}
