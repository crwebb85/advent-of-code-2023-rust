# advent-of-code-2023-rust

```sh
cargo new day-01
cd day-01
mkdir bin
mv ./main.rs ./bin/part1.rs
#when one file 
cargo run
#when multiple files
cargo run --bin part1
cargo run --bin part2

#to run tests
cargo test
#when multiple files
cargo test --bin part1
# cargo test --bin part2
```
Starter template
```rust
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("");
        assert_eq!(result, "4");
    }
}
```


