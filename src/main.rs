mod attacks;
mod bitboard;
mod board;
mod codegen;
mod rand;
mod squares;

fn main() {
    //find_magic_number(e4, relevant_bits, is_bishop)
    let attacks = crate::attacks::Attacks::new();

    println!("{:?}", attacks);
}
