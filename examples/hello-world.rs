use arena_rs::Arena;

fn main() {
    let mut arena = Arena::new(1024);
    let hello = arena.alloc("Hello").unwrap();
    let world = arena.alloc("World").unwrap();
    println!("{}, {}!", *hello, *world);
}
