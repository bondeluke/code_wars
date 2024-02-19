use num_bigint::BigInt;

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
mod last_digit_huge;

fn main() {
    println!("{}", BigInt::from(2).pow(41));
    println!("{}", last_digit::last_digit("12", "20"));
    println!("{}", last_digit_huge::last_digit(&vec![2, 2, 101, 2]));
    for n in 2..89 {
        // println!("==={n}===");
        for p in 11..12 {
            let bi = BigInt::from(n).pow(p).to_string();
            let last2 = &bi[bi.len() - 2..];
            println!("{}: {}", n, last2)
        }
    }
}
