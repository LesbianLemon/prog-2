use std::{fmt::Display, ops::{Add, Mul, Sub}};

struct ArithmeticSequence<T>
where
    T: Add<Output = T>
{
    d: T,
    a_0: T,
    a_i: T,
}

impl<T> ArithmeticSequence<T>
where
    T: Add<Output = T> + Copy
{
    fn new(a_0: T, d: T) -> ArithmeticSequence<T> {
        ArithmeticSequence {
            d,
            a_0,
            a_i: a_0
        }
    }

    fn next(&mut self) -> T {
        let tmp: T = self.a_i;
        self.a_i = self.a_i + self.d;

        tmp
    }

    fn n_th(&self, n: u64) -> T {
        let mut res: T = self.a_0;
        for i in 0..n {
            res = res + self.d;
        }

        res
    }

    fn reset(&mut self) {
        self.a_i = self.a_0;
    }

    fn current(&self) -> T {
        self.a_i
    }

    fn sum(&self, n: u64) -> T {
        let mut res: T = self.a_0;
        for i in 0..n {
            res = res + self.n_th(i);
        }

        res
    }

    fn seq_sum(&self, other: ArithmeticSequence<T>) -> ArithmeticSequence<T> {
        ArithmeticSequence::new(
            self.a_0 + other.a_0,
            self.d + other.d
        )
    }
}

impl<T> ArithmeticSequence<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy
{
    fn seq_mul(&self, other: ArithmeticSequence<T>) -> ArithmeticSequence<T> {
        ArithmeticSequence::new(
            self.a_0*other.a_0,
            self.d*other.d
        )
    }
}

impl<T> PartialEq for ArithmeticSequence<T>
where
    T: Add<Output = T> + Copy + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.a_0 == other.a_0 && self.d == other.d
    }
}


trait Sequence<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;
    fn k_th(&self, k: u64) -> T;
    fn contains(&self, a: T) -> bool;
}

impl<T> Sequence<T> for ArithmeticSequence<T>
where
    T: Add<Output = T> + Copy + PartialEq + Display
{
    fn name(&self) -> String {
        format!("ArithmeticSequence<a_0: {}, d: {}>", self.a_0, self.d)
    }

    fn start(&self) -> T {
        self.a_0
    }

    fn k_th(&self, k: u64) -> T {
        self.n_th(k)
    }

    fn contains(&self, a: T) -> bool {
        

        while 
    }
}

struct ConstantSequence<T> {
    a_0: T
}

impl<T> Sequence<T> for ConstantSequence<T>
where
    T: Copy + PartialEq + Display
{
    fn name(&self) -> String {
        format!("ConstantSequence<a_0: {}>", self.a_0)
    }

    fn start(&self) -> T {
        self.a_0
    }

    fn k_th(&self, k: u64) -> T {
        self.a_0
    }

    fn contains(&self, a: T) -> bool {
        self.a_0 == a
    }
}


enum BinOperation {
    Plus,
    Minus,
    Times,
}

enum Expr<T> {
    Const(T),
    Operation(Box<Expr<T>>, BinOperation, Box<Expr<T>>),
}

// Operation(Box<Const(1)>, Plus, Operation(Box<2>, Times, Box<3>))

impl<T> Expr<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy
{
    fn eval(&self) -> T {
        match self {
            Expr::Const(x) => *x,
            Expr::Operation(expr1, op, expr2) =>
                match op {
                    BinOperation::Plus => (&**expr1).eval() + (&**expr2).eval(),
                    BinOperation::Minus => (&**expr1).eval() - (&**expr2).eval(),
                    BinOperation::Times => (&**expr1).eval() * (&**expr2).eval(),
                },
        }
    }
    
    fn collect(&self) -> u32 {
        match self {
            Expr::Const(x) => 1,
            Expr::Operation(expr1, op, expr2) => (&**expr1).collect() + (&**expr2).collect(),
        }
    }
} 

fn main() {
    println!("Hello, world!");
}
