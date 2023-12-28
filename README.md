## Installation
Before installing please make sure you have the latest rustc installed along with cargo, and you are running the latest stable release.

If you are not sure, you can just run:
```bash
rustup default stable
```

For now the best way to install foundry is:
1. Clone the repo
```bash
git clone https://github.com/bchainhub/foundry.git
```

2. Go to `foundry/orbitalisup/
```bash
cd foundry/orbitalisup/
```

3. Run orbitalisup
```bash
./orbitalisup --path ../
```

This will compile the entire project and will install all your binaries to ~/.orbitalis.
Now just restart your terminal and you can use one of the binaries. For reference here are the changed names:

forge: spark
anvil: shuttle
chisel: pilot
cast: probe

Later we will release binaries to github so the script will be able to just download them, but for now this is the best way.

## Acknowledgements
