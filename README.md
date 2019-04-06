# gebers

Game Boy emulator written in Rust.


## Status

This is a personal project to learn more about Rust and how to emulate old systems.

For now, the emulator only implements the CPU and a very simplified version of
the memory. Sound, video, and support for the controls are not emulated yet.

The emulator passes 10 out of the 11 tests of [Blargg's CPU test ROMs](http://gbdev.gg8.se/files/roms/blargg-gb-tests/).
The one that tests interrupts fails because those have not been implemented yet.


## Test

Run the tests with:

```bash
cargo test
```


## Usage

Compile:
```bash
cargo build --release
```

Run:
```bash
cargo run
```


## Resources

- [BGB Pan Docs](http://bgb.bircd.org/pandocs.htm)
- [Tutorial to write a Game Boy emulator in Rust](https://github.com/rylev/DMG-01) by [@rylev](https://github.com/rylev/)
- [Game Boy emulation in Javascript](http://imrannazar.com/gameboy-Emulation-in-JavaScript)
- [Game Boy instruction set](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Blargg's test ROMs](http://gbdev.gg8.se/files/roms/blargg-gb-tests/)
- [awesome-gbdev repo](https://github.com/gbdev/awesome-gbdev)


## License

[MIT License](http://opensource.org/licenses/MIT)
