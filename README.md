# imagecheck

A checker that compares two images and based on an EPS value that checks the difference between the pixels, returns if the images matched correctly of if not, throws an error regarding where they differ.

Uses a json file to get the paths for the images

## Run the program

### Prerequisite

`rust` or `rustup`

```bash
cargo run -- tests.json <xx>-image_editor
```

## Build the final executable

```bash
cargo build --release
```
