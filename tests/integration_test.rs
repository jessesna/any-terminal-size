#[cfg(not(windows))]
fn test_unix() {
    let s = any_terminal_size::any_terminal_size_of_process(std::process::id());
    println!("Size from any_terminal_size_of_process(): {:?}", s);
}

fn test_shared() {
    let s = any_terminal_size::any_terminal_size();
    println!("Size from any_terminal_size(): {:?}", s);
}

#[test]
fn shared() {
    test_shared();
}

#[test]
#[cfg(not(windows))]
fn unix() {
    test_unix();
}
