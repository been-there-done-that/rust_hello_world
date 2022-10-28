# data types in rust
* scalar
    * Integer
        * u8
        * i8
        * u16
        * i16
        * u32
        * i32
        * u64
        * i64
        * u128
        * i128
        * default for integer is i32

    * Float
        * f32
        * f64
        * default float is f32

    * Boolean
        * let b1 = true
        * let b2: bool = false

    * Char
        * let c1 = 'a';
        * let c2 = '\u{1F601}' - unicode value will be prefixed with `\u`

* Compound
    * Tuples
        * collection of same/different data types.

    * Arrays
        * collection of same data types.

* expression vs statements
    * expression evaluates to something
    * and statements do something but won't return anything.

*****

* control flow contracts
    * if
    * else if
        * if <condition> {
          //code
          } else if <condition> {
          // other code
          } else {
          // default code
          }
    * if is an expression

    * loop
        * this is an infinite loop which will be run forever
        * `break` will be used to tell it to stop

    * while loop
        * while <condition> {
          // code to be executed.
          }


# memory management

* stack
    * LIFO - last in first out
    * grows upwards and shrinks dowwards
    * size must be know at the time of compiling
    * pushing on the stack is must faster than pushing into the heap.
    * the args to the function and the local variables to the function will be pushed to the stack when the scope of the function is done
      these values will be popped.
    *

* heap
    * unorganized data (with unknown size will be store in the heap)
    * memory will be alocated based on the availability


# ownership Rules

* Each value in rust has a variable that;s called it;s owner.
* There can only be one owner at a time
* When the owner goes out of scope, The value will be dropped.



* Types such as integers that have a known size at the compile time are stored entirely on the stack.
* there is a trait called copy which will let use the value after the copy as well
  we can use to annotate.

# OwnerShip:: References & Borrowing

* passing a values to a function is simpler to variable assignment(Assuming this is simpler to the case of the string
  assignment(s1, s2=s1::s1 is no longer valid)

# references - immutable references
* instead of expecting a string as input a function will expect a reference as input.
* borrow resouces without taking the ownership.

# reference - mutable references
* only difference is it will be `mut` keyword.
* rust won;t allow multiple mutable refences in the same code
    * this is cause to avoid a condition is called racecondition
        * Two or more pointers accessing the same data at the same time
        * Atleast one the pointer is being used to write to the data
        * There is not mechanism being used to synchronize access to the data.
    * multiple immutable can coexists in the same scope

* the combination of mutable and immutable reference might cause a race condition.


# slices
* let yiu reference a contiguous sequence of elements in a collection, rather than the whole collection.

# Struct

* custom data type make a meaningful Group
* building blocks of creating new data type
* Pieces can be of different types(like tuples)
* values can be accessed using dot(.) notation.
* ::syntax::
  ```rust
    struct Sample {
        abc: String,
        aef: i32
    }
    ```

# Tuple struct

* special type of struct (underneath they are tuples.)
* ::syntax::
    ```rust
      struct ColorRGB(i32, i32, i64);
  ```
* only type of simpler things can be passed around for typles as the language is of strictly typed.


# methods 

* similler to fucntions
* starts with fn keywords
* can have parameters
* will also return values
* difference is
  * defined within the context of struct(or an enum or a trait object))
  * first parameter is always self
* ::sytax::
    ```rust
  struct Reactangle {
  width: u32,
  length: u32
  }
  impl Rectange {
      fn area(&self) {
        self.width * self.height
        }
  }
    ```
* A single `impl` can contain more than one method.

# Enums
* Allow to define a type by enumerating it;s possible value
* it's good way os organizing the static values
* there can be only one value to be taken.
  * ::syntax::
  * ```rust
    enum Color {
        Red,
        Green,
        Blue,
    }
    ```
  * access values by `::` example : `Color::Red`
  
* We will use enums as the types to the struct.



# option Enum
* Option enum is predefined enum in rust standard library.
* this has two values
  * Some(data) 
  * None
* ::syntax::
  * ```rust
    enum Option<T>{
    Some(T),
    None
    }
    ```
    * the T represents value of any type(Generic type parameter.)
    * rust doesn't support the `null` Keyword.
    * The value `None` in enum option can be used by a function to return a null value.
    * if there is no data to return, the function return the Some(data).

# match statement & enum

* match statement can be used to compare values in an enum
* ::syntax::
  ```rust
     match size {
    TshirtSize::Small => println!("hlleo small");
    }
    ```
    * All match options should be exhaustive(if you don;t define the values from the enum in the match it
      will show it as error).
    * the placehold for the rest of the non defined options is `_`(placeholder).


# error handling
* Errors are of two groups
  * Recoverable errors (Result<T, E>)
  * Unrecoverable errors (Panic!)
* there is no exception in rust
* there is recoverable(Result) and panic(macro)
* panic 
  * when the panic macro executes, program will print a failure msg, unwind and cleanup the stack, and then quit
  * this occurs when bug of some kind detected and it;s not clean to programmers how to handle the error.
    * ::syntax::
      ```rust
      panic!("error message");
      ```
    * panic  can be coming from out code or some other code that our code is calling.
* recoverable errors
  * ::syntax::
    ```rust
    enum Result<T, E> {
    Ok(T),
    Err(E)
    }
      ```
#### As result is verbose sometimes it doesn't clearly commnucate the intent we will go for the advanced options (unwrap & expect)
# unwrap & expect
* shortcut for panic on error.
* if result value is ok variant, unwrap will return the value inside ok,
* if result is Err variant, unwrap will call the panic macro.
* expect will let us give the custom error message.

# Generics

* write code for multiple contexts with different types, more concise and clean code by reducing code complexity and type safty.
* can be applied to methods functions, structures, enumerations, collections and traits.
* the `<T>` syntax known as the type paramerter, is used to declare a generic construct. T represents any data-type.


# Generic Struct
* we can define struct to use a generic type parameter in one or more fileds using the `<>` syntax
  * ::syntax::
    ```rust
    struct Data<T> {
        val: T
    }
    
    ```

# Generic function
* place the generics in signature of function where we would usually specify data types of parameters and return value.
* Make code more flexible and provides more functionality to callers of our function.
* prevents code duplication.
* ::syntax::
  ```rust
    fn foo<t> (a: T, b: T) -> T {
    //..
    }
    ```
  * function foo is a generic over some type T.
  * This function has 2 parameter named a and b.
  * The function will return a value of the same type T.

# Traits

* A trait tells the rust compiler about fucntionality a particular type has and can share with other types
* Define shared behaviour in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behaviour.
* Similler to interfaces in other languages, with some differences.

# define traits(defining a trait)
* ::syntax::
  ```rust
    trait tait_x {
        fn method1(&self); // abstrac or method which is empty.
        
        // this is already implimented
        fn method2(&self) {  // concrete methods.
            //..
        } 
     }
    ```

# implementating traits for a struct

* ::syntax::
  ```rust
    struct A {...}
    trait trait1 {
        fn foo1(&self);
  }    
  
  impl trait1 for A {
    fn foo1(&self) {
        // todo!()
    } 
  }
    ```

# trait bounds

# stand I/O

* rust standard library features for i/o are organized around 2 traits.
    * read
        * types that impliment read have methods for byte-oriented input
          they are called `readers`.
        * Stdin, File
            * components that your program can read bytes from
                * ex: Reading input from keyboard, files, etc
            * `read_line()` method of this trait can be used to read data, one line at a time, from a file or
              standard input stream.
        * ::syntax::
          ```rust
            read_line(&mut line) -> Result
           ```
            * reads a line of text and appends it to line, which is a string.
            * the return value is an io::Result, the number of bytes read.
        * :ex:
          ```rust
            let reader  = std::io::stdin();
            let mut line = String::new();
            let b = reader.read_lin(&mut line).unwrap();
           ```

* write
    * Types that implement write support both byte-oriented and UTF_8 text output.
    * They are called writers.
    * Stdout, File
        * Components that your program can write bytes to
            * ex: Priting values to console, writing to files, etc
            * `write()` method of this trair can be used to write data to a file or standard output stream.
                * ::syntax::
              ```rust
                write(&buf) -> Result
               ```
                * write some of the bytes in the slice buf to the underlying stream.
                * the return value is an `io:Result`, the number of bytes written.
            * :ex:
              ```rust
                let mut writer  = std::io::stdout();
                let b = writer.write("xyz".as_bytes()).unwrap();
               ```

# file struct

* represents a file
* Allow read-write operations on a file
* All methods in file struct return a variant of io::Result enumeration.
    * widly used common methods.
        * `open()` - open in read-only
        * `create()` - open in write only, old content is destroyed.
        * `remove_file()` - remove file from filesystem.
        * `append()` - set option for append mode for file.
        * `write_all()` - attempt to write entire buffer into this write.
        * `read_to_string()` - read all bytes untill EOF in this source.
    * ::syntax::
      ```rust
        // create
        let mut f = create("hello.txt")
      
       // write
       write_all("first line");
       write_all("\nkjhbjkh");
      ```

#  