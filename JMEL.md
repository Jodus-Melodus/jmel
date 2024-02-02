# JMEL Scripting Language

## Symbols

`+` - addition
`-` - subtraction
`*` - multiplication
`/` - division
`%` - modulus (remainder)
`!`  - logical NOT
`&` - logical AND
`|` - logical OR
`=` - assignment operator
`==` - equal to
`!=` - not equal to
`<`  - less than
`>`  - greater than
`<=` - less or equals
`>=` - greater or equals
`//` - comments

## Datatypes

### `null`

- The Null datatype represents a null value.
- It has no properties or methods.

e.g.

```jmel
let x = null;
```

### `boolean`

- A Boolean datatype is used to represent logical values: true and false.
- It has two properties, `true` and `false`. These are read-only constants with the values 1 and 0 respectively.

e.g.

```jmel
let t = true;
let f = false;
```

### `integer`

- All numbers without any decimals.
- Supports (`+`, `-`, `*`, `/`, `%`)

e.g.

```jmel
3
99
```

### `real`

- All numbers with decimals.
- Supports (`+`, `-`, `*`, `/`)

e.g.

```jmel
10.5
27.84
```

### `string`

- A sequence of characters, enclosed in double `"` or single `'` quotes.
- Supports (`+`, `*`)

e.g.

```jmel
"Hello World!"
'This is a string.'
```

- Strings can be concatinated using `+`

```jmel
"hi" * 3 = "hihihi"
"Hello " + "world" = "Hello world"
```

- Strings can be indexed using `[]`.

e.g.

```jmel
let s = "Hello, World!";
s[0]                        // H
s[5]                        // ,
```

### `array`

- An array of values separated by commas and enclosed in square brackets `[...]`.
- Supports (`+`)

e.g.

```jmel
[1, 2, 3]
["Hello world", 2.3, [3]]
[[1, 2], [3, 4]]
```

- Arrays can be indexed using `[]`.

e.g.

```jmel
let arr = [4, 2, 9, 1];
arr[0]                      // 4
arr[2]                      // 9
```

### `tuple`

- Tuples allow you to assign multiple variables at a time.
- They are created with `(...)` syntax.
- Elements cannot be accessed or modified individually once they're assigned.

e.g.

```jmel
let (x, y) = (4, true);
x                           // 4
y                           // true

(x, y) = ("hi", 1)
x                           // "hi"
y                           // 1
```

### `object`

- A collection of key/value pairs separated by colons `:`, with each pair on its own line indented under the previous one.
- The keys are followed by a colon and then the value.
- Objects can be nested within other objects.

e.g.

```jmel
let obj = {
    a: 4,
    b           // uses the variable 'b'
};
```

## Hardcasting

- Hardcasting is when you change the datatype of a value.
- It is done using `as` or `to` around an expression.

e.g.

```jmel
let x = 5;

print(x as real);       // 5.0
print(x to string);     // "5"
```

## Variables

### Assigning

- Variables are assigned using the assignment operator `=` which assigns the value on its right to the variable on its left.
- When no value is specified the variable will default to `null`.

e.g.

```jmel
let x = 4;
let y;
```

### Constants

- A immutable variable.
- Declared with all caps.

e.g.

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

e.g.

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

e.g.

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

### User Defined Functions

Functions in JMEL are defined using the following syntax:
`func <name>([arguments]) { [code] }`

- The `<name>` is the function's identifier. It must start with a letter followed by any combination of letters, digits, or underscores.
- The `[arguments]` section lists the names that will represent the data provided by the caller when calling the function.
- The `[code]` section contains the code that the function will execute when it is called.

e.g.

```jmel
func add(a, b) { a + b }   // This function takes two arguments 'a' and 'b', adds them together and returns the result.
```
