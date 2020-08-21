# rust-terrain-gen
A Rust project amenable to newcomers to the language. A working demo using web assembly (i.e. Rust running natively in your browser) is available at [thebutlah.github.io/rust-terrain-gen](https://thebutlah.github.io/rust-terrain-gen)

## Why Terrain Generation as a demo/beginner project?
Terrain generation has a couple of important properties:
1. Its interesting, and gives a cool result. New projects in a new language are too often about printing to the command line or other not-so-exciting use cases. This project has something visual that you can actually interact with.
2. Terrain generation requires a teensy tiny bit of math - not enough to overwhelm anyone, but enough to get introduced to [ndarray](https://docs.rs/ndarray/0.13.1/ndarray/), one of Rusts popular numerical libraries (think NumPy from Python).
3. Terrain generation is easily parallelized. This gives a perfect opportunity to introduce iterators, [rayon](https://github.com/rayon-rs/rayon), and how easy it is to make code parallelized in Rust.
4. We can get a taste of enums and pattern matching, as we will be using enums to describe the type of the terrain.
5. Because we are just making fancy images, we can start out by saving the results to a file, and graduate to WebAssembly and displaying the results in your web browser. This is super cool from a dev-ops standpoint, and also means that we get to show others our work even on a phone.

Example of generated terrain:
![An example of the generated terrain](docs/example_terrain.png)
