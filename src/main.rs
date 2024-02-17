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

fn main() {
    println!("to_camel_case(\"the_stealth_warrior\") = \"{}\"", to_camel_case::to_camel_case("the_stealth_warrior"));
    println!("prime_factors(720) = {}", prime_factors::prime_factors(720))
}
