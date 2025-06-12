# Ray Tracing in One Weekend written in Rust
This is my implementation of the **Ray Tracing in One Weekend** book written primarily by Peter Shirley done in Rust.
The most notable differences aside from general Rust'isms is parallelising the ray tracing process via the `Grid` datatype
and using the [image](https://crates.io/crates/image) library to write the image instead of using `stdout`.

Next I will be implementing the next book in the series **Ray Tracing: The Next Week**.

# References
- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
