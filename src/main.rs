mod attacks;
mod bitboard;
mod squares;

mod codegen;

pub enum Side {
    White,
    Black,
}

fn main() {
    //find_magic_number(e4, relevant_bits, is_bishop)
    let attacks = crate::attacks::Attacks::new();

    println!("{:?}", attacks);
}
