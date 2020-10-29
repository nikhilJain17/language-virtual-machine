use std::mem;

fn pain() {
    println!("Hello, world!");
    structs();
}   

fn tuples() {
    let person : (&str, &str, i8) = ("Nikhil", "NJ", 21);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}

fn arrays() {
    // arrays are fixed length -- use vectors otherwise 
    let mut numbers : [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    numbers[1] *= -1;
    println!("{:?}", numbers);

    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}

fn vectors() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("Vector: {:?}", numbers);

    for x in numbers.iter_mut() {
        (*x) *= 2;
    }

    println!("Vector: {:?}", numbers);

}

// funcs
fn functions() {
    fnhelper("Hello", "Nikhil");
    let sum:i32 = add(2, 3);
    println!("Sum: {}", sum);

    // Closure
    let n3:i32 = 10;
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(2, 3));

}

fn fnhelper(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1:i32, n2:i32) -> i32 {
    return n1 + n2;
}

// ptrs
fn pointers() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable iwll no longer hold that value.
    // Use pointers!

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

}

// structs
struct Color { // traditional struct
    red: u8,
    green: u8,
    blue: u8
}

struct ColorTuple(u8, u8, u8); // tuple struct --> like namedTuple

struct Person {
    first_name : String,
    last_name : String
}

impl Person {
    // Constructor
    fn new(first:&str, last: &str) -> Person {
        Person {
            first_name : first.to_string(),
            last_name : last.to_string()
        }
    }
    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }
}

fn structs() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    c.red = 200;

    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    // --
    let mut c2 = ColorTuple(255, 0, 0);
    println!("ColorTuple: {} {} {}", c2.0, c2.1, c2.2);

    // --
    let mut p = Person::new("Nikhil", "Jain");
    println!("Person: {}, {}", p.first_name, p.last_name);
    println!("Person: {}", p.full_name());
}