use romaniam_number_converter::{ArabNum, convert_to_romaniam_num};
fn main() {
    let arab_num = ArabNum::build().unwrap_or_else(|err| {
        panic!("Problem parsing arguments: {err}");
    });
    let romanian_number = convert_to_romaniam_num(arab_num.num);
    println!("arab_nubmer: {}, romanian_number: {romanian_number}", arab_num.num);
}