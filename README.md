# RustGlass

A dynamically typed esoteric programming language focusing on ease of use and performance.

## Examples

### FizzBuzz

```
func FizzBuzz(number) => 
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

### RadixSort

```
func radixSort(arr) =>
    max = arr[0] =>
    for i = 1 to n =>
        if arr[i] > max =>
            max = arr[i]
        end
    end

    for i = 1 to max =>
        bucket = [0] * 10
        for j = 1 to n =>
            bucket[arr[j] % 10] += 1
        end

        for j = 2 to 10 =>
            bucket[j] += bucket[j - 1]
        end

        for j = n to 1 step -1 =>
            bucket[arr[j] % 10] -= 1
            arr[bucket[arr[j] % 10]] = arr[j]
        end
    end

    return arr
end

println(radixSort([1, 3, 2, 4, 5]))
```