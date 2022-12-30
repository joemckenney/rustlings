// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.

fn main() {
    struct Stuff {
        name: String,
        number: i32,
    }

    let c = {
        Stuff {
            name: "Joe".to_string(),
            number: 2,
        }
    };

    println!("Hello {}, {}!", c.name, c.number);
}
