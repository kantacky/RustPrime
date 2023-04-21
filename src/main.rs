mod prime;

fn main() {
    let max: usize = usize::max_value();

    for i in 2..max {
        if prime::is_prime(i) {
            println!("{}", i);
        }
    }

    // let is_prime_vec: Vec<bool> = prime::get_is_prime_vec(max);

    // for i in 0..is_prime_vec.len() {
    //     if is_prime_vec[i] {
    //         println!("{}", i);
    //     }
    // }
}
