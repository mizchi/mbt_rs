derive(Show)
pub enum Expr {
  Num(Double)
  Add(Expr, Expr)
  Sub(Expr, Expr)
  Mul(Expr, Expr)
  Div(Expr, Expr)
  Neg(Expr)
  Var(String)
}
derive(Show)
pub struct Env {
  bindings : Array[(String, Double)]
}
pub fn Env::new() -> Env {
  { bindings: Array::new() }
}
pub fn Env::set(self, name : String, value : Double) {
  for binding in self.bindings {
    if binding.0 == name {
      binding.1 = value
      return
    }
  }
  self.bindings.push((name, value))
}
pub fn Env::get(self, name : String) -> Option[Double] {
  for binding in self.bindings {
    if binding.0 == name {
      return Some(binding.1)
    }
  }
  None
}

pub fn eval(expr : Expr, env : Env) -> Result[Double, String] {
  match expr {
    Expr::Num(n) => Ok(n)
    Expr::Add(a, b) => {
      let va = eval(a, env)!
      let vb = eval(b, env)!
      Ok(va + vb)
    }
    Expr::Sub(a, b) => {
      let va = eval(a, env)!
      let vb = eval(b, env)!
      Ok(va - vb)
    }
    Expr::Mul(a, b) => {
      let va = eval(a, env)!
      let vb = eval(b, env)!
      Ok(va * vb)
    }
    Expr::Div(a, b) => {
      let va = eval(a, env)!
      let vb = eval(b, env)!
      if vb == 0.0 {
        Err("division by zero".to_string())
      } else {
        Ok(va / vb)
      }
    }
    Expr::Neg(e) => {
      let v = eval(e, env)!
      Ok(-v)
    }
    Expr::Var(name) =>     if env.get(name) is Some(v) {
      Ok(v)
    } else {
      Err(""undefined variable: {}" , name" // TODO: convert to string interpolation)
    }
  }
}
pub fn simplify(expr : Expr) -> Expr {
  match expr {
    Expr::Add(a, b) => {
      let a = simplify(a)
      let b = simplify(b)
      match (a, b) {
        (Expr::Num(x), Expr::Num(y)) => Expr::Num(x + y)
        (_, Expr::Num(n)) if n == 0.0 => a
        (Expr::Num(n), _) if n == 0.0 => b
        _ => Expr::Add(a, b)
      }
    }
    Expr::Mul(a, b) => {
      let a = simplify(a)
      let b = simplify(b)
      match (a, b) {
        (Expr::Num(x), Expr::Num(y)) => Expr::Num(x * y)
        (_, Expr::Num(n)) if n == 1.0 => a
        (Expr::Num(n), _) if n == 1.0 => b
        (_, Expr::Num(n)) if n == 0.0 => Expr::Num(0.0)
        (Expr::Num(n), _) if n == 0.0 => Expr::Num(0.0)
        _ => Expr::Mul(a, b)
      }
    }
    Expr::Neg(e) => {
      let e = simplify(e)
      if e is Expr::Num(n) {
        Expr::Num(-n)
      } else {
        Expr::Neg(e)
      }
    }
    _ => expr.copy()
  }
}
pub fn count_nodes(expr : Expr) -> Int {
  match expr {
    Expr::Num(_) | Expr::Var(_) => 1
    Expr::Neg(e) => 1 + count_nodes(e)
    Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) =>     1 + count_nodes(a) + count_nodes(b)
  }
}
pub fn collect_vars(expr : Expr) -> Array[String] {
  let mut vars = Array::new()
  collect_vars_inner(expr, vars)
  vars
}
fn collect_vars_inner(expr : Expr, vars : Array[String]) {
  match expr {
    Expr::Var(name) =>     if not(vars.contains(name)) {
      vars.push(name.copy())
    }
    Expr::Neg(e) => collect_vars_inner(e, vars)
    Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
      collect_vars_inner(a, vars)
      collect_vars_inner(b, vars)
    }
    Expr::Num(_) => 
  }
}

fn num(n : Double) -> Expr {
  Expr::Num(n)
}
fn add(a : Expr, b : Expr) -> Expr {
  Expr::Add(a, b)
}
fn mul(a : Expr, b : Expr) -> Expr {
  Expr::Mul(a, b)
}
fn var(name : String) -> Expr {
  Expr::Var(name.to_string())
}
test "eval basic" {
  let env = Env::new()
  let expr = add(num(2.0), num(3.0))
  assert_eq(eval(expr, env), Ok(5.0))
}
test "eval complex" {
  let env = Env::new()
  let expr = mul(add(num(2.0), num(3.0)), num(4.0))
  assert_eq(eval(expr, env), Ok(20.0))
}
test "eval with var" {
  let mut env = Env::new()
  env.set("x".to_string(), 10.0)
  let expr = add(var("x"), num(5.0))
  assert_eq(eval(expr, env), Ok(15.0))
}
test "eval undefined var" {
  let env = Env::new()
  let expr = var("y")
  assert_true(eval(expr, env).is_err())
}
test "simplify" {
  let expr = add(num(0.0), var("x"))
  assert_eq(simplify(expr), var("x"))
  let expr = add(num(2.0), num(3.0))
  assert_eq(simplify(expr), num(5.0))
  let expr = mul(var("x"), num(1.0))
  assert_eq(simplify(expr), var("x"))
}
test "count nodes" {
  let expr = add(num(1.0), mul(num(2.0), num(3.0)))
  assert_eq(count_nodes(expr), 5)
}
test "collect vars" {
  let expr = add(var("x"), mul(var("y"), var("x")))
  let vars = collect_vars(expr)
  assert_eq(vars, ["x", "y"])
}

