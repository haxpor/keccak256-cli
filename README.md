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

2. With `--method-id` flag

```
$ echo "approve(address spender, uint256 amount)" | keccak256 --method-id
0xc292633e
```

3. Additional with `--no-0x` flag

```
$ echo "approve(address spender, uint256 amount)" | keccak256 --method-id
c292633e
```

4. Encode content of input file

```
$ cat myfile.txt
line1
line2
line3

$ cat myfile.txt | keccak256
0xf500641328ecab72baae3555278fc470860a4a243e62e9534120d8405def6143
```

# License
MIT, Wasin Thonkaew
