# Desired features

## Chain calls without defining variable

```haskell
write_file = ->
  File::open "test.txt"?
   ..write "Hello, World!"?
   ..close!
```

## Named arguments and default value

```haskell
listen = addr, port = 8000 -> #some code here
main = ->
  if something
    listen "localhost"
  else if something_else
    listen port: 8080, addr: "localhost"
```

## Functions as first class citizen

```haskell
call = f, x -> f x
return_fn = -> x -> x + 2
main = -> call return_fn!, 5
```

## Function signature

```haskell
add : T -> T -> T
add = x, y -> x + y
```

## Function currying

```haskell
add = x, y -> x + y
add4 = add 4
main = -> add4 2
```

## Function Shorthand

```haskell
plus2 = (+2)
main = -> plus2 2
```

## Function composition

```haskell
add = x -> x + 2
mul = x -> x * 2
add_mul = mul . add
```

## Custom operators

```haskell
infix 1 |> = x, f -> f x

main = -> [1, 2, 3] |> map (+2)
```

## Trait

```haskell
trait ToString
  @to_string : String

impl ToString Int
  @to_string = -> @show!
```

## Dynamic trait

```haskell
some_func : ToString T -> String
some_fumc = x -> x.to_string!
```

## Structs

```haskell
struct Hello
  world : String
  some_default_field = true

impl Hello
  new = s -> Hello world: s
  @display = -> @world.print!

main = ->
  hello = Hello::new "World"
  hello.display!
```

## Generics

```haskell
struct Wrapper T
  inner: T

enum Choice T, U
  Left T
  Right U
```

## Automatic Reference/Dereference

```haskell
main = ->
  a = 5
  #Here b is &Int
  b = a 
  add a, b

#Will detect if one is a reference and autoderef if needed
add = a, b -> a + b
```
## Unsafe pointer arithmetic

```haskell
main = ->
  unsafe
    a = 42
    p: *Int8 = 0
    # very unsafe
    *p
```

## Pattern matching

```haskell
main = ->
  a = (10, "hello")
  match a
    (0, "world")      => "something"
    (a, str) if a > 5 => str
    _                 => "otherwise"
```

## Destructuring

```haskell
fn_return_tuple = -> (10, "a string")
main = ->
  (num, str) = fn_return_tuple!
```

## Enums

```haskell
enum Error
  SomeError
  SomeErrorWithContext String
  SomeErrorWithMoreContext String, Int

impl Show Error
  @show = ->
    match @
      Self::SomeError => "SomeError"
      Self::SomeErrorWithContext s => "SomeError" + s
      Self::SomeErrorWithMoreContext s, i => "SomeError" + s + i.show!

main = -> Error::SomeErrorWithContext "Hello" .print!
```

## Macros

```haskell
macro generate
  $name:ident, $type:ty =>
    struct Generated
      $name: $type

%generate hello, String
```

## Modules and dependancies

`Rock.toml`  
```toml
name = "my_awesome_package"

#[dependencies]
my_dep = [ path = "../my_dep" ]
```

`lib.rk`  
```haskell
mod my_mod

> my_dep::SomeType
> my_mod::SomeOtherType

struct MyStruct
  my_field: SomeType

< MyStruct
```


# TODO

  - Macros
    - Macro nested var repetition $($($arg:ident)*)*
    - Don't ignore \n and indent for macro args parsing
    - Better error management and diagnostic details
  - Parser
    - Operator infix declaration
    - Adaptative indentation: Scan the first indentation level and set it as default
    - Pattern matching and destructuring
    - Allow empty lines in block and anywhere basically
    - Allow multiline function arguments
    - Default arguments and named arguments
    - Mod management
    - Import/Export
    - Trait
    - Error bubbling with `?`
    - Empty fn call with `!`
    - Function signature
    - Tuples
    - Struct default value
    - Struct default constructor
    - Chain calls without variables
    - Multiline dot chaining
    - Space dot closes function call
    - Comments
    - Async?
  - Desugar
    - Operator precedence
    - Loops into `loop`
    - Match into `if`
    - Dot notation into function calls
    - Closures into functiondecl
  - Modules and dependancies
    - Basic external module management
  - Code formating ?
  - Rust FFI

