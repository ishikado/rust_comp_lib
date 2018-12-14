// list up primes less than or equal to upper_bound
// return a prime list and a list which element has true if its index is prime 
pub fn sieve(upper_bound : usize) -> (Vec<i32>, Vec<bool>) {
    let mut prime_list : Vec<i32>  = Vec::new();
    let mut is_prime               = vec![true ; upper_bound + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 0..upper_bound + 1 {
        if is_prime[i] {
            prime_list.push(i as i32);
            let mut mul = i + i;
            while mul <= upper_bound {
                is_prime[mul] = false;
                mul += i;
            }
        }
    }
    return (prime_list, is_prime);
}

#[test]
fn sieve_test(){
    let (prime_list, is_prime) = sieve(30);
    assert_eq!(prime_list, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    assert_eq!(is_prime,   vec![false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false]);
}
