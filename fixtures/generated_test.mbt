test "add" {
  assert_eq(add(1, 2), 3)
}

test "max" {
  assert_eq(max(3, 5), 5)
  assert_eq(max(7, 2), 7)
}

test "swap" {
  assert_eq(swap(1, 2), (2, 1))
}

test "compute" {
  assert_eq(compute(), 3)
}

test "counter" {
  assert_eq(counter(), 10)
}

test "sum_to" {
  assert_eq(sum_to(10), 55)
}

test "safe_div" {
  assert_eq(safe_div(10, 2), Some(5))
  assert_eq(safe_div(10, 0), None)
}

test "negate" {
  assert_eq(negate(5), -5)
  assert_eq(negate(-3), 3)
}

test "clamp" {
  assert_eq(clamp(5, 0, 10), 5)
  assert_eq(clamp(-1, 0, 10), 0)
  assert_eq(clamp(15, 0, 10), 10)
}

test "unwrap_or" {
  assert_eq(unwrap_or(Some(5), 0), 5)
  assert_eq(unwrap_or(None, 0), 0)
}

test "is_weekend" {
  assert_eq(is_weekend(6), true)
  assert_eq(is_weekend(7), true)
  assert_eq(is_weekend(3), false)
}

test "classify" {
  assert_eq(classify(5), 1)
  assert_eq(classify(-3), -1)
  assert_eq(classify(0), 0)
}

test "fibonacci" {
  assert_eq(fibonacci(0), 0)
  assert_eq(fibonacci(1), 1)
  assert_eq(fibonacci(10), 55)
}

test "first_of_pair" {
  assert_eq(first_of_pair((10, 20)), 10)
}

test "divmod" {
  assert_eq(divmod(10, 3), (3, 1))
}

test "len_ref" {
  assert_eq(len_ref("hello"), 5)
}

test "inc_mut" {
  assert_eq(inc_mut(10), 11)
}

test "unbox" {
  assert_eq(unbox(42), 42)
}

test "longer" {
  assert_eq(longer("hi", "hello"), "hello")
}

test "find_ref" {
  let result : Option[Int] = find_ref([], 1)
  assert_eq(result, None)
}

test "make_rgb" {
  let c = make_rgb(255, 0, 128)
  assert_eq(c.r, 255)
  assert_eq(c.g, 0)
  assert_eq(c.b, 128)
}

test "rc_value" {
  assert_eq(rc_value(42), 42)
}

test "arc_pair" {
  assert_eq(arc_pair(10, 20), 30)
}

test "apply" {
  let double = fn(x : Int) -> Int { x * 2 }
  assert_eq(apply(double, 5), 10)
}

test "extract_opt" {
  assert_eq(extract_opt(Some(42)), 42)
  assert_eq(extract_opt(None), 0)
}

test "sign" {
  assert_eq(sign(5), 1)
  assert_eq(sign(-3), -1)
  assert_eq(sign(0), 0)
}

test "fizzbuzz" {
  assert_eq(fizzbuzz(15), 0)
  assert_eq(fizzbuzz(3), 1)
  assert_eq(fizzbuzz(5), 2)
  assert_eq(fizzbuzz(7), 3)
}

test "quadratic" {
  assert_eq(quadratic(2.0, 3.0, 1.0, 2.0), 15.0)
}

test "all_positive" {
  assert_eq(all_positive(1, 2, 3), true)
  assert_eq(all_positive(1, -2, 3), false)
}

test "any_zero" {
  assert_eq(any_zero(1, 0, 3), true)
  assert_eq(any_zero(1, 2, 3), false)
}

test "check_positive" {
  assert_eq(check_positive(5), 10)
  assert_eq(check_positive(-3), 0)
}

test "typed_let" {
  assert_eq(typed_let(), 50)
}

test "nested_match" {
  assert_eq(nested_match(Some(Some(42))), 42)
  assert_eq(nested_match(Some(None)), -1)
  assert_eq(nested_match(None), -2)
}

test "int64_val" {
  assert_eq(int64_val(), 123456789L)
}

test "noop" {
  noop()
}

test "min_max" {
  assert_eq(min_max(3, 7), (3, 7))
  assert_eq(min_max(9, 2), (2, 9))
}

test "tuple access" {
  assert_eq(first((10, 20)), 10)
  assert_eq(second((10, 20)), 20)
}

test "factorial" {
  assert_eq(factorial(5), 120)
  assert_eq(factorial(0), 1)
}

test "grade" {
  assert_eq(grade(95), 5)
  assert_eq(grade(85), 4)
  assert_eq(grade(55), 1)
}

test "is_vowel" {
  assert_eq(is_vowel('a'), true)
  assert_eq(is_vowel('b'), false)
}

test "complex_calc" {
  assert_eq(complex_calc(10, 20, 30), 30)
}

test "make_point" {
  let p = make_point(3, 4)
  assert_eq(get_point_x(p), 3)
}

test "get_second" {
  assert_eq(get_second([10, 20, 30]), 20)
}

test "greeting" {
  assert_eq(greeting(), "hello")
}

test "initial" {
  assert_eq(initial(), 'A')
}

test "make_pair" {
  let p = make_pair(1, "hello")
  assert_eq(get_pair_first(p), 1)
}

test "LIMIT" {
  assert_eq(LIMIT, 256)
}

test "always_true" {
  assert_eq(always_true(), true)
}

test "compare" {
  assert_eq(compare(1, 1), true)
}

test "three_nums" {
  assert_eq(three_nums(), [10, 20, 30])
}

test "maybe_val" {
  assert_eq(maybe_val(true), Some(42))
  assert_eq(maybe_val(false), None)
}

test "apply_twice" {
  let double = fn(x : Int) -> Int { x * 2 }
  assert_eq(apply_twice(double, 3), 12)
}

test "opposite" {
  assert_eq(opposite(North), South)
  assert_eq(opposite(East), West)
}

test "is_horizontal" {
  assert_eq(is_horizontal(East), true)
  assert_eq(is_horizontal(North), false)
}

test "parse_int" {
  assert_eq(parse_int("42"), Ok(42))
  assert_eq(parse_int(""), Err("empty string"))
}

test "safe_divide" {
  assert_eq(safe_divide(10.0, 2.0), Ok(5.0))
  assert_true(safe_divide(10.0, 0.0) is Err(_))
}

test "is_adult" {
  let p : Person = { name: "Alice", age: 30, address: { city: "Tokyo", zip: "100" } }
  assert_eq(is_adult(p), true)
}

test "eval_op" {
  assert_eq(eval_op("add", 3, 4), 7)
  assert_eq(eval_op("mul", 3, 4), 12)
  assert_eq(eval_op("div", 10, 0), 0)
}

test "or_default" {
  assert_eq(or_default(Some(42)), 42)
  assert_eq(or_default(None), 0)
}

test "contains_val" {
  assert_eq(contains_val([1, 2, 3], 2), true)
  assert_eq(contains_val([1, 2, 3], 5), false)
}

test "color_code" {
  assert_eq(color_code("red"), 1)
  assert_eq(color_code("blue"), 3)
  assert_eq(color_code("?"), -1)
}

test "gcd" {
  assert_eq(gcd(12, 8), 4)
  assert_eq(gcd(17, 13), 1)
}

test "power" {
  assert_eq(power(2, 10), 1024)
  assert_eq(power(3, 3), 27)
}

test "is_even" {
  assert_eq(is_even(4), true)
  assert_eq(is_even(7), false)
}

test "make_empty_vec" {
  assert_eq(make_empty_vec(), [])
}

test "option_unwrap_or" {
  assert_eq(option_unwrap_or(Some(5)), 5)
  assert_eq(option_unwrap_or(None), 0)
}

test "option_is_some" {
  assert_eq(option_is_some(Some(1)), true)
  assert_eq(option_is_some(None), false)
}

test "str_to_lower" {
  assert_eq(str_to_lower("HELLO"), "hello")
}

test "str_to_upper" {
  assert_eq(str_to_upper("hello"), "HELLO")
}

test "str_starts_with" {
  assert_eq(str_starts_with("hello world", "hello"), true)
  assert_eq(str_starts_with("hello world", "world"), false)
}

test "str_trim" {
  assert_eq(str_trim("  hello  "), "hello")
}

test "bytes_length" {
  assert_eq(bytes_length(b"hello"), 5)
}

test "count_byte" {
  assert_eq(count_byte(b"hello", b'l'), 2)
}

test "is_prime" {
  assert_eq(is_prime(2), true)
  assert_eq(is_prime(17), true)
  assert_eq(is_prime(4), false)
  assert_eq(is_prime(1), false)
}

test "apply_fn" {
  assert_eq(apply_fn(fn(x) { x * 3 }, 4), 12)
}

test "compose" {
  assert_eq(compose(fn(x) { x + 1 }, fn(x) { x * 2 }, 5), 11)
}

test "map_array" {
  let double = fn(x : Int) -> Int { x * 2 }
  assert_eq(map_array([1, 2, 3], double), [2, 4, 6])
}

test "filter_array" {
  let positive = fn(x : Int) -> Bool { x > 0 }
  assert_eq(filter_array([1, -2, 3, -4, 5], positive), [1, 3, 5])
}

test "fold_array" {
  let add = fn(a : Int, b : Int) -> Int { a + b }
  assert_eq(fold_array([1, 2, 3, 4], 0, add), 10)
}

test "count_char" {
  assert_eq(count_char("hello world", 'l'), 3)
}

test "clamp_f64" {
  assert_eq(clamp_f64(0.5, 0.0, 1.0), 0.5)
  assert_eq(clamp_f64(-1.0, 0.0, 1.0), 0.0)
  assert_eq(clamp_f64(2.0, 0.0, 1.0), 1.0)
}

test "lerp" {
  assert_eq(lerp(0.0, 10.0, 0.5), 5.0)
  assert_eq(lerp(0.0, 10.0, 0.0), 0.0)
  assert_eq(lerp(0.0, 10.0, 1.0), 10.0)
}

test "sum_array" {
  assert_eq(sum_array([1, 2, 3, 4, 5]), 15)
  assert_eq(sum_array([]), 0)
}

test "product_array" {
  assert_eq(product_array([1, 2, 3, 4]), 24)
}

test "http_status" {
  assert_eq(http_status(200), "OK")
  assert_eq(http_status(404), "Not Found")
  assert_eq(http_status(999), "Unknown")
}

test "find_index" {
  assert_eq(find_index([10, 20, 30], 20), 1)
  assert_eq(find_index([10, 20, 30], 99), -1)
}

test "all_positive_arr" {
  assert_eq(all_positive_arr([1, 2, 3]), true)
  assert_eq(all_positive_arr([1, -2, 3]), false)
  assert_eq(all_positive_arr([]), true)
}

test "double_if_positive" {
  assert_eq(double_if_positive(5), Some(10))
  assert_eq(double_if_positive(-3), None)
}

test "chain_options" {
  assert_eq(chain_options(Some(3), Some(4)), Some(7))
  assert_eq(chain_options(Some(3), None), None)
  assert_eq(chain_options(None, Some(4)), None)
}

test "chain_checked" {
  assert_eq(chain_checked(1, 2, 3, 100), Ok(6))
  assert_true(chain_checked(50, 50, 50, 100) is Err(_))
}

test "sum_points_x" {
  let points = [{ x: 1, y: 0 }, { x: 2, y: 0 }, { x: 3, y: 0 }]
  assert_eq(sum_points_x(points), 6)
}

test "running_sum" {
  assert_eq(running_sum([1, 2, 3, 4]), [1, 3, 6, 10])
}

test "max_in_array" {
  assert_eq(max_in_array([3, 1, 4, 1, 5]), Some(5))
  assert_eq(max_in_array([]), None)
}

test "min_in_array" {
  assert_eq(min_in_array([3, 1, 4, 1, 5]), Some(1))
  assert_eq(min_in_array([]), None)
}

test "count_words" {
  assert_eq(count_words("hello world"), 2)
  assert_eq(count_words("  hello   world  "), 2)
  assert_eq(count_words(""), 0)
}

test "in_range" {
  assert_eq(in_range(5, 1, 10), true)
  assert_eq(in_range(0, 1, 10), false)
}

test "clamp_to_byte" {
  assert_eq(clamp_to_byte(128), 128)
  assert_eq(clamp_to_byte(-5), 0)
  assert_eq(clamp_to_byte(300), 255)
}

test "wrap_around" {
  assert_eq(wrap_around(5, 3), 2)
  assert_eq(wrap_around(-1, 5), 4)
}

test "dot_product" {
  assert_eq(dot_product([1, 2, 3], [4, 5, 6]), 32)
}

test "pairwise_max" {
  assert_eq(pairwise_max([1, 5, 3], [4, 2, 6]), [4, 5, 6])
}

test "transpose_2x2" {
  assert_eq(transpose_2x2(1, 2, 3, 4), (1, 3, 2, 4))
}

test "classify_triangle" {
  assert_eq(classify_triangle(3, 3, 3), "equilateral")
  assert_eq(classify_triangle(3, 3, 5), "isosceles")
  assert_eq(classify_triangle(3, 4, 5), "scalene")
  assert_eq(classify_triangle(1, 2, 10), "invalid")
}

test "fizzbuzz_string" {
  assert_eq(fizzbuzz_string(15), "FizzBuzz")
  assert_eq(fizzbuzz_string(3), "Fizz")
  assert_eq(fizzbuzz_string(5), "Buzz")
  assert_eq(fizzbuzz_string(7), "7")
}

test "flatten_nested" {
  assert_eq(flatten_nested([[1, 2], [3, 4], [5]]), [1, 2, 3, 4, 5])
}

test "take_n" {
  assert_eq(take_n([1, 2, 3, 4, 5], 3), [1, 2, 3])
  assert_eq(take_n([1, 2], 5), [1, 2])
}

test "drop_n" {
  assert_eq(drop_n([1, 2, 3, 4, 5], 2), [3, 4, 5])
}

test "is_digit_string" {
  assert_eq(is_digit_string("123"), true)
  assert_eq(is_digit_string("12a"), false)
  assert_eq(is_digit_string(""), false)
}

test "char_at" {
  assert_eq(char_at("hello", 1), Some('e'))
  assert_eq(char_at("hello", 10), None)
}

test "lcm" {
  assert_eq(lcm(4, 6), 12)
  assert_eq(lcm(0, 5), 0)
}

test "collatz_steps" {
  assert_eq(collatz_steps(1), 0)
  assert_eq(collatz_steps(6), 8)
}

test "digit_sum" {
  assert_eq(digit_sum(123), 6)
  assert_eq(digit_sum(0), 0)
  assert_eq(digit_sum(-42), 6)
}

test "is_power_of_two" {
  assert_eq(is_power_of_two(1), true)
  assert_eq(is_power_of_two(8), true)
  assert_eq(is_power_of_two(6), false)
}
