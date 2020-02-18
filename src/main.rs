extern crate ulid;
fn main() {
    let ulid = ulid::Ulid::new();
    println!("{}", ulid);
}
