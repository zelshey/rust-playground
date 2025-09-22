fn main() {
    let ret = fibonacci_recursive(10);
    println!("Fibonacci of 10 is {}", ret);
}

fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_loop(n: u32) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut i = 1;
    let mut prev = 0;
    let mut curr = 1;

    while i < n {
        curr += prev;
        prev = curr - prev;
        i += 1;
    }
    curr
}

#[cfg(test)]
mod test {
    use super::fibonacci_loop;
    use super::fibonacci_recursive;

    #[test]
    fn basics_recursive() {
        assert_eq!(
            fibonacci_recursive(0),
            0,
            "fibonacci_recursive(0) should be 0"
        );
        assert_eq!(
            fibonacci_recursive(5),
            5,
            "fibonacci_recursive(5) should be 5"
        );
        assert_eq!(
            fibonacci_recursive(6),
            8,
            "fibonacci_recursive(6) should be 8"
        );
    }

    #[test]
    fn basics_loop() {
        assert_eq!(fibonacci_loop(0), 0, "fibonacci_loop(0) should be 0");
        assert_eq!(fibonacci_loop(1), 1, "fibonacci_loop(1) should be 1");
        assert_eq!(fibonacci_loop(2), 1, "fibonacci_loop(2) should be 1");
        assert_eq!(fibonacci_loop(3), 2, "fibonacci_loop(3) should be 2");
        assert_eq!(fibonacci_loop(4), 3, "fibonacci_loop(4) should be 3");
        assert_eq!(fibonacci_loop(5), 5, "fibonacci_loop(5) should be 5");
        assert_eq!(fibonacci_loop(6), 8, "fibonacci_loop(6) should be 8");
    }

    #[test]
    fn larger_loop() {
        assert_eq!(
            fibonacci_loop(25),
            75025,
            "fibonacci_loop(25) should be 75025"
        );
        assert_eq!(
            fibonacci_loop(40),
            102334155,
            "fibonacci_loop(40) should be 102334155"
        );
        assert_eq!(
            fibonacci_loop(50),
            12586269025,
            "fibonacci_loop(50) should be 12586269025"
        );
        assert_eq!(
            fibonacci_loop(65),
            17167680177565,
            "fibonacci_loop(65) should be 17167680177565"
        );
        assert_eq!(
            fibonacci_loop(100),
            354224848179261915075,
            "fibonacci_loop(100) should be 354224848179261915075"
        );
    }

    #[test]
    fn larger_recursive() {
        assert_eq!(
            fibonacci_recursive(25),
            75025,
            "fibonacci_recursive(25) should be 75025"
        );
        assert_eq!(
            fibonacci_recursive(40),
            102334155,
            "fibonacci_recursive(40) should be 102334155"
        );
        assert_eq!(
            fibonacci_recursive(45),
            1134903170,
            "fibonacci_recursive(45) should be 1134903170"
        );
        // Taking too long
        // assert_eq!(
        //     fibonacci_recursive(50),
        //     12586269025,
        //     "fibonacci_recursive(50) should be 12586269025"
        // );
    }
}
