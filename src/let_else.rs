use std::str::FromStr;

#[test] fn ex_1() {
    let res = Some(42);
    // let else - since rust 1.66
    // before:
    let x = match res {
      Some(x) => x,
      None => return,
    };
    // now:
    let Some(x) = res else { return };
    assert_eq!(x, 42);
}

// ex-2
fn process_str(s: &str) -> u64 {
    // Using it like an "early return"
    // u64::from_str(s) -> Result<Self, ParseIntError>
    let Ok(count) = u64::from_str(s) else {
        panic!("Can't parse integer: '{s}'");
    };
    // instead of
    let res = u64::from_str(s);
    if res.is_err() {
        return 42
    }
    let count = res.unwrap();

    // ... do_something_with(count)
    count + 1
}

#[test]
fn ex2_simple_works() {
    assert_eq!(process_str("3"), 4);
}

#[test]
#[should_panic]
fn ex3_panics() {
    assert_eq!(process_str("aa"), 3);
}