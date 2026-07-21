pub fn main() {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let sol = include_str!("../input.txt")
        .split("\n\n")
        .map(|passport| {
            let parts: Vec<&str> = passport
                .split_whitespace()
                .map(|part| part.split(':').next().unwrap())
                .collect();
            fields.iter().all(|field| parts.contains(field))
        })
        .filter(|&b| b)
        .count();

    println!("Solution: {sol}");
}
