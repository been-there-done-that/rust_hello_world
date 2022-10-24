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