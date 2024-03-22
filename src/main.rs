use std::time::Instant;
use crate::prime_streaming_nc17::{stream17};

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
mod product_fib;
mod sum_by_factors;
mod digital_root;
mod format_duration;
mod next_bigger_number;
mod screen_locking_patterns;
mod make_a_spiral;
mod rectangle_rotation;
mod rail_fence_cypher;
mod large_factorials;
mod sudoku_solver;
mod tree_by_levels;
mod prime_streaming;
mod order_weight;
mod prime_streaming_nc17;

fn main() {
    let start_time = Instant::now();
    let mut prime_iterator = stream17();
    for _ in 0..42331 - 10 {
        prime_iterator.next();
    }
    for _ in 0..10 {
        println!("{}", prime_iterator.next().unwrap());
    }
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Duration: {:01}.{:03}s", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}


