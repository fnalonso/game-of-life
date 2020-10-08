pub enum Color {
    Black = 0,
    Grey = 7
}

pub fn print_cell(x: i32, y: i32, color:Color) {
    print!("\x1b[{};{}H\x1b[48;5;{}m X ", x, y, color as u32);
}

pub fn clear() {
    print!("\x1b[2J");
}

pub fn reset() {
    print!("\x1b[m");
}