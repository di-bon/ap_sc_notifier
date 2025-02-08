# Simulation Controller Notifier
This repo contains the code for the Simulation Controller Notifier for the Advanced Programming course help at the University of Trento in the academic year 2024/2025.

Its usefulness relies on using a single component to handle all the communication to the Simulation Controller, given the fact that it can be embedded into ```Arc<T>```.

## Usage
To use the Simulation Controller Notifier, add
```toml
ap_sc_notifier = { git = "https://github.com/di-bon/ap_sc_notifier.git" }
```
to your Cargo.toml file.

Then, just import it in your project files using
```rust
use ap_sc_notifier::SimulationControllerNotifier;
```

## Panics
See the documentation for each function.
