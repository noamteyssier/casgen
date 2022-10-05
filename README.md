# casgen

Generates randomized CAS12 style 6-plex CRISPR reads.

## Construct

A construct is defined by the following structure:

```text
{stagger}{left_constant}
  [{spacer}{variable}] *6
{right_constant}{stagger}
```

Where the `stagger` is a random sequence of 0-8 basepairs,
the `{left,right}_constant` is not guaranteed to be the
same length.
The `spacer` region will be the same length and the `variable`
region is the same length as well.

## Results

The following sequences will
