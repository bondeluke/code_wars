use crate::millionth_fibonacci::fib;

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
mod sum_intervals;
mod prime_factors;
mod even_or_odd;
mod positive_sum;
mod remove_smallest;
mod human_readable_time;
mod last_digit;
mod odd_bit;
mod rgb_to_hex;
mod directions_reduction;
mod rot13;
mod snail;
mod alphanumeric;
mod millionth_fibonacci;

fn main() {
    let number: u8 = 10; // Example number
    let shifted_left = number << 2; // Left shift by 2 positions
    let shifted_right = number >> 1; // Right shift by 1 position

    println!("Original number: {}", number);
    println!("Left shift by 2: {}", shifted_left);
    println!("Right shift by 1: {}", shifted_right);
    println!("{}", fib(7))
}
