# Bevy Splash Screen
![BevySplashScreen_Presentation](https://user-images.githubusercontent.com/56278796/235970678-5ec68136-4624-419d-b1e1-f7d9f311bdae.gif)

</br>
<p align="center">
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/bevy_splash_screen/ci.yml?label=ci&style=flat-square">
    <a href="https://crates.io/crates/bevy_splash_screen"><img alt="GitHub release (latest by date)" src="https://img.shields.io/crates/v/bevy_splash_screen"></a>
</p>

# Versions
Aviable and compatible versions

|  bevy  |   SplashScreen  |
|--------|-----------------|
|  0.11  |      0.5.0      |
|  0.11  |      0.4.4      |
| 0.10.1 |      0.3.0      |

# Features
- Suport multiple screens (Multiple brands on sequencial screens)
- Multiple brands (images/text)
- Animated color
- Custom Skipable Method (Using Event)
- Background Color for each screen
- Manage workflow of splash scrreen with States

# Usage
Check out the [examples](./examples) for details.

Add to Cargo.toml
```toml
[dependencies]
bevy = "0.12"
bevy_splash_screen = "0.5.0"
```

> **WARN:** You probably need to add this if you also use `bevy_tweening`
> ```
> [patch.crates-io]
> bevy_tweening = { git = "https://github.com/sibsibsib/bevy_tweening", branch = "mirrored_repeat_fix" }
> ```

# TODOs
Open for contributions =D

- [ ] Logic for static brands (persistent on all screen brands) :(
- [ ] Tween more customizable (transform, scale, etc)
