# Internal audit of lockbox-solana
The review has been performed based on the contract code in the following repository:<br>
`https://github.com/valory-xyz/lockbox-governor-solana` <br>
commit: `7a1381a0885a0fad49be24225d6a483c54701a76` or `v0.1.0-pre-internal-audit`<br> 

## Objectives
The audit focused on contracts in folder `programs`.

### Flatten version
N/A

### OS Requirments checks
Pre-requisites (README.md)
```
anchor --version
anchor-cli 0.29.0
solana --version
solana-cli 1.18.1 (src:5d824a36; feat:756280933, client:SolanaLabs)
rustc --version
rustc 1.74.1 (a28077b28 2023-12-04)
```
Checks - passed [-]
```
bash setup-env.sh 
anchor --version
anchor-cli 0.29.0
solana --version
solana-cli 1.18.5 (src:928d8ac2; feat:3352961542, client:SolanaLabs)
rustc --version
rustc 1.78.0 (9b00956e5 2024-04-29)
```
Issue: The versions written in the readme do not match in detail the ones in the script.
```
In setup-env.sh
RUSTVER="1.78"
SOLANAVER="1.18.5"
ANCHORVER="0.29.0"
```

