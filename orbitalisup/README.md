# `orbitalisup`

Update or revert to a specific Orbitalis branch with ease.

## Installing

```sh
curl -L https://orbitalis.paradigm.xyz | bash
```

## Usage

To install the **nightly** version:

```sh
orbitalisup
```

To install a specific **version** (in this case the `nightly` version):

```sh
orbitalisup --version nightly
```

To install a specific **branch** (in this case the `release/0.1.0` branch's latest commit):

```sh
orbitalisup --branch release/0.1.0
```

To install a **fork's main branch** (in this case `transmissions11/orbitalis`'s main branch):

```sh
orbitalisup --repo transmissions11/orbitalis
```

To install a **specific branch in a fork** (in this case the `patch-10` branch's latest commit in `transmissions11/orbitalis`):

```sh
orbitalisup --repo transmissions11/orbitalis --branch patch-10
```

To install from a **specific Pull Request**:

```sh
orbitalisup --pr 1071
```

To install from a **specific commit**:

```sh
orbitalisup -C 94bfdb2
```

To install a local directory or repository (e.g. one located at `~/git/orbitalis`, assuming you're in the home directory)

##### Note: --branch, --repo, and --version flags are ignored during local installations.

```sh
orbitalisup --path ./git/orbitalis
```

---

**Tip**: All flags have a single character shorthand equivalent! You can use `-v` instead of `--version`, etc.

---
