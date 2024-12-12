fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe and idiomatic way to modify the first element
    println!("v: {:?}", v); // v will be [4, 2, 3]

    v.push(5); // This is now safe because we're using standard vector methods
    println!("v: {:?}", v); // v will be [4, 2, 3, 5]
} 