// https://www.codewars.com/kata/5541f58a944b85ce6d00006a/train/rust

#[allow(dead_code)]
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let (mut a, mut b) = (0, 1);

    while a * b < prod {
        (a, b) = (a + b, a);
    }

    (b, a, a * b == prod)
}

#[cfg(test)]
mod tests {
    use crate::product_fib::product_fib;

    fn do_test(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }

    #[test]
    fn basics_product_fib() {
        do_test(4895, (55, 89, true));
        do_test(5895, (89, 144, false));
    }
}
