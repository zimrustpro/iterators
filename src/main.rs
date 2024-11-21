struct GivesOne;

impl Iterator for GivesOne {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}

fn main() {
    let five_ones = GivesOne.into_iter().take(5).collect::<Vec<i32>>();
    println!("{:?}", five_ones);
}
