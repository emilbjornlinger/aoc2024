fn main() {
    let my_vec = vec![1, 2, 3, 4];

    for (idx, val) in my_vec[1..].iter().enumerate() {
        println!("idx: {idx}, val: {val}");
    }
}
