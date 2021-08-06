pub fn area_of(x: i32, y: i32) -> i32 {
  // 2a. Fix this function to correctly compute the area of a rectangle given
  // dimensions x and y by multiplying x and y and returning the result.
  //
  let area = x * y;
  area
  // Challenge: The previous line is not idiomatic (not recommended best practice).
  //            Run `cargo clippy`, figure out what's wrong, and fix it.  Once it is fixed,
  //            `cargo clippy` won't return areas, and `cargo run` will still produce the same
  //            output. See also https://github.com/rust-lang/rust-clippy
}

pub fn volume(x: i32, y: i32, z: i32) -> i32 {
  let volume = x * y * z;
  volume
}
