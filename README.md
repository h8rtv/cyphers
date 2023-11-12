# cyphers
Simple implementation of Cesar and Vernam cyphers

## Usage

### Cesar

```bash
$ cargo run -- cesar -c -k 12 < path/to/file.txt > secret.txt # Encrypt
$ cargo run -- cesar -d -k 12 < secret.txt > text.txt # Decrypt
```

### Vernam

```bash
$ cargo run -- vernam -c -o key.txt < path/to/file.txt > secret.txt # Encrypt
$ cargo run -- vernam -d -k key.txt < secret.txt > text.txt # Decrypt
```

### Cryptanalysis to find Cesar keys on portuguese texts

```bash
$ cargo run -- cryptanalysis < secret.txt
```
