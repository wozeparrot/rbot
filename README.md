# rbot (A Rust FRC Library)

**This Monorepo is currently split into 3 parts: [rbothal](#rbothal), [rbotlib](#rbotlib), and [cargo-rbot](#cargo-rbot).**

---

## rbothal

The Hardware Abstraction Layer bindings for [rbotlib](#rbotlib). DO NOT USE THIS DIRECTLY, instead use [rbotlib](#rbotlib).

## rbotlib

The actual rust library for programming FRC robots in rust. Currently in a highly experimental state, use at your own risk.

## cargo-rbot

Install with `cargo install cargo-rbot`. Used to create and deploy rbot projects.

## Examples

Located in [`rbot-examples`](rbot-examples/).

# Credits

Lots of credit goes to [first-rust-competition](https://github.com/Lytigas/first-rust-competition) as that inspired me to create this in the first place. More credit goes to [rust-wpilib](https://github.com/robotrs/rust-wpilib) as that helped me out a lot on creating the wpilib port.