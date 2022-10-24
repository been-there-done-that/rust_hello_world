fn main() {}

/* Slice starts here
fn main() {
    let s = String::from(", Hello");
    let hello = &s[0..6];
    let rest = &s[6..];
    println!("Starting index from 0 - 6 is :: {hello} ::");
    println!("The rest of the slice is :: {rest} ::");
    let a = [1, 3, 21, 12];
    let slice = &a[..2];
    println!("The slice value is :: {:?} ::", slice)
}
slice end here
*/
/* mutable references start
fn main() {
    let mut s1 = String::from("Another String");
    change(&mut s1);
    println!("The value post change is :: {s1} ::");
    change(&mut s1);
    println!("The value post change is :: {s1} ::");
    reference_check();
    mutale_immutable_refence();
}

fn change(s: &mut String) {
    s.push_str(", Appended");
}

fn reference_check() {
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
        println!("r1 = :: {r1} ::");
    }
    {
        let r2 = &mut s;
        println!("r2 = :: {r2} ::");
    }
}

fn mutale_immutable_refence() {
    let mut s = String::from("yo man");
    let s3 = &mut s;
    let s2 = &s;

    println!("The mutated s value is :: {s} ::");
    println!("The value of s2 is :: {s2} ::");
}
mutable references end
*/
/* immnutable references start
fn main() {
    let s1 = String::from("New string");
    let length = new_borrow(&s1);
    println!("The value after borrowing is :: {s1} :: and the length is :: {length} ::")
}

fn new_borrow(s: &String) -> usize {
    s.len()
}
immutable references
*/
/* borrowing expalnation starts --------------------
fn main() {
    let x = 10;
    let y = x;
    println!("The value of x :: {x} :: and y :: {y} ::");

    // move
    {
        let mut a = String::from("hello");
        a.push_str(", World");
        println!("The value of a :: {a} ::");

        let b = a;
        // println!("The value of a :: {a} :: and b :: {b} ::");
        println!("The value of b :: {b} ::");
    }

    // clone

    {
        let str = String::from("Hello");
        println!("str = {str}");

        let str1 = str.clone();
        println!("str1 = {str1}");

        println!("The value of str :: {str} :: and str1 :: {str1} ::");
    }
    let s = String::from("Random String");
    take_ownership(s);
    let s1 = gives_ownership();
    println!("The s value :: {s1} ::");

    let (s2, l) = get_length(s1);
    println!("The s1 length of s1 is :: {l} ::");
    println!("The s value :: {s2} ::");
}

fn get_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn take_ownership(s: String) -> String {
    println!("s = :: {s} ::");
    s
}

fn gives_ownership() -> String {
    let s = String::from("Random String");
    s
}
borrowing expalnation ends -------------------- */

/*

fn main() {
    func_a();
    func_b(12, 1.2);

    // let;s use if else statements to assign a value
    let condition = true;
    let number = if !condition {
        5
    } else {
        10
    };
    println!("The value evaluated from the if else if :: {number} ::");
    let mut count = 0;
    loop {
        println!("Printing inside the Loop:: {count} ::");
        count += 1;
        if count >= 10 {
            break;
        }
    }
    println!("Printing inside the Loop:: {count} ::");

    let mut count = 0;
    while count <= 10 {
        println!("The loop counter is :: {count} ::");
        count += 1;
    }

    // for loops
    let collection: [i32; 4] = [1, 4, 3, 5];

    for mut ele in collection {
        ele = ele + ele;
        println!("The element un the collection is :: {ele} :: {:?} ::",collection);
    }

    // scope fo a variable
    {
        let x = 5;
        println!("x = {x}");
    }
    // println!("x = {x}");
}

fn func_a() {
    println!("func_a");
}

fn func_b(x: i32, y: f64) {
    println!("Value of x :: {x} :: and y :: {y} :: ");
    let z: i32 = {
        x + 1
    };

    println!("The z value in the expression :: {z} ::");
    let sum = sum(1, 2);
    println!("The sum value that is returned is :: {sum} ::");
}

fn sum(x: i64, y: i64) -> i64 {
    x + y
}

fn main() {
    let mut x: i64 = 10;
    println!("Hello, world! {}", x);
    x = 20;
    print!("Value of x is :: {}", x);
    let c2 = '\u{1F603}';
    println!("The unicode value is :: {c2}");
    let a1 = i32::MAX;
    println!("The max value in i32 is {a1}");
    let b1: bool = 1 < 4;
    println!("The boolean value is :: {b1} ::");

    // tuple
    let tup: (i32, u8, f32) = (10, 20, 3.2);
    let (x, y, z) = tup;
    println!("Value of x = {x}, y = {y}, z = {z}");
    let tup1 = tup.0;
    println!("The first value of the tuple is :: {tup1} ::");

    // array is of same data type
    // fixed length(can't be changed)
    let arr: [i8; 5] = [1,2, 3, 4,3];

    println!("Array elements are :: {:#?} ::",arr);
    let arr1: &i8 = arr.last().unwrap();
    println!("The last element from the array is :: {arr1}");
    let arr1: &i8 = arr.last().unwrap();
    println!("The last element from the array is :: {arr1}");
    let arr1: i8 = arr[arr.len() -1];
    println!("The last element from the array is :: {arr1}");
    let arr1 = arr[100];
    println!("The last element from the array is :: {arr1}");

}
*/
