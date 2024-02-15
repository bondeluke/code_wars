mod to_camel_case;
mod find_short;
mod is_pangram;
mod multiples_of_3_or_5;
mod alphabet_position;
mod spin_words;
mod delete_nth;
mod find_odd;
mod likes;
mod move_zeros;
mod tribonacci;

fn main() {
    let word = "Hello world!";

    let x = (&word[..1]).to_uppercase();
    let y = &word[1..];

    println!("x={x} y={y}");
}
