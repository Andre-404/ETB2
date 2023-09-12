use bear_lib_terminal::terminal;

fn main() {
    terminal::open("Test", 80, 30);
    terminal::print_xy(0, 0, "[color=red]asdf[bkcolor=blue]asdf");
    terminal::refresh();
    let _ = terminal::wait_event();
    terminal::close();
    println!("Hello, world!");
}
