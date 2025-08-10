# CS:GO Inv Patcher

Have you ever wanted to use your inventory and skins in CS:GO Legacy because CS2 just sucks? No problem! With the CS:GO Inv Patcher you can!

Just install the CS:GO Legacy version, [run the patcher](#usage), select `CS:GO` and wait. Now you can use your inventory and skins in CS:GO and play with your friends.

> If you encounter any issues or bugs, please report them by creating an [issue](https://github.com/Toaaa/csgo-inv-patcher/issues/new). Include as much detail as possible.

## Usage

> ⚠️ Make sure that the game is closed before running the patcher.

1. Run a [compiled binary](#installation).

2. Follow the Prompt: When the patcher runs, it will ask you which version you want to patch. In our case, we want to patch the CS:GO version, so we select CS:GO.

```
? Which version would you like to patch to? ›
❯ Patch to CS:GO
  Patch back to CS2
```

After choosing the version, the patcher will ask how you'd like to locate your CS(GO) installation:

```
✔ Which version would you like to patch to? · Patch to CS:GO
? Search for CS:GO automatically or enter path manually? ›
❯ Auto-scan drives
❯  Enter a custom path
```

If you choose Auto-scan drives, the patcher will scan all available drives on your PC to find the CS(GO) installation and apply the patch.

```
✔ Which version would you like to patch to? · Patch to CS:GO
Scanning all possible drives on your PC...
Scanning: C:\..
Scanning: D:\..
Scanning: E:\..
```

If you choose Enter a custom path, you'll be prompted to manually enter the CS:GO install folder (the folder that contains the `csgo` subfolder, by default this should be `C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive`):

```
✔ Which version would you like to patch to? · Patch to CS:GO
✔ Search for CS:GO automatically or enter path manually? · Enter a custom path
? Enter the path to your CS:GO folder ›
```

If the patch has been successfully applied, you will see a confirmation message. If everything went well, you can now start CS:GO Legacy and use your inventory and skins.

```
File successfully patched to version: CS:GO.
You should now be able to use your skins in CS:GO Legacy. You shouldn't play CS2 while this is active.
```

> [!IMPORTANT]\
> It's important to mention: While the CS:GO patch is applied, YOU SHOULD NOT PLAY CS2!

If you want to play CS2 again (for whatever reason), you can simply start the patcher again and select `CS2` this time.

## Installation

Currently, there is a [precompiled version](https://github.com/Toaaa/csgo-inv-patcher/releases/latest) for **Windows only**.


---


## Building from source
### Requirements

- A working installation of the Rust toolchain (typically installed via [rustup](https://rustup.rs/)).
- The CS:GO Legacy version [installed on your computer](https://bo3.gg/news/how-to-download-csgo-in-steam-after-cs2-release).

If you want to build it from source, clone this repository and navigate to its directory:

```bash
git clone https://github.com/toaaa/csgo-inv-patcher.git
cd csgo-inv-patcher
```

Next, build the application using Cargo:

```bash
cargo build --release
```

Once the build process is complete, you can find the executable file in the `target/release` directory.
Alternatively, you can run it using:

```sh
cargo run --release
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your improvements or bug fixes.

***Note**: This tool modifies game files and should be used responsibly. Ensure that you have backups of any files before making modifications in case something goes wrong.*
