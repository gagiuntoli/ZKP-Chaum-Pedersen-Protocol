use rand::Rng;

pub fn exponentiate(num: u32, exp: u32, p: u32) -> u32 {
    num.pow(exp) % p
}

pub fn solve(x: u32, k: u32, c: u32, q: u32) -> u32 {
    // s = (k - c * x) mod q
    let s = (k as i32 - (c * x) as i32) % q as i32;
    if s >= 0 {
        s as u32
    } else {
        assert!(q as i32 + s >= 0);
        (q as i32 + s) as u32
    }
}

pub fn verify(g: u32, h: u32, p: u32, y1: u32, y2: u32, r1: u32, r2: u32, c: u32, s: u32) -> bool {
    // R1 = g ^ s * Y1 ^ c
    let eq1 = r1 == (exponentiate(g, s, p) * exponentiate(y1, c, p)) % p;
    // R2 = h ^ s * Y2 ^ c
    let eq2 = r2 == (exponentiate(h, s, p) * exponentiate(y2, c, p)) % p;

    eq1 && eq2
}

pub fn random_number() -> u32 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

fn main() {
    println!("{}", random_number());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponentiate() {
        assert_eq!(exponentiate(1, 2, 11), 1);
        assert_eq!(exponentiate(2, 3, 11), 8);
        assert_eq!(exponentiate(2, 4, 11), 5);
    }

    #[test]
    fn test_solve() {
        // (10 - 2 * 1) mod 101 = 8
        assert_eq!(solve(2, 10, 1, 101), 8);
        // (10 - 2 * 6) mod 101 = 99
        assert_eq!(solve(2, 10, 6, 101), 99);
    }

    #[test]
    fn test_verify() {
        //p=23 q=11 g=4 h=9 x=6 k=7 c=4 Y1=2, Y2=3, R1=8, R2=4, s=5
        let g = 4;
        let h = 9;
        let q = 11;
        let p = 23;
        let y1 = 2;
        let y2 = 3;
        let r1 = 8;
        let r2 = 4;
        let c = 4;
        let s = 5;
        assert!(verify(g, h, p, y1, y2, r1, r2, c, s));
        assert!(!verify(g, h, p, y1, y2, r1, r2, c, (s + 1) % q));
    }

    #[test]
    fn test_toy_example() {
        //p=23 q=11 g=4 h=9 x=6 k=7 c=4 Y1=2, Y2=3, R1=8, R2=4, s=5
        let g = 4;
        let h = 9;
        let q = 11;
        let p = 23;

        let x = 6; // secret

        // registration
        let y1 = exponentiate(g, x, p);
        let y2 = exponentiate(h, x, p);

        // verification (a)
        let k = random_number() % 10; // 0 - 9
        let r1 = exponentiate(g, k, p);
        let r2 = exponentiate(h, k, p);

        // verification (b)
        let c = random_number() % 10; // 0 - 9

        // verification (c)
        let s = solve(x, k, c, q);
        assert!(verify(g, h, p, y1, y2, r1, r2, c, s));
        assert!(!verify(g, h, p, y1, y2, r1, r2, c, (s + 1) % q));
    }
}
