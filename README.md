# EnergyMon Rust Wrappers

The `energymon` crate provides some abstractions over the `energymon-sys`
crate, available at
[https://github.com/energymon/energymon-sys](https://github.com/energymon/energymon-sys).

## Dependencies

The `energymon` crate depends on the `energymon-sys` crate.

Additionally, you should have the native `energymon` libraries installed to the
system.

The latest `EnergyMon` C libraries can be found at
[https://github.com/energymon/energymon](https://github.com/energymon/energymon).

## Usage
Add `energymon` as a dependency in `Cargo.toml`:

```toml
[dependencies.energymon]
git = "https://github.com/energymon/energymon-rust.git"
```
