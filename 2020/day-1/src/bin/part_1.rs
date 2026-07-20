const TARGET_SUM: i32 = 2020;

// 1721
// 979
// 366
// 299
// 675
// 1456
fn main() {
    let input: Vec<i32> = {
        let mut data: Vec<i32> = include_str!("../input.txt")
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        data.sort_by(|a, b| b.cmp(a));
        data
    };

    let mut i = 0;
    loop {
        let elem = input[i];
        for n in input.iter().skip(i).rev() {
            if elem + n > TARGET_SUM {
                break;
            } else if elem + n == TARGET_SUM {
                let sol = elem * n;
                println!("solution: {sol}");
                return;
            }
        }
        i += 1;
    }
}
