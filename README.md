# casgen

Generates randomized CAS12 style 6-plex CRISPR reads.

## Construct

A construct is defined by the following structure:

```text
{stagger}{left_adapter}
  [{constant}{spacer}] *6
{right_adapter}{stagger}
```

Where the `stagger` is a random sequence of 0-8 basepairs,
the `{left,right}_adapter` is not guaranteed to be the
same length.
The `constant` region will be the same length and the `spacer`
region is the same length as well.

## Usage

### Installation

```bash
git clone https://github.com/noamteyssier/casgen
cd casgen
cargo install --path .
```

### Running

```bash
casgen
```
