use fancy_regex::Regex;

fn main() {
    let re = Regex::new(r"^([0-9]+)\1+$").unwrap();
    let ranges = include_str!("../input.txt").trim().split(',');
    let mut tot = 0u64;
    for r in ranges {
        if let Some((start, end)) = r.split_once('-') {
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            for i in start..=end {
                let istr = i.to_string();
                if re.is_match(&istr).unwrap() {
                    tot += i;
                }
            }
        }
    }
    dbg!(tot);
}
