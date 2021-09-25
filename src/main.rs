use input_i_scanner::{InputIScanner, scan_with};

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let n = scan_with!(_i_i, usize);
    todo!();
}
