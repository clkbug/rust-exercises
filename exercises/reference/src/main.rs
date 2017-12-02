/*
ゴール：
1. データをコピーしなくても、greetを呼び出せるようにしてください
2. "Hello dear rustaceans"ではなく、"Hello rustaceans"となるように、
   2回目のgreetの呼び出しを行なってください
3. 2をスライスを使って実現してください
*/

fn main() {
    let mut name = format!("dear rustaceans");
    greet(&mut name);
    greet(&mut name);
}

fn greet(name: &mut String) {
    println!("Hello {}", name);
    *name = name[5..].to_string();
}
