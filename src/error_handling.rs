/// 3 equivalent error handling examples which panics (w/o propagating to caller)

// Using expect
// #[test] // Uncomment to see the panic
fn ex_1() {
    let xx: Result<u32, &str> = Err("emergency failure");
    let _yy = xx.expect("Testing expect"); 
        // panics with `Testing expect: emergency failure`
}

// Using match - same as `expect("...")` in ex_1:
// #[test] // Uncomment to see the panic
fn ex_2() {
    let xx: Result<u32, &str> = Err("emergency failure");

    // panics with `Testing expect: emergency failure`
    let _yy = match xx {
        Ok(m) => m,
        Err(error) => panic!("Testing expect: {:?}", error),
    };
}

// Using unwrap - same as using an empty expect `expect("")` in ex_1:
// #[test] // Uncomment to see the panic
fn ex_3() {
    let xx: Result<u32, &str> = Err("emergency failure");
    let _yy = xx.unwrap(); 
        // panics with `: emergency failure`
}

//// ---------