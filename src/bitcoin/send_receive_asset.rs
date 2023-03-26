use bdk::{
    bitcoin::{Address, Amount, OutPoint, PublicKey, SigHashType, Transaction, TxIn, TxOut},
    database::MemoryDatabase,
    wallet::{AddressIndex, CoinSelectionAlgorithm, Wallet},
};

// Define the network you want to use, e.g. testnet or mainnet
let network = bdk::bitcoin::Network::Testnet;

// Create a new wallet instance with an empty keychain and a memory database
let wallet = Wallet::new(
    network,
    "".into(), // Enter the seed phrase for an existing wallet, or leave empty to create a new one
    MemoryDatabase::default(),
    bdk::blockchain::EsploraBlockchain::from(
        "https://blockstream.info/testnet/api".to_string(),
        network,
    )
    .unwrap(),
);

// Generate a new receive address
let address = wallet.get_address(AddressIndex::New).unwrap();

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
let to_address = Address::from_str("tb1q5r6ju5y9d5au8h5mlwmmtgrg0hsmvplp6wdsf8").unwrap();
let fee_rate = 100; // Satoshis per byte
let mut builder = wallet.build_tx();
builder
    .add_recipient(to_address.script_pubkey(), Amount::from_sat(100))
    .fee_rate(fee_rate)
    .enable_rbf()
    .do_not_spend_change();
let (psbt, details) = builder.finish().unwrap();

// Print the PSBT to the console
println!("Unsigned PSBT: {}", psbt);

// Sign the PSBT with the wallet's private keys
let signed_psbt = wallet.sign(psbt, None).unwrap();

// Print the signed PSBT to the console
println!("Signed PSBT: {}", signed_psbt);

// Broadcast the transaction to the Bitcoin network
let txid = wallet.broadcast(signed_psbt).unwrap();

// Print the transaction ID to the console
println!("Transaction ID: {}", txid);

