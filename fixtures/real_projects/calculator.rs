/// A simple expression tree calculator.

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Var(String),
}

#[derive(Debug, Clone)]
pub struct Env {
    bindings: Vec<(String, f64)>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            bindings: Vec::new(),
        }
    }

    pub fn set(&mut self, name: String, value: f64) {
        for binding in &mut self.bindings {
            if binding.0 == name {
                binding.1 = value;
                return;
            }
        }
        self.bindings.push((name, value));
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        for binding in &self.bindings {
            if binding.0 == name {
                return Some(binding.1);
            }
        }
        None
    }
}

pub fn eval(expr: &Expr, env: &Env) -> Result<f64, String> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Add(a, b) => {
            let va = eval(a, env)?;
            let vb = eval(b, env)?;
            Ok(va + vb)
        }
        Expr::Sub(a, b) => {
            let va = eval(a, env)?;
            let vb = eval(b, env)?;
            Ok(va - vb)
        }
        Expr::Mul(a, b) => {
            let va = eval(a, env)?;
            let vb = eval(b, env)?;
            Ok(va * vb)
        }
        Expr::Div(a, b) => {
            let va = eval(a, env)?;
            let vb = eval(b, env)?;
            if vb == 0.0 {
                Err("division by zero".to_string())
            } else {
                Ok(va / vb)
            }
        }
        Expr::Neg(e) => {
            let v = eval(e, env)?;
            Ok(-v)
        }
        Expr::Var(name) => {
            if let Some(v) = env.get(name) {
                Ok(v)
            } else {
                Err(format!("undefined variable: {}", name))
            }
        }
    }
}

/// Simplify constant expressions.
pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::Add(a, b) => {
            let a = simplify(a);
            let b = simplify(b);
            match (&a, &b) {
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x + y),
                (_, Expr::Num(n)) if *n == 0.0 => a,
                (Expr::Num(n), _) if *n == 0.0 => b,
                _ => Expr::Add(Box::new(a), Box::new(b)),
            }
        }
        Expr::Mul(a, b) => {
            let a = simplify(a);
            let b = simplify(b);
            match (&a, &b) {
                (Expr::Num(x), Expr::Num(y)) => Expr::Num(x * y),
                (_, Expr::Num(n)) if *n == 1.0 => a,
                (Expr::Num(n), _) if *n == 1.0 => b,
                (_, Expr::Num(n)) if *n == 0.0 => Expr::Num(0.0),
                (Expr::Num(n), _) if *n == 0.0 => Expr::Num(0.0),
                _ => Expr::Mul(Box::new(a), Box::new(b)),
            }
        }
        Expr::Neg(e) => {
            let e = simplify(e);
            if let Expr::Num(n) = e {
                Expr::Num(-n)
            } else {
                Expr::Neg(Box::new(e))
            }
        }
        _ => expr.clone(),
    }
}

/// Count the number of nodes in an expression tree.
pub fn count_nodes(expr: &Expr) -> usize {
    match expr {
        Expr::Num(_) | Expr::Var(_) => 1,
        Expr::Neg(e) => 1 + count_nodes(e),
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
            1 + count_nodes(a) + count_nodes(b)
        }
    }
}

/// Collect all variable names in an expression.
pub fn collect_vars(expr: &Expr) -> Vec<String> {
    let mut vars = Vec::new();
    collect_vars_inner(expr, &mut vars);
    vars
}

fn collect_vars_inner(expr: &Expr, vars: &mut Vec<String>) {
    match expr {
        Expr::Var(name) => {
            if !vars.contains(name) {
                vars.push(name.clone());
            }
        }
        Expr::Neg(e) => collect_vars_inner(e, vars),
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
            collect_vars_inner(a, vars);
            collect_vars_inner(b, vars);
        }
        Expr::Num(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(n: f64) -> Expr {
        Expr::Num(n)
    }

    fn add(a: Expr, b: Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }

    fn mul(a: Expr, b: Expr) -> Expr {
        Expr::Mul(Box::new(a), Box::new(b))
    }

    fn var(name: &str) -> Expr {
        Expr::Var(name.to_string())
    }

    #[test]
    fn test_eval_basic() {
        let env = Env::new();
        let expr = add(num(2.0), num(3.0));
        assert_eq!(eval(&expr, &env), Ok(5.0));
    }

    #[test]
    fn test_eval_complex() {
        let env = Env::new();
        // (2 + 3) * 4 = 20
        let expr = mul(add(num(2.0), num(3.0)), num(4.0));
        assert_eq!(eval(&expr, &env), Ok(20.0));
    }

    #[test]
    fn test_eval_with_var() {
        let mut env = Env::new();
        env.set("x".to_string(), 10.0);
        let expr = add(var("x"), num(5.0));
        assert_eq!(eval(&expr, &env), Ok(15.0));
    }

    #[test]
    fn test_eval_undefined_var() {
        let env = Env::new();
        let expr = var("y");
        assert!(eval(&expr, &env).is_err());
    }

    #[test]
    fn test_simplify() {
        // 0 + x → x
        let expr = add(num(0.0), var("x"));
        assert_eq!(simplify(&expr), var("x"));

        // 2 + 3 → 5
        let expr = add(num(2.0), num(3.0));
        assert_eq!(simplify(&expr), num(5.0));

        // x * 1 → x
        let expr = mul(var("x"), num(1.0));
        assert_eq!(simplify(&expr), var("x"));
    }

    #[test]
    fn test_count_nodes() {
        let expr = add(num(1.0), mul(num(2.0), num(3.0)));
        assert_eq!(count_nodes(&expr), 5);
    }

    #[test]
    fn test_collect_vars() {
        let expr = add(var("x"), mul(var("y"), var("x")));
        let vars = collect_vars(&expr);
        assert_eq!(vars, vec!["x", "y"]);
    }
}
