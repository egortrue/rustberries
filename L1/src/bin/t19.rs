// L1.19

fn simple_reverse(string: &str) -> String {
    string.split(' ').rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    let content = "snow dog sun";
    println!("{}", simple_reverse(content));
    assert_eq!(simple_reverse(content), "sun dog snow");
}
