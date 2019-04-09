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

## References
[Mandlebrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set) https://en.wikipedia.org/wiki/Mandelbrot_set