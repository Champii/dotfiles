# Desired features

## Reference/Dereference
## Unsafe pointer arithmetic
## Pattern matching
## Destructuring

```haskell
fn_return_tuple = -> (10, "a string")
main = ->
  (num, str) = fn_return_tuple!
```

## Automatic single argument (it)

```haskell
some_func = -> it + 2
```

## Functions as first class citizen

```haskell
call = f, x -> f x
return_fn = -> x -> x + 2
main = -> call return_fn!, 5
```

## Function signature

```haskell
add : a => a => a
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
add_mul = add . mul
```

## Custom operators

```haskell
infix |> 1
|> = x, f -> f x
main = -> [1, 2, 3] |> map (+2)
```

## Trait

```haskell
trait ToString
  @to_string : a => String

impl ToString Int
  @to_string = -> @show!
```

## Dynamic trait

```haskell
some_func : ToString a => String
some_fumc = x -> x.to_string!
```

## Structs

```haskell
struct Hello
  world : String
  new = s -> Hello world: s
  @display = -> @world.print!

main = ->
  hello = Hello::new "World"
  hello.display!
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
