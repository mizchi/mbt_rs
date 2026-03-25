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

test "rc_value: Rc stripped to Int" {
  assert_eq(rc_value(42), 42)
}

test "arc_pair: Arc stripped to Int" {
  assert_eq(arc_pair(10, 20), 30)
}
