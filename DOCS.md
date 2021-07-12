# Redstone proof of concept

## How to compile

### Build optimization

The following instructions use the ```--release``` flag. This means that cargo will optimize the code while compiling. It takes considerably longer to compile but makes the executables *much* faster. If you want to compile faster or if you are debugging the code, remove the ```release``` tag and the bins will end up in ```target/debug``` rather than ```target/release```. Please note that using these debug bins will result in a considerably lower vote and hence lower reward. On slower machines, using debug may cause you to receive vote below the minimum (meaning you get banned for an epoch). For this reason, we do not recommend that you remove the ```---release``` tag unless needed.
### Linux

#### Prerequisites
Rust makes abundant use of Rust's syntax extensions and other advanced, unstable features. Because of this, you will need to use a nightly version of Rust. If you already have a working installation of the latest Rust nightly, feel free to skip to the next section.

To install a nightly version of Rust, we recommend using rustup. Install rustup by following the instructions on its website. Once rustup is installed, configure Rust nightly as your default toolchain by running the command:
```
rustup default nightly
```

```
rustup update && cargo update
```
You will also need the following packages: Cargo (or rustc) and git.

##### Ubuntu

```bash

git clone -b master --single-branch https://github.com/RedStoneCoin/redstone-poc
cd redstone poc
cargo build --release
```

After the completion, the binaries will be in the `target/release` folder.

```bash
cd target
./Redstone poc
```
Proof of concept will genearte blockchain files.

##### Generic Linux

Ensure you have the dependencies listed above.


```bash
git clone -b master --single-branch https://github.com/RedStoneCoin/redstone-poc
cd redstone-poc
cargo build --release
```
After the completion, the binaries will be in the `target/release` folder.

```bash
cd target
./Redstone poc
```


## Contributing
Pull requests are welcomed. If you can help with the code, please fork the repo, make your changes to the forked repo and then open a PR into the development branch. Please <b>NEVER</b> open a PR into the master branch. Any PRs into the master branch without prior authorization will be closed.

## Contributors
A huge thank you to everyone who has controbuted to the redstone project:
- [Leo Cornelius (Developer) ](https://github.com/LeoCornelius)

