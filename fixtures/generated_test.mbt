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

// Lifetime/ownership tests - these Rust functions had &, Box, lifetimes
// but generate clean MoonBit that just works without any of that.

test "len_ref: &str dropped" {
  assert_eq(len_ref("hello"), 5)
}

test "inc_mut: &mut dropped" {
  assert_eq(inc_mut(10), 11)
}

test "unbox: Box unwrapped" {
  assert_eq(unbox(42), 42)
}

test "longer: lifetimes dropped" {
  assert_eq(longer("hi", "hello"), "hello")
}

test "find_ref: Option<&T> simplified" {
  let result : Option[Int] = find_ref([], 1)
  assert_eq(result, None)
}

test "make_rgb: derive struct" {
  let c = make_rgb(255, 0, 128)
  assert_eq(c.r, 255)
  assert_eq(c.g, 0)
  assert_eq(c.b, 128)
}

test "rc_value: Rc stripped to Int" {
  assert_eq(rc_value(42), 42)
}

test "arc_pair: Arc stripped to Int" {
  assert_eq(arc_pair(10, 20), 30)
}

test "apply: closure param" {
  let double = fn(x : Int) -> Int { x * 2 }
  assert_eq(apply(double, 5), 10)
}

test "extract_opt: if let → is" {
  assert_eq(extract_opt(Some(42)), 42)
  assert_eq(extract_opt(None), 0)
}

test "sign: match with guard" {
  assert_eq(sign(5), 1)
  assert_eq(sign(-3), -1)
  assert_eq(sign(0), 0)
}

test "fizzbuzz: nested if" {
  assert_eq(fizzbuzz(15), 0)
  assert_eq(fizzbuzz(3), 1)
  assert_eq(fizzbuzz(5), 2)
  assert_eq(fizzbuzz(7), 3)
}

test "quadratic: multiple let" {
  assert_eq(quadratic(2.0, 3.0, 1.0, 2.0), 15.0)
}

test "all_positive: bool ops" {
  assert_eq(all_positive(1, 2, 3), true)
  assert_eq(all_positive(1, -2, 3), false)
}

test "any_zero: bool ops" {
  assert_eq(any_zero(1, 0, 3), true)
  assert_eq(any_zero(1, 2, 3), false)
}

test "check_positive: return early" {
  assert_eq(check_positive(5), 10)
  assert_eq(check_positive(-3), 0)
  assert_eq(check_positive(0), 0)
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

test "float_val" {
  assert_true(float_val() > 3.14)
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

test "always_true" {
  assert_eq(always_true(), true)
}

test "always_false" {
  assert_eq(always_false(), false)
}

test "compare" {
  assert_eq(compare(1, 1), true)
  assert_eq(compare(1, 2), true)
}

test "three_nums" {
  assert_eq(three_nums(), [10, 20, 30])
}

test "maybe_val" {
  assert_eq(maybe_val(true), Some(42))
  assert_eq(maybe_val(false), None)
}

test "leaf" {
  let n = leaf(5)
  assert_eq(n.value, 5)
}

test "tree_sum" {
  let n = leaf(10)
  assert_eq(tree_sum(n), 10)
}


test "apply_twice" {
  let double = fn(x : Int) -> Int { x * 2 }
  assert_eq(apply_twice(double, 3), 12)
}

test "describe_num" {
  assert_eq(describe_num(0), "zero")
  assert_eq(describe_num(1), "one")
  assert_eq(describe_num(-5), "negative")
  assert_eq(describe_num(200), "big")
  assert_eq(describe_num(50), "other")
}

test "origin_3d" {
  let c = origin_3d()
  assert_eq(c.x, 0)
  assert_eq(c.y, 0)
  assert_eq(c.z, 0)
}

test "offset_coord" {
  let c = origin_3d()
  let c2 = offset_coord(c, 1, 2, 3)
  assert_eq(c2.x, 1)
  assert_eq(c2.y, 2)
  assert_eq(c2.z, 3)
}

test "clamp_positive" {
  assert_eq(clamp_positive(5), 5)
  assert_eq(clamp_positive(-3), 0)
}

test "abs_val" {
  assert_eq(abs_val(5), 5)
  assert_eq(abs_val(-3), 3)
}

test "sum_of_squares" {
  assert_eq(sum_of_squares(3), 14)
}

test "opposite direction" {
  assert_eq(opposite(North), South)
  assert_eq(opposite(East), West)
}

test "is_horizontal" {
  assert_eq(is_horizontal(East), true)
  assert_eq(is_horizontal(North), false)
}

test "LIMIT const" {
  assert_eq(LIMIT, 256)
}

