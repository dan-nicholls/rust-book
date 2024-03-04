fn read(y: bool) {
    if y {
        println!("y is true");
    }
}
fn main() {
    let x = true; // Valid
    read(x);
    // let x = true; // Invalid

    // Box allocated memory to the heap. A & B are pointers.
    let a = Box::new([0; 1_000_000]);
    let b = a;

    // # Ownership

    // ## Eg 1
    let b = Box::new(0); // `b` takes ownership of the heap-allocated integer.
    let b2 = b; // Ownership of the heap-allocated integer is moved to `b2`.
    println!("{}", b); // ❌ This line tries to use `b` after its ownership has been moved.
    move_a_box(b2); // Assuming `move_a_box` takes ownership of its argument, `b2` is moved here.

    // ## Eg 2
    let b = Box::new(0); // `b` takes ownership of the heap-allocated integer.
    let b2 = b; // Ownership of the heap-allocated integer is moved to `b2`.
    move_a_box(b); // ❌ This line tries to use `b` after its ownership has been moved, which is not allowed.

    // ## Eg 3
    let b = Box::new(0); // `b` takes ownership of the heap-allocated integer.
    move_a_box(b); // Ownership of `b` is moved to the `move_a_box` function.
    let b2 = b; // ❌ Error: attempt to use `b` after it has been moved.
}
