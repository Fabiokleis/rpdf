# rpdf
Rust cross-platform cli/gui to convert images to pdf.

## Workspaces
The project was divided in 3 workspace members,
convert it's a lib, cli and gui both are binaries
```toml
[workspace]
members = [
    "convert",
    "cli",
    "gui",
]
```

## Build
To build all workspace members
```shell
cargo build 
```
or to run individually
```shell
cargo build convert|cli|gui
```

## Run
To run on terminal interface
```shell
cargo run -p cli -- -c image_path.jgp -- out_put.pdf
```

To run on graphical interface
```shell
cargo run -p gui
```
## Dependencies

```toml
[dependencies]
printpdf = { version = "0.5.3", features = ["embedded_images"] }
clap = "4.0.32"
fltk = "1.3.25"
```

To work with pdfs [printpdf](https://github.com/fschutt/printpdf)

To create a cli app [clap](https://github.com/clap-rs/clap)

To create a cross-platform gui [fltk](https://github.com/fltk-rs/fltk-rs)

## GUI Version 0.1 screenshot on Linux
![image](https://user-images.githubusercontent.com/66813406/217687414-552ee1bc-6b6b-4bea-9438-712023f01686.png)


## GUI Version 0.1 - Next features
- Custom pdf size, A1, A2, A3, A4 ... etc.
- Drag and drop images in preview section
- Status bar at the bottom
- Support merge with pdfs
- GUI Translations
- Best appearence

## Contributing

Thank you for wanting to contribute to the Rpdf project!

Starts opening a issue and let's your suggestions, create pull a request.
Please consider to follow a clear way to write issue/pr.

All contributions are welcome!

## License

MIT
