use ldk::{
    chain::keysinterface::Sign as _, 
    chain::transaction::OutPoint as _, 
    chain::keysinterface::KeysInterface as _, 
    chain::keysinterface::InMemorySigner as _, 
    wallet::{AddressIndex, Wallet, WalletContext},
    blockfilter::{BlockFilter, GolombFilter},
    logger::Logger,
    miniscript::descriptor::DescriptorPublicKey,
    miniscript::Descriptor,
    util::events::{Event, MessageSendEvent},
};

use bitcoin::{
    blockdata::{opcodes::All, script::Builder, transaction::OutPoint, transaction::TxOut},
    util::amount::Amount,
};

// Define the network you want to use, e.g. testnet or mainnet
let network = bitcoin::network::constants::Network::Testnet;

// Create a new wallet instance with an empty keychain and an in-memory signer
let keys_interface = InMemorySigner::new();
let mut wallet = Wallet::new(
    WalletContext::new(network),
    keys_interface.clone(),
    BlockFilter::new(GolombFilter::new_for_key(network.genesis_blockheader().block_hash())),
    Logger::new("wallet".into()),
);

// Generate a new receive address
let address = wallet.get_new_address().unwrap();

// Print the address to the console
println!("Receive address: {}", address);

// To receive Bitcoin to this address, send it to the displayed address using a Bitcoin wallet or exchange

// Wait for the transaction to confirm
// Note: This is not strictly necessary if you only want to receive Bitcoin and not spend it immediately

// Check the wallet's balance and transactions
let (balance, txs) = wallet.sync().unwrap();

// Print the wallet's balance to the console
println!("Wallet balance: {}", balance);

// Create a new transaction that sends 100 satoshis to the specified address
let to_address = bitcoin::Address::from_str("tb1q5r6ju5y9d5au8h5mlwmmtgrg0hsmvplp6wdsf8").unwrap();
let fee_rate = 100; // Satoshis per byte

let mut builder = bitcoin::TransactionBuilder::new();

let mut txin = bitcoin::TxIn {
    previous_output: OutPoint {
        txid: tx_id,
        vout: output_index,
    },
    sequence: 0xFFFFFFFF,
    witness: Vec::new(),
    script_sig: bitcoin::Script::default(),
};

builder.add_input(txin, txout);

let output_script = Builder::new()
    .push_opcode(All::OP_DUP)
    .push_opcode(All::OP_HASH160)
    .push_slice(&to_address.script_pubkey().as_bytes()[2..22])
    .push_opcode(All::OP_EQUALVERIFY)
    .push_opcode(All::OP_CHECKSIG)
    .into_script();

builder.add_output(TxOut {
    value: Amount::from_sat(100),
    script_pubkey: output_script,
});

builder.set_version(2);
builder.set_lock_time(0);

// Sign the transaction with the wallet's private keys
let descriptor = Descriptor::<DescriptorPublicKey>::new(
    "wpkh(.....)", // Enter your descriptor here
    bitcoin::ScriptContext::new(
        bitcoin::network::constants::Network::Testnet, // Replace with mainnet for live transactions
        bitcoin::ScriptType::P2WPKHv0,
    ),
)
.unwrap();
let signers = vec![keys_interface.clone()];
let mut psbt =

