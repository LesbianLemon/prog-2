struct ArithmeticSequence {
    d: i64,
    a_0: i64,
    a_i: i64,
}

impl ArithmeticSequence {
    fn new(a0: i64, d: i64) -> ArithmeticSequence {
        ArithmeticSequence {
            d: d,
            a_0: a0,
            a_i: a0,
        }
    }

    fn next(&mut self) -> i64 {
        self.a_i += self.d;
        self.a_i - self.d
    }

    fn n_th(&self, n: u64) -> i64 {
        self.a_0 + (n as i64)*self.d
    }

    fn reset(&mut self) {
        self.a_i = self.a_0;
    }

    fn current(&self) -> i64 {
        self.a_i
    }

    fn sum(&self, n: u64) -> i64 {
        let mut res: i64 = 0;

        for i in 0..n {
            res += self.n_th(i);
        }

        res
    }

    fn seq_sum(&self, other: ArithmeticSequence) -> ArithmeticSequence {
        ArithmeticSequence::new(
            self.a_0 + other.a_0,
            self.d + other.d
        )
    }
}

struct GeometricSequence {
    k: i64,
    a_0: i64,
    a_i: i64,
}

impl GeometricSequence {
    fn new(a0: i64, k: i64) -> GeometricSequence {
        GeometricSequence {
            k: k,
            a_0: a0,
            a_i: a0,
        }
    }

    fn next(&mut self) -> i64 {
        self.a_i *= self.k;
        self.a_i / self.k
    }

    fn n_th(&self, n: u64) -> i64 {
        let mut curr: i64 = self.a_0;

        for i in 0..n {
            curr *= self.k;
        }

        curr
    }

    fn reset(&mut self) {
        self.a_i = self.a_0;
    }

    fn current(&self) -> i64 {
        self.a_i
    }

    fn sum(&self, n: u64) -> i64 {
        let mut res: i64 = 0;

        for i in 0..n {
            res += self.n_th(i);
        }

        res
    }

    fn seq_prod(&self, other: GeometricSequence) -> GeometricSequence {
        GeometricSequence::new(
            self.a_0*other.a_0,
            self.k*other.k
        )
    }
}

enum BinOperation {
    Plus,
    Minus,
    Times,
}

enum Expr {
    Const(u32),
    Operation(Box<Expr>, BinOperation, Box<Expr>),
}

// Operation(Box<Const(1)>, Plus, Operation(Box<2>, Times, Box<3>))

impl Expr {
    fn eval(&self) -> u32 {
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
