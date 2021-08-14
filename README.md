# RustGlass

TODO - README.md

### Example
```
func fizzBuzz(number) => 
    if number % 15 == 0 => 
        return "FizzBuzz"
    else if number % 3 == 0 =>
        return "Fizz"
    else if number % 5 == 0 =>
        return "Buzz"
    else 
        return number
    end
end

println(for i = 1 to 16 => fizzBuzz(i))
```

which would print `[1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz, 11, Fizz, 13, 14, FizzBuzz]` to console.