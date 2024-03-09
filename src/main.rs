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

fn main() {
    for a in 0..20 { print!("{:<3} ", a); }
    println!();
    for b in 1..20 {
        print!("{:<3} ", b);
        for a in 1..=b {
            print!("{:<3} ", rectangle_rotation::rectangle_rotation(a, b));
        }
        println!()
    }
}
