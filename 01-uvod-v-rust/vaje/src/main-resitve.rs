use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(mut a0: u32, mut a1: u32, n: u32) -> u32 {
    for _ in 0..n {
        (a0, a1) = (a1, a0 + a1)
    }

    return a0;
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno(year : u32) -> bool {
    return year % 400 == 0 || (year % 100 != 0 && year % 4 == 0)
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn je_veljaven_datum((day, month, year) : Date) -> bool {
    let valid_month : bool = 1 <= month && month <= 12;
    let mut valid_day : bool = match month {
        2 => (je_prestopno(year) && day <= 29) || (!je_prestopno(year) && day <= 28),
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        _ => day <= 30,
    };
    valid_day = valid_day && (1 <= day);

    return valid_month && valid_day;
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    let mut val : u32 = fun(start);
    while !cond(val) {
        start = val;
        val = fun(start);
    }

    return start;
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    let mut c : f64 = (a + b) / 2.0;
    while fun(c).abs() >= prec && (b - a) >= prec {
        let mid_val : f64 = fun(c);
        
        if mid_val > 0.0 {
            b = c;
        }
        else {
            a = c;
        }

        c = (a + b) / 2.0;
    }

    return c
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    panic!("Not implemented");
}

fn vsebuje<T : PartialEq>(v: &Vec<T>, x : &T) -> bool {
    for y in v {
      if x == y {
        return true
      }
    }
    return false
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A
/// Napišite funkcijo `fn selection_sort(mut arr: [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
        assert_eq!(55, fib(0, 1, 10));
    }

    #[test]
    fn test_prestopno() {
        assert_eq!(true, je_prestopno(2024));
        assert_eq!(true, je_prestopno(2000));
        assert_eq!(false, je_prestopno(1999));
        assert_eq!(false, je_prestopno(2025));
    }

    #[test]
    fn test_datum() {
        assert_eq!(true, je_veljaven_datum((19, 2, 2025)));
        assert_eq!(false, je_veljaven_datum((50, 1, 1)));
        assert_eq!(true, je_veljaven_datum((29, 2, 2024)));
        assert_eq!(false, je_veljaven_datum((0, 2, 2024)));
    }

    #[test]
    fn test_iteracija() {
        assert_eq!(16, iteracija(2, |x| x*x, |x| x >= 100));
    }

    #[test]
    fn test_bisekcija() {
        assert_eq!(0.125, bisekcija(0.0, 2.0, |x| x, 0.15));
    }
}
