# JMEL Scripting Language

## Symbols

- `+` addition
- `-` subtraction
- `*` multiplication
- `/` division
- `%` modulus (remainder)
- `!`  logical NOT
- `&` logical AND
- `|` logical OR
- `=` assignment operator
- `==` equal to
- `!=` not equal to
- `<`  less than
- `>`  greater than
- `<=` less or equals
- `>=` greater or equals
- `//` comments

## Datatypes

### `null`

- The Null datatype represents a null value.
- It has no properties or methods.

```jmel
let x = null;
```

---

### `boolean`

- A Boolean datatype is used to represent logical values: true and false.
- It has two properties, `true` and `false`. These are read-only constants with the values 1 and 0 respectively.

```jmel
let t = true;
let f = false;
```

---

### `integer`

- All numbers without any decimals.
- Supports (`+`, `-`, `*`, `/`, `%`)

```jmel
3
99
```

---

### `real`

- All numbers with decimals.
- Supports (`+`, `-`, `*`, `/`)

```jmel
10.5
27.84
```

---

### `string`

- A sequence of characters, enclosed in double `"` or single `'` quotes.

#### Declaring a string

```jmel
"Hello World!"
'This is a string.'
```

#### String concatination and operators

```jmel
"Hello " + "world" = "Hello world"
"hi" * 3 = "hihihi"
```

#### String indexing

```jmel
let s = "Hello, World!";
s[0]                        // H
s[5]                        // ,
```

#### String methods

##### string`.length()`

- Returns the length of the string.

---

### `array`

- An array of values separated by commas and enclosed in square brackets `[...]`.

#### Declaring an array

```jmel
[1, 2, 3]
["Hello world", 2.3, [3]]
[[1, 2], [3, 4]]
```

#### Array operators

```jmel
let x = [3, 2, 5];
x = x + 3;              // [3, 2, 5, 3]
```

#### Array indexing

- Arrays can be indexed using `[]`.

```jmel
let arr = [4, 2, 9, 1];
arr[0]                      // 4
arr[2]                      // 9
```

#### Array methods

##### array`.length()`

- Returns the number of elements in the array.

---

### `tuple`

- Tuples allow you to assign multiple variables at a time.
- Elements cannot be accessed or modified individually once they're assigned.

#### Declaring a tuple

```jmel
let x = tup(4, 1, true);
```

#### Tuple Operators

- Tuples support all arithmatic operators.

```jmel
let y = tup(7, 8);
x + y                   // (11, 9)
```

---

### `object`

- A collection of key/value pairs separated by colons `:`, with each pair on its own line indented under the previous one.
- The keys are followed by a colon and then the value.
- Objects can be nested within other objects.

#### Declaring an object

```jmel
let obj = {
    a: 4,
    b           // uses the variable 'b'
};
```

#### Object methods

##### object`.length()`

- Returns the number of properties in the object.

---

## Hardcasting

- Hardcasting is when you change the datatype of a value.
- It is done using `as` or `to` around an expression.

```jmel
let x = 5;

print(x as real);       // 5.0
print(x to string);     // "5"
```

## Variables

### Assigning

- Variables are assigned using the assignment operator `=` which assigns the value on its right to the variable on its left.
- When no value is specified the variable will default to `null`.

```jmel
let x = 4;
let y;
```

### Constants

- A immutable variable.
- Declared with all caps.

```jmel
let X = 3;
let HELLO = "world";
```

## Comparisons

### Comparisons with [int](#integer) and [flt](#real)

- Supports all the comparison operators

### Comparisons with [bool](#boolean)

- `==` | `!=` if the boolean values are equal or not

### Comparisons with [str](#string)

#### [str](#string) and [int](#integer)

- Compares the length of the string to the value

#### [str](#string) and [str](#string)

- `>` | `<` | `>=` | `<=` will compare lengths
- `==` | `!=` will compare characters (exact match)

### Comparisons with [array](#array)

#### [array](#array) and [int](#integer)

- Compares the length of the array to the value

#### [array](#array) and [array](#array)

- `>` | `<` | `>=` | `<=` will compare lengths
- `==` | `!=` will compare values (exact match)

## Conditional Statements

### If Statement

- If an evaluated condition evaluates to `true` then it will execute the appropriate code block.
- If statements can contain an else block that will execute when the condition evaluates to `false`.

```jmel
if 3 == 2 {
    print("This won't be printed");
} else {
    print("This will be printed");
}
```

### Case Statement

- A case statement is similar to a series of if/else statements, but allows for multiple conditions to be tested easily.
- When the `null` case is reached it will execute no matter what, so always put it last.

```jmel
case "hello" of {
    "hello" : {
        print("hello");
    };
    "world" : {
        print("world");
    };
    null : {
        print("default");
    };
};
```

- The output will be 'hello'

## Functions

### Built-In Functions

#### `print()`

Description:

- Prints the arguments supplied to the console.

Arguments:

- Any number of values can be passed as arguments.

Example:

```jmel
print("Hello", ", ", "World!");     // Output: Hello, World!
```

#### `input()`

Description:

- Prompts the user for input and returns it as a string.

Arguments:

- Any number of values can be passed as arguments.

Example:

```jmel
let name = input("Please enter your name: ");                  // Output (prompt): Please enter your name:
```

#### `tup()`

Description:

- Creates a new tuple with supplied values

Arguments:

- Any number of values can be passed as arguments.

Example:

```jmel
tup("Hello", 3, true);     // Output: ("Hello", 3, true)
```

### User Defined Functions

Functions in JMEL are defined using the following syntax:
`func <name>([arguments]) { [code] }`

- The `<name>` is the function's identifier. It must start with a letter followed by any combination of letters, digits, or underscores.
- The `[arguments]` section lists the names that will represent the data provided by the caller when calling the function.
- The `[code]` section contains the code that the function will execute when it is called.

```jmel
func add(a, b) { a + b }   // This function takes two arguments 'a' and 'b', adds them together and returns the result.
```
