use codec::{Decode, Encode};

#[derive(Clone, Encode, Default, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
struct NeverMindWhatType();

// Version 1
#[derive(Clone, Encode, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct TransactionOutputV1 {
    pub(crate) value: NeverMindWhatType,
    pub(crate) destination: NeverMindWhatType,
    pub(crate) data: TxDataV1,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum TxDataV1 {
    #[codec(index = 1)]
    NftIssueV1 { id: u64 },
}

// Version 2
#[derive(Clone, Encode, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct TransactionOutputV2 {
    pub(crate) value: NeverMindWhatType,
    pub(crate) destination: NeverMindWhatType,
    pub(crate) data: TxDataV2,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum TxDataV2 {
    #[codec(index = 2)]
    NftIssueV2 {
        id: u64,
        token_name: Vec<u8>,
        owner: Vec<u8>,
    },
    #[codec(index = 1)]
    NftIssueV1 { id: u64 },
}

fn main() {
    let old_version = TransactionOutputV1 {
        value: NeverMindWhatType::default(),
        destination: NeverMindWhatType::default(),
        data: TxDataV1::NftIssueV1 { id: 160 },
    };

    let new_version = TransactionOutputV2 {
        value: NeverMindWhatType::default(),
        destination: NeverMindWhatType::default(),
        data: TxDataV2::NftIssueV2 {
            id: 10000,
            token_name: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00],
            owner: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00],
        },
    };

    println!("Let's encode all txs");
    let mut borrowed = TransactionOutputV1::encode(&old_version);
    let mut enc_old_version = borrowed.as_slice();
    let mut borrowed = TransactionOutputV2::encode(&new_version);
    let mut enc_new_version = borrowed.as_slice();

    dbg!(&enc_old_version);
    dbg!(&enc_new_version);

    // If we decode an old version of transaction in TransactionOutputV2
    let dec_old = TransactionOutputV2::decode(&mut enc_old_version).ok();
    dbg!(&dec_old);

    // If we decode an new version of transaction in TransactionOutputV1
    let dec_new = TransactionOutputV1::decode(&mut enc_new_version).ok();
    dbg!(&dec_new);
}
