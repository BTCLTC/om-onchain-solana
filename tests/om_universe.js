const anchor = require("@project-serum/anchor");
const spl_token = require("@solana/spl-token");

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "2AErqaectczAb7pj4prASxbSv6pDGh89XEkQB8HYADxp" // had to deploy that to local. Since metaplex is not deployed by default
);
const ROR_TEST_TREASURY = new anchor.web3.PublicKey(
  "BD8WwZrk3zGB1CX6JkwnoGxFJiEvZ8sHqxSHJ7TqSoob" // my wallet as treasury for now
);
const OM_UNIVERSE_ID = new anchor.web3.PublicKey(
  "64iK3xKi71fxKE6nDR8NZkEjNCFrKWCg521GsBNZ66R8" // my wallet as treasury for now
);
const getMasterEdition = async (mint) => {
  return (
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )
  )[0];
};

const getMetadata = async (mint)=> {
  return (
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )
  )[0];
};

const getTokenWallet = async (wallet, mint) => {
  return (
    await anchor.web3.PublicKey.findProgramAddress(
      [
        wallet.toBuffer(),
        spl_token.TOKEN_PROGRAM_ID.toBuffer(),
        mint.toBuffer()
      ],
      spl_token.ASSOCIATED_TOKEN_PROGRAM_ID
    )
  )[0];
};

const getNodeAccount = async (token, programId) => {
  return (
    await anchor.web3.PublicKey.findProgramAddress(
      [
        'node_account'.toBuffer(),
        token.toBuffer(),
      ],
      programId
    )
  )[0];
}

const createAssociatedTokenAccountInstruction = (
  associatedTokenAddress,
  payer,
  walletAddress,
  splTokenMintAddress,
) => {
  const keys = [
    { pubkey: payer, isSigner: true, isWritable: true },
    { pubkey: associatedTokenAddress, isSigner: false, isWritable: true },
    { pubkey: walletAddress, isSigner: false, isWritable: false },
    { pubkey: splTokenMintAddress, isSigner: false, isWritable: false },
    {
      pubkey: anchor.web3.SystemProgram.programId,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: spl_token.TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    {
      pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
      isSigner: false,
      isWritable: false,
    },
  ];
  return new anchor.web3.TransactionInstruction({
    keys,
    programId: spl_token.ASSOCIATED_TOKEN_PROGRAM_ID,
    data: Buffer.from([]),
  });
}

describe("om-universe", () => {
  // Use a local provider.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  // Program for the tests.
  const program = anchor.workspace.OmUniverse;

  it("mint-genesis", async () => {
  });

  it("mint-children", async () => {
    const myWallet = provider.wallet;
    const mint = anchor.web3.Keypair.generate();
    const token = await getTokenWallet(myWallet.publicKey, mint.publicKey);
    const node_account = await getNodeAccount(token, OM_UNIVERSE_ID)
    const metadata = await getMetadata(mint.publicKey);
    const masterEdition = await getMasterEdition(mint.publicKey);
    const rent = await provider.connection.getMinimumBalanceForRentExemption(
      spl_token.MintLayout.span
    );
    
    console.log("========= PRE-TEST LOGS =========");
    console.log('wallet:', myWallet.publicKey.toString());
    console.log('mint:', mint.publicKey.toString());
    console.log('token:', token.toString());
    console.log('metadata:', metadata.toString());
    console.log('masterEdition:', masterEdition.toString());
    console.log('system token program ID:', spl_token.TOKEN_PROGRAM_ID.toString());
    console.log('system token metadata program ID:', TOKEN_METADATA_PROGRAM_ID?.toString());
    console.log('system associated token program ID:', spl_token.ASSOCIATED_TOKEN_PROGRAM_ID?.toString());
    console.log('========= END OF LOGS =========');

    await program.rpc.mintStory({
      name: "My story",
      URI: "",
      symbol: "ROR"
    }, {
      accounts: {
        // mint vars
        payer: myWallet.publicKey,
        treasury: ROR_TEST_TREASURY,
        mint: mint.publicKey,
        metadata,
        masterEdition,
        mintAuthority: myWallet.publicKey,
        updateAuthority: myWallet.publicKey,
        tokenAccount: token,

        // graph vars
        node_account,

        // system vars
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: spl_token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
      // remainingAccounts:[
      //   {
      //     pubkey: null,
      //     isWritable: false,
      //     isSigner: false,
      //   }
      // ],
      signers: [mint],
      instructions: [
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: myWallet.publicKey,
          newAccountPubkey: mint.publicKey,
          space: spl_token.MintLayout.span,
          lamports: rent,
          programId: spl_token.TOKEN_PROGRAM_ID,
        }),
        spl_token.Token.createInitMintInstruction(
          spl_token.TOKEN_PROGRAM_ID,
          mint.publicKey,
          0,
          myWallet.publicKey,
          myWallet.publicKey
        ),
        createAssociatedTokenAccountInstruction(
          token,
          myWallet.publicKey,
          myWallet.publicKey,
          mint.publicKey
        ),
        spl_token.Token.createMintToInstruction(
          spl_token.TOKEN_PROGRAM_ID,
          mint.publicKey,
          token,
          myWallet.publicKey,
          [],
          1
        ),
      ],
    });
  });

  it("change-type", async() => {
  })
});

/// TOMORROW:
// 1. get minting function done with providing previous parent and populate the right data
// 2. have a mint genesis function and prevent all future calls to that function by storing that somewhere
