# EnergyMon Rust Wrappers

The `energymon-rust` crate provides some abstractions over the `energymon-sys`
crate, available at
[https://github.com/connorimes/energymon-sys]().

## Dependencies

The `energymon-rust` crate depends on the `energymon-sys` crate.

Additionally, you must have the `energymon` libraries installed to the system.

The latest `EnergyMon` C libraries can be found at
[https://github.com/connorimes/energymon]().

## Usage
Add `energymon-rust` as a dependency in `Cargo.toml`:

```toml
[dependencies.energymon-rust]
git = "https://github.com/connorimes/energymon-rust.git"
```
