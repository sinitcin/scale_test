use codec::{Decode, Encode};

#[derive(Clone, Encode, Default, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
// This type we use only for demonstration. The current tests work only with `data` field.
// Other fields we don't use here.
//
//   In the core project we will use real types !!!
//
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
    NftIssueV1 { id: u64, token_name: Vec<u8> },
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
        data: TxDataV1::NftIssueV1 {
            id: 160,
            token_name: vec![0, 1, 2, 3],
        },
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
    let borrowed = TransactionOutputV1::encode(&old_version);
    let mut enc_old_version = borrowed.as_slice();
    let borrowed = TransactionOutputV2::encode(&new_version);
    let mut enc_new_version = borrowed.as_slice();

    dbg!(&enc_old_version);
    dbg!(&enc_new_version);

    // If we decode an old version of transaction in TransactionOutputV2
    let dec_old = TransactionOutputV2::decode(&mut enc_old_version).ok();
    dbg!(&dec_old);

    // If we decode an new version of transaction in TransactionOutputV1
    let dec_new = TransactionOutputV1::decode(&mut enc_new_version).ok();
    dbg!(&dec_new);

    let err = TransactionOutputV1::decode(&mut enc_new_version);
    if let Err(err) = err {
        dbg!(err.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_old_node_read_a_new_data() {
        // In this test, we are going to check the possibility to read new 
        //  versions of data on an old codebase. We make a new version of data,
        //  encode it to raw bytes and then we are going to decode it as an old
        //  version of data.
        //
        // Expected behavior - return error.
        let new_version = TxDataV2::NftIssueV2 {
            id: 10000,
            token_name: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00],
            owner: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00],
        };
        assert!(TxDataV1::decode(&mut &TxDataV2::encode(&new_version)[..]).is_err());
    }

    #[test]
    fn a_new_node_read_an_old_data() {
        // In this test, we are going to check the possibility to read an old 
        //  version of data on a new codebase. We make an old version of data,
        //  encode it to raw bytes and then we are going to decode it as a new
        //  version of data.
        //
        // Expected behavior - return data.
        let old_version = TxDataV1::NftIssueV1 {
                id: 160,
                token_name: vec![0, 1, 2, 3],
            };
        assert!(TxDataV2::decode(&mut &TxDataV1::encode(&old_version)[..]).is_ok());
    }

    #[test]
    fn check_immutability() {
        // In this test we are checking is changed the current version of data in the project or not.
        let id_v1 = 0;

        #[derive(Clone, Encode, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
        enum BlankTxDataV1 {
            #[codec(index = 1)]
            // shuffle token name and id in positions
            NftIssueV1 { token_name: Vec<u8>, id: u64 },
        }

        let current_data = TxDataV1::NftIssueV1 {
            id: id_v1,
            token_name: vec![1, 2, 3, 4],
        };
        let enc_data = TxDataV1::encode(&current_data);

        let dec_data = BlankTxDataV1::decode(&mut &enc_data[..]).expect("Can't decode data");
        dbg!(&dec_data);

        let id_blank = match dec_data {
            BlankTxDataV1::NftIssueV1 { token_name: _, id } => id,
        };
        // The error raised over here
        assert_ne!(id_v1, id_blank);

        #[derive(Clone, Encode, Decode, Eq, PartialEq, PartialOrd, Ord, Debug)]
        enum BlankTxDataV2 {
            #[codec(index = 1)]
            // Return the previous positions for the token name and id
            NftIssueV1 { id: u64, token_name: Vec<u8> },
        }

        let dec_data = BlankTxDataV2::decode(&mut &enc_data[..]).expect("Can't decode data");
        let id_blank = match dec_data {
            BlankTxDataV2::NftIssueV1 { token_name: _, id } => id,
        };
        // As far as we can see if fields ordering correct then no errors occurred
        assert_eq!(id_v1, id_blank);
    }
}
