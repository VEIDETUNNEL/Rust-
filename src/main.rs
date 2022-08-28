// #[allow(dead_code)]
#[allow(dead_code)]
#[allow(unused_mut)]

pub fn main() {
    /*
    let mut string = String::from("gunnal was here");

    string.push_str("ehllo");

    let mut x: bool = true;
    let c: char = '\u{1F3B8}';

    let max = std::i128::MAX;

    println!("{:?}", (max, x, c, string));
    {
        let num: i32 = 12;

        println!("{}", num);
    }
    let spaces: &str = "    ";
    println!("{}", spaces.len())
        */

    let mut arr: [i32; 6] = [1, 65, -12, 56, 234, 1];
    let mut i = 0;
    while i < arr.len() {
        println!("{}", arr[i]);
        if arr[i] < 0 {
            continue;
        }
        i = i + 1;
    }
}
