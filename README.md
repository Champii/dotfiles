# Desired features

## Function currying

```haskell
main = ->
  add4 = add 4
  add4 2

add = x, y -> x + y
```

## Custom operators

```haskell
infix |> 1
|> = x, f -> f x
```
