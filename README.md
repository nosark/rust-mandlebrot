# Rust Mandlebrot Fractal Renderer

This program takes a set of command line arguments and with those
renders an image representitive of fractals created by examining 
sections of the Mandlebrot set. The Mandlebrot set is the set of
values of `c` in the complex plane for which the orbit of `0` under
the iteration of the quadratic map remains bounded. [wiki]. 
The images are created by iterating over
> Pc : z -> z^2 + c

stays within some finite radius `r`.
Using Grayscale, it shades in each individual pixel
tracking z's position on the given image plane from a complex plane
conversion. The work is split up among threads using crossbeam, and 
in turn they split up the rows of the image to be rendered until it's
completed.


## Getting Started

1. Clone this repository 
2. Run `cargo build --release`
3. Now run `./target/release/mandlebrot.exe FILENAME.png IMAGE_DIMENSIONS UPPERLEFT LOWERRIGHT`
   
   Example: `./target/release/mandlebrot.exe mandelbrot.png 1000x750 -1.20,0.35 -1,0.20`
   
   ***NOTE***: If running with powershell follow the below example and put Upperleft and LowerRight in quotes like so:
   
   Example: `./target/release/mandlebrot.exe mandelbrot.png 1000x750 "-1.20,0.35" "-1,0.20"`
   
4. Check out the rendered image in your Parent Directory!
5.  ENJOY!
## References
[Mandlebrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set) https://en.wikipedia.org/wiki/Mandelbrot_set