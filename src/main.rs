extern crate num;
extern crate image;
extern crate crossbeam;

use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;



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

#[allow(dead_code)]
fn pixel_to_point(bounds:(usize, usize),
            pixel: (usize, usize),
            upper_left: Complex<f64>,
            lower_right: Complex<f64>) -> Complex<f64> {
            let (width, height) = (lower_right.re - upper_left.re, 
                                upper_left.im - lower_right.im);

            Complex {
                re: upper_left.re +pixel.0 as f64 * width / bounds.0 as f64,
                im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
                //subtraction here because pixel.1 increases as we go down,
                // but the imaginary component increases as we go up.
            }
}


#[allow(dead_code)]
fn render(pixels: &mut [u8],
        bounds:(usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0 .. bounds.1 {
        for column in 0 .. bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
                            upper_left, lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 255) { 
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

#[allow(dead_code)]
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) 
    -> Result<(), std::io::Error> {
        let output = File::create(filename)?;

        let encoder = PNGEncoder::new(output);
        encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
        Ok(())
}

/// parse_pair test
#[test]
fn	test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,",	','), None);
    assert_eq!(parse_pair::<i32>(",10",	','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','),	Some((10,	20)));
    assert_eq!(parse_pair::<i32>("10,20xy",	','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'),	None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",	'x'), Some((0.5,	1.5)));
}

/// parse_complex test
#[test]
fn	test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{	re:	1.25,	im:	-0.0625	}));
    assert_eq!(parse_complex(",-0.0625"),	None);
}

/// pixel_to_point test
#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,100), (25, 75),
                    Complex { re: -1.0, im: 1.0 },
                    Complex { re: 1.0, im: -1.0 }),
                    Complex { re: -0.5, im: -0.5 });
}

/// This program takes a set of command line arguments and with those
/// renders an image representitive of fractals created by examining 
/// sections of the Mandlebrot set. The Mandlebrot set is the set of
/// values of 'c' in the complex plane for which the orbit of 0 under
/// the iteration of the quadratic map remains bounded. [wiki]. 
/// The images are created by iterating over P(c) : z -> z^2 + c
/// which at a critical point z = 0, either escapes to infinity or
/// stays within some finite radius 'r'.
/// Using Grayscale, it shades in each individual pixel
/// tracking z's position on the given image plane from a complex plane
/// conversion.
/// 
/// The work is split up among threads using crossbeam, and in turn they split up the rows of
/// the image to be rendered until it's completed. 
fn main() {
    let args: Vec<String> = std::env::args().collect();

    // if they have the incorrect (arguments / amount of arguments), tell them!
    if args.len() != 5 {
        writeln!(std::io::stderr(),
            "Usage: mandlebrot FILE PIXELS UPPERLEFT LOWERRIGHT")
            .unwrap();

        writeln!(std::io::stderr(),
            "Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1,0.20", 
            args[0])
            .unwrap();

        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing the image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing the upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing the lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    let threads = 8;
    let rows_per_thread = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut[u8]> =
            pixels.chunks_mut(rows_per_thread * bounds.0).collect();

        crossbeam::scope(|spawner| {
            for(i, band) in bands.into_iter().enumerate() {
                let top = rows_per_thread * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = 
                    pixel_to_point(bounds, (0, top), upper_left, lower_right);

                let band_lower_right = 
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }

    write_image(&args[1], &pixels, bounds)
        .expect("error writing the PNG file");

    writeln!(std::io::stdout(),
        "\n Mandlebrot Program Finished! Program exited successfully!\n Check your Parent Directory for the resulting image!\n\n")
        .unwrap();
    std::process::exit(0);
}
