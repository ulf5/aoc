fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut dial = 50;
    let mut pw = 0;
    for l in lines {
        match l.as_bytes() {
            [b'R', rest @ ..] => {
                let a: i32 = str::from_utf8(rest).unwrap().parse().unwrap();
                pw += (dial + a).div_euclid(100).abs();
                dial = (dial + a).rem_euclid(100);
            }
            [b'L', rest @ ..] => {
                let a: i32 = str::from_utf8(rest).unwrap().parse().unwrap();
                pw += (dial - a).div_euclid(100).abs();
                if dial == 0 {
                    pw -= 1;
                }
                dial = (dial - a).rem_euclid(100);
                if dial == 0 {
                    pw += 1;
                }
            }
            _ => continue
        }
    }
    dbg!(pw);
}
