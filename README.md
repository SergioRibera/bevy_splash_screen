# Bevy Splash Screen

</br>
<p align="center">
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/bevy_splash_screen/ci.yml?label=ci&style=flat-square">
    <a href="https://crates.io/crates/bevy_splash_screen"><img alt="GitHub release (latest by date)" src="https://img.shields.io/crates/v/bevy_splash_screen"></a>
</p>

# Versions
Aviable and compatible versions

|  bevy  |   SplashScreen  |
|--------|-----------------|
| 0.10.1 |      0.1.0      |

# Features
- Suport multiple screens (Multiple brands on sequencial screens)
- Multiple brands (images/text)
- Animated color
- Background Color for each screen
- Manage workflow of splash scrreen with States

# Usage
Check out the [examples](./examples) for details.

Add to Cargo.toml
```toml
[dependencies]
bevy = "0.10.1"
bevy_splash_screen = "0.1.0"
```

# TODOs
Open for contributions =D

- [ ] Logic for static brands (persistent on all screen brands) :(
- [ ] Tween more customizable (transform, scale, etc)
