fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!("v: {:?}", v); // v will be [4, 2, 3]

    // This is where it gets dangerous!
    // The vector's capacity might not be big enough to contain the new element
    v.push(5);

    // The above line might cause undefined behavior, such as memory corruption.
    // Because the pointer might be invalidated when the vector's capacity changes.
    println!("v: {:?}", v); // This might crash or print unexpected results
}