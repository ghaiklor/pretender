# pretender

CLI to help you pretend your machine does upgrades, compilation, docker pulling or whatever.

## Why

It is just a fun project I thought about some time ago.
I was wondering if I could write a CLI that will pretend to do something with progress bars and stuff.
Afterwards, I thought, why not to combine the idea of drawing useless progress bars and the idea of pretending it does something.

That’s how pretender repository was created.

## Why Rust

Because I love Rust.
That’s the language I do not write often, but when there is a chance - I’ll take Rust.
In the long run, I want to switch to Rust, so any fun project I’m creating, I’m trying to create it with Rust language.

## Getting Started

Right now, it is a project that can be compiled on your machine and installed by yourself only.
For that, clone the repository, install the Rust toolchain and just call the cargo to build it:

```shell
cargo build
```

After that, you will get a debug build of the project in the target directory and you call the binary from there:

```shell
./target/debug/pretender brew
```

## List of pretenders

### Homebrew

The pretender for brew makes it look like the upgrade process is going on.

*NOTE: The demo is shot on 5 fps and 2x speed. In the real-time it is much slower and took 4 minutes.*

![Brew Demo][1]

## License

[MIT](./LICENSE)

[1]: https://user-images.githubusercontent.com/3625244/138553475-39c9cbff-789e-4e94-80f9-9c1762042bce.gif
