// calc rolling hash

pub struct RollingHash {
    primes  : Vec<u64>, // use for rolling hash
    hash    : Vec<u64>,
}

impl RollingHash {
    pub fn new(str : &Vec<u8>, prime : u64) -> RollingHash {
        let mut primes : Vec<u64> = vec![1];
        let str_len = str.len() as usize;
        for i in 1..(str_len + 5) {
            let prv_primes = primes[i-1];
            primes.push(prv_primes.wrapping_mul(prime))
        }
        let mut hash : Vec<u64> = vec![0];
        for i in 1..(str_len + 1) {
            let prv_hash = hash[i - 1];
            hash.push(prv_hash.wrapping_mul(prime).wrapping_add(str[i-1] as u64));
        }
        return RollingHash{primes : primes, hash : hash}
    }

    // get hash value in [x, y]
    pub fn get_hash(&self, x : usize, y : usize) -> u64 {
        return self.hash[y + 1].wrapping_sub(self.hash[x].wrapping_mul(self.primes[y - x + 1]));
    }
    // is self hash value in [x1, y1] and r hash value in [x2, y2] same?
    pub fn is_match(&self, x1 : usize, y1 : usize, x2 : usize, y2 : usize, r : &RollingHash) -> bool {
        return self.get_hash(x1, y1) == r.get_hash(x2, y2);
    }
}


#[test]
fn rolling_hash_test(){

    let md = 1000000000 + 7;
    let r1 = RollingHash::new(&vec![0, 1, 2, 3], md);
    let r2 = RollingHash::new(&vec![0, 1, 2, 3, 2, 0, 1, 2], md);

    assert_eq!(r1.get_hash(0, 2), r2.get_hash(0, 2));
    assert_eq!(r1.get_hash(1, 2), r2.get_hash(6, 7));
    assert_ne!(r1.get_hash(0, 3), r2.get_hash(0, 2));

    assert!(r1.is_match(0, 2, 0, 2, &r2));
    assert!(!r1.is_match(0, 3, 0, 2, &r2));

}
