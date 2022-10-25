fn main() {}

/*
// single or multiple traits implementation start here

fn main() {
    let d = Dog {
        name: "Tommy".to_string(),
        age: 4,
    };
    d.speak();
    d.voice_level();

    let d = Cat {
        name: "Fluffy".to_string(),
        age: 2,
    };
    d.speak();
    d.voice_level();
}

struct Dog {
    name: String,
    age: i32,
}

struct Cat {
    name: String,
    age: i32,
}

trait Voice {
    fn speak(&self);

    fn voice_level(&self) {
        println!("The default voice level is :: -1 ::");
    }
}

impl Voice for Dog {
    fn speak(&self) {
        println!(
            "The dog name is :: {} :: and age is :: {} :: ",
            self.name, self.age
        );
        println!("we added a voice using the voice trait to DOG");
    }
}

impl Voice for Cat {
    fn speak(&self) {
        println!(
            "The cat name is :: {} :: and age is :: {} :: ",
            self.name, self.age
        );
        println!("we added a voice using the voice trait to CAT");
    }
    fn voice_level(&self) {
        println!("The voice level for cat has been changed to :: -10  ::");
    }
}

// single or multiple traits implementation end here

*/

/*
// generic function start here
fn main() {
    println!("The non generic solution is :: {} ::", max_i32(2, 3));
    println!("The non generic solution is :: {} ::", max_f32(12.3, 3.12));

    println!("The generic solution is :: {} ::", max_generic(12, 30));
    println!(
        "The generic solution is :: {} ::",
        max_generic(31.14, 30.12)
    );
}

fn max_i32(a: i32, b: i32) -> i32 {
    if b > a {
        return b;
    } else {
        return a;
    }
}

fn max_f32(a: f32, b: f32) -> f32 {
    if b > a {
        return b;
    } else {
        return a;
    }
}

fn max_generic<T: PartialOrd>(a: T, b: T) -> T {
    if b > a {
        return b;
    } else {
        return a;
    }
}


// generic function end here
*/

/*
// generics introduction start here

fn main() {
    let mut vec_int: Vec<i32> = vec![5, 20];
    vec_int.push(10);

    println!("Vec = :: {:#?} ::", vec_int);

    // not possible as the vec is of type int.
    // vec_int.push()
    let t: Data<i32> = Data { val: 10 };
    println!("val = :: {} ::", t.val);
    let t2: Data<String> = Data {
        val: "ABC".to_string(),
    };
    println!("val = :: {} ::", t2.val);

    let p = PointXY { x: 10, y: 20 };

    println!("x = :: {} :: and y = :: {} ::", p.x, p.y);

    let p = PointXY { x: 10.12, y: 20.0 };

    println!("x = :: {} :: and y = :: {} ::", p.x, p.y);
}

struct Data<T> {
    val: T,
}

struct PointXY<T> {
    x: T,
    y: T,
}


// generics introduction end here
*/

/*
unwrap and expect
use std::fs::File;

fn main() {
    let _f = File::open("dummys.txt").expect("The file can be anything idot so ignore this.");
}

 */

/*
Recoverable and unrecorable errors start here
use std::fs::File;

fn main() {
    // panic!("Some unexpected situation!");

    // let v = vec![1, 2, 3];
    // unrecoverable error and backtracking.
    // println!("{}", v[10]);

    let f = File::open("dummy.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening file :: {:?}", error)
        }
    };
}

Recoverable and unrecorable errors end here
*/
/*
Pattern matching start here
fn main() {
    print_tshirt_size(TShirtSize::Small);
    print_tshirt_size(TShirtSize::Medium);
    print_tshirt_size(TShirtSize::Large);
    print_tshirt_size(TShirtSize::Xlarge);
}

fn print_tshirt_size(size: TShirtSize) {
    match size {
        TShirtSize::Small => {
            println!("The shirt size is :: small:: !");
        }
        TShirtSize::Medium => {
            println!("The shirt size is :: Medium :: !");
        }
        TShirtSize::Large => {
            println!("The shirt size is :: Large :: !");
        }
        _ => {
            println!("The shirt default value is :: Xlarge ::")
        }
    }
}

enum TShirtSize {
    Small,
    Medium,
    Large,
    Xlarge,
}

Pattern matching end here
*/

/*
Option Enum start here
fn main() {
    let value = is_odd(10);
    println!("The option value returned is :: {:?} :: ", value);

    println!("The option value returned is :: {:?} :: ", is_odd(11));
}

fn is_odd(num: i32) -> Option<bool> {
    if num % 2 == 0 {
        None
    } else {
        Some(true)
    }
}

Option Enum end here
*/
/*
Enum started here
fn main() {
    let mon = Day::Monday;
    println!("mon :: {:?} ::", mon);
    let p = Person {
        name: String::from("ksjdbhfsdb"),
        birthday: Day::Friday,
    };
    println!("The person struct is :: {:?}", p);
}

// print of enum is implimented we need to mark with this to make it printable.
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
struct Person {
    name: String,
    birthday: Day,
}

Enum enum here
*/

/* ways of the struct methods start here
fn main() {
    let rect = Rectangle {
        width: 21,
        length: 23,
    };
    let area = rect.area(2);

    println!("The total area computed via the methods is :: {area} ::")
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self, i: u32) -> u32 {
        self.width * self.length * i
    }
}
ways of the struct methods end here
*/

/* ways of the struct start
// area of a rectangle
// different ways of doing things with and without different kinds of structs
fn main() {
    let length: u32 = 10;
    let width: u32 = 20;
    let area = area1(length, width);
    println!("The area is :: {area} ::");
    let rect1 = RectangleWL(10, 30);
    let area1 = area2(rect1);
    println!("The new area with tuple struct is :: {area1} ::");

    let rect2 = Reactangle {
        length: 10,
        width: 300,
    };
    let area2 = area3(rect2);
    println!("The new area with tuple struct is :: {area2} ::");
}

struct Reactangle {
    length: u32,
    width: u32,
}

struct RectangleWL(u32, u32);

fn area3(rect: Reactangle) -> u32 {
    rect.length * rect.width
}

fn area2(rect: RectangleWL) -> u32 {
    rect.0 * rect.1
}

fn area1(length: u32, width: u32) -> u32 {
    length * width
}

ways of the struct end
 */

/* struct starts here

// structs starts here
fn main() {
    let user1 = User {
        username: String::from("Deesh"),
        sign_in_count: 1,
        is_active: true,
        email: String::from("internetwasmyidea@gmail.com"),
    };

    println!(
        "Email id :: {} :: {} :: {} :: {} ::",
        user1.email, user1.is_active, user1.username, user1.sign_in_count
    );

    let user1: User = build_user(String::from("internetwasmyidea"), String::from("Somename"));
    println!(
        "Email id :: {} :: {} :: {} :: {} ::",
        user1.email, user1.is_active, user1.username, user1.sign_in_count
    );

    let user2: User = build_user_alter(String::from("internetwasmyidea"), String::from("Somename"));
    println!(
        "Email id :: {} :: {} :: {} :: {} ::",
        user2.email, user2.is_active, user2.username, user2.sign_in_count
    );

    // this is same as the below one
    let user3 = User {
        email: String::from("internetwasmyidea@gmail.com"),
        username: String::from("Somename"),
        is_active: user1.is_active,
        sign_in_count: user1.sign_in_count,
    };
    println!(
        "Email id :: {} :: {} :: {} :: {} ::",
        user3.email, user3.is_active, user3.username, user3.sign_in_count
    );

    let user4 = User {
        email: String::from("internetwsdfsdasmyidea@gmail.com"),
        ..user2
    };
    println!(
        "Email id :: {} :: {} :: {} :: {} ::",
        user4.email, user4.is_active, user4.username, user4.sign_in_count,
    );

    let red = ColorRGB(255, 0, 0);
    let blue = ColorRGB(0, 0, 255);
    let magenta = add_colors(red, blue);
    println!(
        "magenta Color is :: [{}, {}, {}]",
        magenta.0, magenta.1, magenta.2
    );
}

fn add_colors(col1: ColorRGB, col2: ColorRGB) -> ColorRGB {
    ColorRGB(col1.0 + col2.0, col1.1 + col2.1, col1.2 + col2.2)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        is_active: true,
        sign_in_count: 10,
    }
}

// this can be written as above as well which will be initialized auto.
fn build_user_alter(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        is_active: true,
        sign_in_count: 10,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

struct ColorRGB(i32, i32, i32);

struct(tuple and  normal structs)

 struct end here */

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
