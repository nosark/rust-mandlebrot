extern crate num;
use num::Complex;
use std::str::FromStr;


/// Try to Determine if c is in the Mandelbrot set, using at most
/// limit iterations to determine if c is a member.
/// 
/// If 'c' is not a member of the set, return Some(i) where 'i' is 
/// the number of iterations it took for 'c' to leave the circle of
/// radius two centered on the origin. If 'c' seems to be a member 
/// (more precisely, if we reached the iteration limit without being
/// able to prove that 'c' is not a member). 
/// return None
#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

/// Parse a command-line string as a coordinate pair in multiple formats
/// example: `"400x600"` , `"1.0,1.5"`
/// 
/// The string should be in the following format <left><sep><right>
/// where <sep> is a seperator argument and left and right are both 
/// strings that can be parsed by `T::from_str`.
#[allow(dead_code)]
fn parse_pair<T:FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match(T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(i), Ok(r)) => Some((i, r)),
                _ => None
            }
        }
    }
}

/// Parse a pair of floating-point numbers seperated by a comma as a complex number
#[allow(dead_code)]
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im}),
        None => None
    }
}

/// parse_pair test
#[test]
fn	test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",								','),	None);
    assert_eq!(parse_pair::<i32>("10,",					','),	None);
    assert_eq!(parse_pair::<i32>(",10",					','),	None);
    assert_eq!(parse_pair::<i32>("10,20",			','),	Some((10,	20)));
    assert_eq!(parse_pair::<i32>("10,20xy",	','),	None);
    assert_eq!(parse_pair::<f64>("0.5x",				'x'),	None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",	'x'),	Some((0.5,	1.5)));
}

/// parse_complex test
#[test]
fn	test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{	re:	1.25,	im:	-0.0625	}));
    assert_eq!(parse_complex(",-0.0625"),	None);
}

fn main() {
    println!("Hello, world!");
}
