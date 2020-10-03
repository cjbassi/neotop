# neotop

![Minimum rustc version](https://img.shields.io/badge/rustc-1.39+-green.svg)
[![crates.io](https://img.shields.io/crates/v/neotop.svg)](https://crates.io/crates/neotop)

A Rust port of [htop] with some additional improvements.

Improvements include:

- full vim keybindings
- bottom keybind bar has been removed
- fix a bug where the screen momentarily flashes blank on redraw
- all configuration is now done with CLI flags instead of with in-app menus
- all CLI flags can be persisted by specifying them in the config file
- add a panel to the help menu explaining what each UI element is displaying

Supported platforms:

- Linux

Windows support is not planned since it is not POSIX compliant and certain system information is not available.

<div align="center">
  <img src="./screenshot.png" />
</div>

## Installation

### Package managers

[![Packaging status](https://repology.org/badge/vertical-allrepos/neotop.svg)](https://repology.org/project/neotop/versions)

### Prebuilt binaries

Prebuilt binaries are provided in the [releases](https://github.com/cjbassi/neotop/releases) tab.

### From source

```bash
cargo install neotop
```

## Related projects

- Rust
  - [bb](https://github.com/epilys/bb)
  - [bottom](https://github.com/ClementTsang/bottom)
  - [zenith](https://github.com/bvaisvil/zenith)
- [bpytop](https://github.com/aristocratos/bpytop)
- [glances](https://github.com/nicolargo/glances)
- [gotop](https://github.com/xxxserxxx/gotop)
- [gtop](https://github.com/aksakalli/gtop)
- [htop]
- [htop-vim](https://github.com/KoffeinFlummi/htop-vim)
- [vtop](https://github.com/MrRio/vtop)

[htop]: https://github.com/htop-dev/htop
