## A Note on Mutability and References in Rust

In Rust, variables are immutable by default. This means that once a variable is assigned a value, it cannot be changed. To declare a variable as mutable, allowing its value to be changed after initialisation, the mut keyword is used.

The & symbol in Rust is primarily creating references to values. A reference provides a way to let multiple parts of code access one piece of data without needing to copy that data into memory multiple times.

There are two types of references:

Immutable (&T) - The default, allowing read-only access to the value referenced
Mutable (&mut T) - allows read and write access to the value
Dereferencing in Rust is accomplished with the * operator, which is used to access the value pointed to by a reference.

## Strings
Keywords for strings:

- `String` - standard string Type - variable length - vector of unsigned 8 bit integers under the hood vec<u8>
- `str` - fixed length, immutable string type - usually seen as:
- `&str` - fixed length immutable string with borrowed value

### Useful to handle ownership:
- `.to_owned()` - this allows you to change the ownership of a value in memory such that you can perform write operations
- `.clone()` - this allows you to copy the value in memory such that the new copy can be mutated without interfering with the original value, with a different owner

More on Ownership, Borrowing and Lifetimes in the Memory Management section.

## Slices
To take a slice of a String, str or &str, one can use the following:

```Rust 
let s = String::from("example text")

let slice_of_s = &s[0..1] // returns the first character from the string s defined above

// Alternatively, we can reference the original string's length so we can make references depend on the length of the string, which may at some point change

let len_string = s.len()

let length_based_slice_of_s = &s[0..len_string]
```
