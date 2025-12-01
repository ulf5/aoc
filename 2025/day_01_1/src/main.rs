fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut dial = 50;
    let mut pw = 0;
    for l in lines {
        match l.as_bytes() {
            [b'R', rest @ ..] => {
                let a: i32 = str::from_utf8(rest).unwrap().parse().unwrap();
                dial = (dial + a).rem_euclid(100);
            }
            [b'L', rest @ ..] => {
                let a: i32 = str::from_utf8(rest).unwrap().parse().unwrap();
                dial = (dial - a).rem_euclid(100);
            }
            _ => continue
        }
        if dial == 0 {
            pw += 1;
        }
    }
    dbg!(pw);
}
