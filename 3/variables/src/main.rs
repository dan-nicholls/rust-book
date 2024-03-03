fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "  ";
    let spaces = spaces.len();

    // let mut spaces1 = "     ";
    // spaces1 = spaces1.len()

    // Floating Point
    let float1 = 2.0; // f64

    let float2: f32 = 3.0; // f32

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Char
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //
    // Compound Types
    //

    // Tople
    let tup: (i32, f64, u8) = (500, 7.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let seven_point_four = tup.1;
    let one = tup.2;

    // Array
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // [data_type, num_of_elements]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];
}
