# Lift

`Ls + File Type`

# Warning

Don't use these tools in prod: I'm learning Rust!

## Requirements

```sh
sudo apt install libmagic1 libmagic-dev

# optional (but needed to use `unbuffer lift | less -R`)
# since unbuffer comes with expect
sudo apt install expect
```

## Build

```sh
cargo build --release --bin lift
```

## TODO

- improve quality
- add permissions of file
- colors: red if error, green if executable file, blue for folders, purple for size...
- add separator "\t" as a field of FileDetails and consider learn about lifetime
- add option --no-color
- add option --depth?
- try multithread + queue?
- Windows vs Linux -> still OK?

