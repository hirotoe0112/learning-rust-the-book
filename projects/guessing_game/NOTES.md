- Variables are immutable by default.
  ```rust
  let apples = 5;
  ```
- To make variables mutable, add `mut`.
  ```rust
  let mut apples = 5;
  ```
- The `::` syntax indicates an associated function.
  - An associated function is a function that is implemented on a type.
    ```rust
    String::new
    ```
- The `std::io::Stdin` type represents a handle to the standard input.
  - Create a instance using `std::io::stdin`.
  - Get input from the user using `read_line`.
    ```rust
    io::stdin().read_line(&mut guess)
    ```
- The `&` indicates that the argument is a reference.
  - By default, a reference is immutable.
    ```rust
    &immutable 
    &mut mutable
    ```
- Shadow the previous variable with a new one.
  ```rust
  let mut guess = String::new();
  let guess: u32 = guess.trim().parse.expect("Please type a number");
  ```