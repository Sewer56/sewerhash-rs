---
hide:
  - toc
---

<div align="center">
	<h1>SewerHash</h1>
	<img src="Reloaded/Images/Reloaded-Icon.png" width="150" align="center" />
	<br/> <br/>
    A Custom SIMD FNV-1A derivative used in Reloaded3 & Friends
</div>

## About

SewerHash is a custom, high speed, unstable hashing algorithm.

Its original intended use is hashing UTF-16 file paths for a C# based [Virtual FileSystem][virtual-filesystem],
and now it's available for Rust.

### Key Features

- **Unstable**: Hash values differ depending on implementation (128-bit vectorised, 256-bit vectorised, non-vectorised).
- **Fast**: Priotises speed over all else.

### Technical Details

SewerHash essentially boils down to the following:

For inputs `< 64 bytes`:

- Unrolled FNV-1a 32/64bit (consuming `usize` bytes at one time, into 2 hashes).
- Hashing remaining 2/4/6 bytes by `2 bytes` at a time.
- Mixing the 2 hashes.

For inputs `>= 64 bytes` (4 `Vector128`s):

- Perform 32bit FNV1a(s) in parallel using `Vec128` / `Vec256` SIMD registers.
- Mix the hashes.
- Leave out remaining `usize` bytes (usually in case of long path, this will just be a file extension).

Notably, it leaves out 1 byte at the end, if length of bytes is an odd number. This is because this algorithm
was originally meant for UTF-16 inputs, thus multiples of 2.

The algorithm is generally low quality for small inputs, instead prioritising hashing speed over distribution.  
Notably, hash quality is very a bit lacking for inputs `< usize`. So <= 4 characters.

[virtual-filesystem]: https://reloaded-project.github.io/reloaded.universal.redirector/