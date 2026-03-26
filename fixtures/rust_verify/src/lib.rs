// This file is auto-generated. Do not edit.
// It includes the fixture functions and behavioral tests.
// The same assertions exist in generated_test.mbt for MoonBit.
// Both must pass → behavioral equivalence.

#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

include!("../../input.rs");

#[cfg(test)]
mod behavioral_tests {
    use super::*;

    #[test] fn test_add() { assert_eq!(add(1, 2), 3); }
    #[test] fn test_max() { assert_eq!(max(3, 5), 5); assert_eq!(max(7, 2), 7); }
    #[test] fn test_swap() { assert_eq!(swap(1, 2), (2, 1)); }
    #[test] fn test_compute() { assert_eq!(compute(), 3); }
    #[test] fn test_counter() { assert_eq!(counter(), 10); }
    #[test] fn test_sum_to() { assert_eq!(sum_to(10), 55); }
    #[test] fn test_safe_div() { assert_eq!(safe_div(10, 2), Some(5)); assert_eq!(safe_div(10, 0), None); }
    #[test] fn test_negate() { assert_eq!(negate(5), -5); assert_eq!(negate(-3), 3); }
    #[test] fn test_clamp() { assert_eq!(clamp(5, 0, 10), 5); assert_eq!(clamp(-1, 0, 10), 0); assert_eq!(clamp(15, 0, 10), 10); }
    #[test] fn test_unwrap_or() { assert_eq!(unwrap_or(Some(5), 0), 5); assert_eq!(unwrap_or(None, 0), 0); }
    #[test] fn test_is_weekend() { assert_eq!(is_weekend(6), true); assert_eq!(is_weekend(3), false); }
    #[test] fn test_classify() { assert_eq!(classify(5), 1); assert_eq!(classify(-3), -1); assert_eq!(classify(0), 0); }
    #[test] fn test_fibonacci() { assert_eq!(fibonacci(0), 0); assert_eq!(fibonacci(1), 1); assert_eq!(fibonacci(10), 55); }
    #[test] fn test_first_of_pair() { assert_eq!(first_of_pair((10, 20)), 10); }
    #[test] fn test_divmod() { assert_eq!(divmod(10, 3), (3, 1)); }
    #[test] fn test_len_ref() { assert_eq!(len_ref("hello"), 5); }
    #[test] fn test_inc_mut() { assert_eq!(inc_mut(&mut 10), 11); }
    #[test] fn test_unbox() { assert_eq!(unbox(Box::new(42)), 42); }
    #[test] fn test_longer() { assert_eq!(longer("hi", "hello"), "hello"); }
    #[test] fn test_find_ref() { assert_eq!(find_ref(&vec![], 1), None); }
    #[test] fn test_make_rgb() { let c = make_rgb(255, 0, 128); assert_eq!(c.r, 255); assert_eq!(c.g, 0); assert_eq!(c.b, 128); }
    #[test] fn test_rc_value() { assert_eq!(rc_value(Rc::new(42)), 42); }
    #[test] fn test_arc_pair() { assert_eq!(arc_pair(Arc::new(10), Arc::new(20)), 30); }
    #[test] fn test_apply() { assert_eq!(apply(|x| x * 2, 5), 10); }
    #[test] fn test_extract_opt() { assert_eq!(extract_opt(Some(42)), 42); assert_eq!(extract_opt(None), 0); }
    #[test] fn test_sign() { assert_eq!(sign(5), 1); assert_eq!(sign(-3), -1); assert_eq!(sign(0), 0); }
    #[test] fn test_fizzbuzz() { assert_eq!(fizzbuzz(15), 0); assert_eq!(fizzbuzz(3), 1); assert_eq!(fizzbuzz(5), 2); }
    #[test] fn test_quadratic() { assert_eq!(quadratic(2.0, 3.0, 1.0, 2.0), 15.0); }
    #[test] fn test_all_positive() { assert!(all_positive(1, 2, 3)); assert!(!all_positive(1, -2, 3)); }
    #[test] fn test_any_zero() { assert!(any_zero(1, 0, 3)); assert!(!any_zero(1, 2, 3)); }
    #[test] fn test_check_positive() { assert_eq!(check_positive(5), 10); assert_eq!(check_positive(-3), 0); }
    #[test] fn test_typed_let() { assert_eq!(typed_let(), 50); }
    #[test] fn test_nested_match() { assert_eq!(nested_match(Some(Some(42))), 42); assert_eq!(nested_match(Some(None)), -1); assert_eq!(nested_match(None), -2); }
    #[test] fn test_int64_val() { assert_eq!(int64_val(), 123456789i64); }
    #[test] fn test_noop() { noop(); }
    #[test] fn test_min_max() { assert_eq!(min_max(3, 7), (3, 7)); assert_eq!(min_max(9, 2), (2, 9)); }
    #[test] fn test_tuple_access() { assert_eq!(first((10, 20)), 10); assert_eq!(second((10, 20)), 20); }
    #[test] fn test_factorial() { assert_eq!(factorial(5), 120); assert_eq!(factorial(0), 1); }
    #[test] fn test_grade() { assert_eq!(grade(95), 5); assert_eq!(grade(85), 4); assert_eq!(grade(55), 1); }
    #[test] fn test_is_vowel() { assert!(is_vowel('a')); assert!(!is_vowel('b')); }
    #[test] fn test_complex_calc() { assert_eq!(complex_calc(10, 20, 30), 30); }
    #[test] fn test_make_point() { let p = make_point(3, 4); assert_eq!(get_point_x(p), 3); }
    #[test] fn test_get_second() { assert_eq!(get_second(vec![10, 20, 30]), 20); }
    #[test] fn test_greeting() { assert_eq!(greeting(), "hello"); }
    #[test] fn test_initial() { assert_eq!(initial(), 'A'); }
    #[test] fn test_always_true() { assert!(always_true()); }
    #[test] fn test_compare() { assert!(compare(1, 1)); }
    #[test] fn test_three_nums() { assert_eq!(three_nums(), vec![10, 20, 30]); }
    #[test] fn test_maybe_val() { assert_eq!(maybe_val(true), Some(42)); assert_eq!(maybe_val(false), None); }
    #[test] fn test_apply_twice() { assert_eq!(apply_twice(|x| x * 2, 3), 12); }
    #[test] fn test_opposite() { assert_eq!(opposite(Direction::North), Direction::South); assert_eq!(opposite(Direction::East), Direction::West); }
    #[test] fn test_is_horizontal() { assert!(is_horizontal(&Direction::East)); assert!(!is_horizontal(&Direction::North)); }
    #[test] fn test_parse_int() { assert_eq!(parse_int("42"), Ok(42)); assert_eq!(parse_int(""), Err("empty string".to_string())); }
    #[test] fn test_safe_divide() { assert_eq!(safe_divide(10.0, 2.0), Ok(5.0)); assert!(safe_divide(10.0, 0.0).is_err()); }
    #[test] fn test_is_adult() { let p = Person { name: "Alice".to_string(), age: 30, address: Address { city: "Tokyo".to_string(), zip: "100".to_string() } }; assert!(is_adult(&p)); }
    #[test] fn test_eval_op() { assert_eq!(eval_op("add", 3, 4), 7); assert_eq!(eval_op("mul", 3, 4), 12); assert_eq!(eval_op("div", 10, 0), 0); }
    #[test] fn test_or_default() { assert_eq!(or_default(Some(42)), 42); assert_eq!(or_default(None), 0); }
    #[test] fn test_contains_val() { assert!(contains_val(&vec![1,2,3], 2)); assert!(!contains_val(&vec![1,2,3], 5)); }
    #[test] fn test_color_code() { assert_eq!(color_code("red"), 1); assert_eq!(color_code("blue"), 3); assert_eq!(color_code("?"), -1); }
    #[test] fn test_gcd() { assert_eq!(gcd(12, 8), 4); assert_eq!(gcd(17, 13), 1); }
    #[test] fn test_power() { assert_eq!(power(2, 10), 1024); assert_eq!(power(3, 3), 27); }
    #[test] fn test_is_even() { assert!(is_even(4)); assert!(!is_even(7)); }
    #[test] fn test_bytes_length() { assert_eq!(bytes_length(b"hello"), 5); }
    #[test] fn test_count_byte() { assert_eq!(count_byte(b"hello", b'l'), 2); }
    #[test] fn test_apply_fn() { assert_eq!(apply_fn(|x| x * 3, 4), 12); }
    #[test] fn test_compose() { assert_eq!(compose(|x| x + 1, |x| x * 2, 5), 11); }
    #[test] fn test_map_array() { assert_eq!(map_array(vec![1,2,3], |x| x*2), vec![2,4,6]); }
    #[test] fn test_filter_array() { assert_eq!(filter_array(vec![1,-2,3,-4,5], |x| x>0), vec![1,3,5]); }
    #[test] fn test_fold_array() { assert_eq!(fold_array(vec![1,2,3,4], 0, |a,b| a+b), 10); }
    #[test] fn test_count_char() { assert_eq!(count_char("hello world", 'l'), 3); }
    #[test] fn test_clamp_f64() { assert_eq!(clamp_f64(0.5, 0.0, 1.0), 0.5); }
    #[test] fn test_lerp() { assert_eq!(lerp(0.0, 10.0, 0.5), 5.0); }
    #[test] fn test_sum_array() { assert_eq!(sum_array(vec![1,2,3,4,5]), 15); }
    #[test] fn test_product_array() { assert_eq!(product_array(vec![1,2,3,4]), 24); }
    #[test] fn test_http_status() { assert_eq!(http_status(200), "OK"); assert_eq!(http_status(999), "Unknown"); }
    #[test] fn test_find_index() { assert_eq!(find_index(vec![10,20,30], 20), 1); assert_eq!(find_index(vec![10,20,30], 99), -1); }
    #[test] fn test_all_positive_arr() { assert!(all_positive_arr(vec![1,2,3])); assert!(!all_positive_arr(vec![1,-2,3])); }
    #[test] fn test_double_if_positive() { assert_eq!(double_if_positive(5), Some(10)); assert_eq!(double_if_positive(-3), None); }
    #[test] fn test_chain_options() { assert_eq!(chain_options(Some(3), Some(4)), Some(7)); assert_eq!(chain_options(None, Some(4)), None); }
    #[test] fn test_is_power_of_two() { assert!(is_power_of_two(8)); assert!(!is_power_of_two(6)); }
    #[test] fn test_is_prime() { assert!(is_prime(2)); assert!(is_prime(17)); assert!(!is_prime(4)); assert!(!is_prime(1)); }
    #[test] fn test_make_empty_vec() { assert_eq!(make_empty_vec(), Vec::<i32>::new()); }
    #[test] fn test_option_unwrap_or() { assert_eq!(option_unwrap_or(Some(5)), 5); assert_eq!(option_unwrap_or(None), 0); }
    #[test] fn test_option_is_some() { assert!(option_is_some(Some(1))); assert!(!option_is_some(None)); }
    #[test] fn test_str_to_lower() { assert_eq!(str_to_lower("HELLO"), "hello"); }
    #[test] fn test_str_to_upper() { assert_eq!(str_to_upper("hello"), "HELLO"); }
    #[test] fn test_str_starts_with() { assert!(str_starts_with("hello world", "hello")); assert!(!str_starts_with("hello world", "world")); }
    #[test] fn test_str_trim() { assert_eq!(str_trim("  hello  "), "hello"); }
}
