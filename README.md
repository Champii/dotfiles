# Desired features

## Function signature

```haskell
add :: Num a => a -> a -> a
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
  to_string :: a -> String
```

