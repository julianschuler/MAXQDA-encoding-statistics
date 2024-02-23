# MAXQDA encoding statistics

This repository contains a small and simple program to analyze text data encoded with MAXQDA.

For now, it prints the number of encoded sentences, total sentences and the percentage of encoded sentences. A sentence is considered to be a string terminated by a period.

## Usage

Install a rust toolchain, clone the directory and switch to it.

The following assumes you have a file `data.txt` with the encoded text and `data.csv` with the exported encodings (Important: Export as UTF-8 CSV) in the same directory:

```sh
cargo run data
```

You can analyze more than one encoded text at the same time by passing multiple arguments (assuming there exist `data1.txt`, `data1.csv`, `data2.txt` and `data2.csv`):

```sh
cargo run data1 data2
```

## License

This project is licensed under the MIT license, see [`LICENSE.md`](LICENSE.md) for further information.
