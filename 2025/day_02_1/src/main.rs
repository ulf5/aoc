fn main() {
    let ranges = include_str!("../input.txt").trim().split(',');
    let mut tot = 0u64;
    for r in ranges {
        if let Some((start, end)) = r.split_once('-') {
            if start.len() % 2 == 1 && start.len() == end.len() {
                continue
            }
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            for i in start..=end {
                let istr = i.to_string();
                if istr.len() % 2 == 1 {
                    continue
                }
                let (head, tail) = istr.split_at(istr.len() / 2);
                if head == tail {
                    tot += i;
                }
            }
        }
    }
    dbg!(tot);
}
