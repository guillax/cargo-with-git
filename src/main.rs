fn main() {
  println!("CARGO_PKG_NAME: {:?}", env!("CARGO_PKG_NAME"));
  println!("CARGO_PKG_VERSION: {:?}", env!("CARGO_PKG_VERSION"));
  println!("CARGO_GIT_COMMIT: {:?}", env!("CARGO_GIT_COMMIT"));
  println!("CARGO_GIT_STATUS: {:?}", env!("CARGO_GIT_STATUS"));
}
