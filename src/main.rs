use crate::prime_streaming::stream;

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

fn main() {
    test_segment(1_000_000);
}

fn test_segment(start: u32) {
    let mut prime_iterator = stream();
    for _ in 0..start {
        prime_iterator.next();
    }
    for _ in 0..10 {
        println!("{}", prime_iterator.next().unwrap());
    }
}
