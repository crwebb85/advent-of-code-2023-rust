fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}
fn get_calibration_value(line: &str) -> i64 {
    const RADIX: u32 = 10;

    let first = line.find(|c: char| (c <= '9') && (c >= '0'));
    let mut first_num = 0;
    if !first.is_none() {
        first_num = line
            .chars()
            .nth(first.unwrap())
            .unwrap()
            .to_digit(RADIX)
            .unwrap();
    }

    let last = line.rfind(|c: char| (c <= '9') && (c >= '0'));
    let mut last_num = 0;
    if !last.is_none() {
        last_num = line
            .chars()
            .nth(last.unwrap())
            .unwrap()
            .to_digit(RADIX)
            .unwrap();
    }
    println!("num:{}{}", first_num, last_num);

    let total = i64::try_from(first_num).unwrap() * 10 + i64::try_from(last_num).unwrap();
    return total;
}

fn part1(input: &str) -> i64 {
    input.split('\n').map(get_calibration_value).sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        println!("{}", input);

        let result = part1(input);

        assert_eq!(result, 142);
    }
}
