# keccak256-cli
keccak256 encryption cli program; accepts input text from stdin. Suitable for commandline users.

It is the same algorithm used to encode method signature to get method id as shown
on those blockchain scans e.g. etherscan.com, bscscan.com, or polygonscan.com

# Installation

```
cargo install keccak256-cli
```

# Usage

1. Normal with no arguments

```
$ echo "approve(address spender, uint256 amount)" | keccak256
0xc292633e5eef92ab7edc697475bda42257ae9897c520dfbbd8cb1c43bbae73ea
```

2. With `--method-id` flag to output as method id; suitable for method signature

```
$ echo "approve(address spender, uint256 amount)" | keccak256 --method-id
0xc292633e
```

3. Additional with `--no-0x` flag to avoid prefixed `0x`

```
$ echo "approve(address spender, uint256 amount)" | keccak256 --method-id --no-0x
c292633e
```

4. Encode content of input file with various flags

```
$ cat myfile.txt
line1
line2
line3

$ cat myfile.txt | keccak256
0xf500641328ecab72baae3555278fc470860a4a243e62e9534120d8405def6143

$ cat myfile.txt | keccak256 --each-line
0x231e770f96ffa24e1fec6b52fd47915ab8c491356d1d02de165a5bf6f3d72280
0x12221da17783f65564f9fab0b1bc89c5c556b4268b4c98279ead6beb6fd86ceb
0x8bb5ab46551d22e650ad150d9cefef6ab6e65e0a2ae3a71c47c645a777314f19

$ cat myfile.txt | keccak256 --each-line --no-0x --method-id
231e770f
12221da1
8bb5ab46
```

# License
MIT, Wasin Thonkaew
