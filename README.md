# OpenMagic: on-chain program

## Getting Started
There are a lot of setups we need to go through to make this work.

### Installing Dependencies
There are 3 dependencies to install: 
  - Solana's CLI tool, see [here](https://docs.solana.com/cli/install-solana-cli-tools)
  - Anchor Framework, see [here](https://project-serum.github.io/anchor/getting-started/installation.html)
  - Metaplex (more on this in a bit)

### Installing Metaplex and the Token Metadata Program
Metaplex is the go-to NFT utility repo in Solana. However it surely does NOT make it easy use. There are two thing we need to do to get to make local developerment possible

First, we need to deploy the token metadata program locally. Clone their [repo](https://github.com/metaplex-foundation/metaplex). For the code to work, put it in the same level with our `ror-story` repo.
```
git clone git@github.com:metaplex-foundation/metaplex.git
```

Next, deploy the program locally:
```
cd metaplex/rust/token-metadata/program
cargo build-bpf
solana program deploy /path/to/metaplex/rust/target/deploy/spl_token_metadata.so
```
Make sure to note the program address and you need to replace the ID for `TOKEN_METADATA_PROGRAM_ID` property in `test/ror-story.js`.

Lastly, notice that we are not using the `spl_token_metadata` from `crate` but instead using the [local path](./programs/ror-story/Cargo.toml). This is because their deployed library is actually broken and doesn't sign transactions properly...

### Running the program
Last thing you'd need to do is to use the Anchor framework to build, deploy and test our Story Mint.
```
anchor build
anchor deploy
anchor test
```

You should see a bunch of addresses being logged out like this
```
========= PRE-TEST LOGS =========
wallet: 5nZodaRztE67Sh8yQehYKdv23iujFW1thvrziWRSHTxf
mint: DW3v8YnsXbah2Tjg8wiHN95jw3mxC5qnGdnShAjw8FEs
token: 5SdkHvdq6azNqpB943VyZGA5KMajC7S9wCmbcvkB1Tw
metadata: 3BCx79xBxerwtdfq8ZK6ewxwcKR8dUUYxJ1XmzgMt9gT
masterEdition: 2BxFwxmg1gTt1fKMDLvh3WuPdMB5MvKMWnfAwawvsMiZ
system token program ID: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
system token metadata program ID: 2AErqaectczAb7pj4prASxbSv6pDGh89XEkQB8HYADxp
system associated token program ID: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
========= END OF LOGS =========
```

The NFT token is in the `token` field (so `5SdkHvdq6azNqpB943VyZGA5KMajC7S9wCmbcvkB1Tw`). You can take it up to the solana explorer, point the network to local (http://127.0.0.1:8899) and see the token and its metadata!
