fn main() {
    use any_terminal_size::{any_terminal_size, Height, Width};
    let size = any_terminal_size();
    if let Some((Width(w), Height(h))) = size {
        println!("The terminal size of your process or [transitive] parent process is {} cols wide and {} lines tall.", w, h);
    } else {
        println!("Unable to get terminal the size.");
    }
}
