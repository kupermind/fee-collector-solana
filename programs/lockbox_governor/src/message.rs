use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use wormhole_io::{Readable, Writeable};

const PAYLOAD_TRANSFER: u8 = 0;
const PAYLOAD_TRANSFER_ALL: u8 = 1;
const PAYLOAD_TRANSFER_TOKEN_ACCOUNTS: u8 = 2;
const PAYLOAD_SET_UPGRADE_AUTHORITY: u8 = 3;
const PAYLOAD_UPGRADE_PROGRAM: u8 = 4;

//pub const HELLO_MESSAGE_MAX_LENGTH: usize = 512;

//#[derive(Clone)]
/// Expected message types for this program. Only valid payloads are:
/// * `Alive`: Payload ID == 0. Emitted when [`initialize`](crate::initialize)
///  is called).
/// * `Hello`: Payload ID == 1. Emitted when
/// [`send_message`](crate::send_message) is called).
///
/// Payload IDs are encoded as u8.
// pub enum HelloWorldMessage {
//     Alive { program_id: Pubkey },
//     Hello { message: Vec<u8> },
// }

#[derive(Clone)]
pub struct TransferMessage {
    pub source: [u8; 32],
    pub destination: [u8; 32],
    pub amount: u64
}

impl AnchorSerialize for TransferMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        PAYLOAD_TRANSFER.write(writer)?;
        self.source.write(writer)?;
        self.destination.write(writer)?;
        self.amount.write(writer)?;

        Ok(())
    }
}

impl AnchorDeserialize for TransferMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let selector = u8::read(reader)?;

        match selector {
            PAYLOAD_TRANSFER => Ok(TransferMessage {
                source: <[u8; 32]>::read(reader)?,
                destination: <[u8; 32]>::read(reader)?,
                amount: u64::read(reader)?,
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid payload ID",
            )),
        }
    }
}

#[derive(Clone)]
pub struct TransferAllMessage {
    pub source_sol: [u8; 32],
    pub source_olas: [u8; 32],
    pub destination_sol: [u8; 32],
    pub destination_olas: [u8; 32]
}

impl AnchorSerialize for TransferAllMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        PAYLOAD_TRANSFER_ALL.write(writer)?;
        self.source_sol.write(writer)?;
        self.source_olas.write(writer)?;
        self.destination_sol.write(writer)?;
        self.destination_olas.write(writer)?;

        Ok(())
    }
}

impl AnchorDeserialize for TransferAllMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let selector = u8::read(reader)?;

        match selector {
            PAYLOAD_TRANSFER_ALL => Ok(TransferAllMessage {
                source_sol: <[u8; 32]>::read(reader)?,
                source_olas: <[u8; 32]>::read(reader)?,
                destination_sol: <[u8; 32]>::read(reader)?,
                destination_olas: <[u8; 32]>::read(reader)?
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid payload ID",
            )),
        }
    }
}

#[derive(Clone)]
pub struct TransferTokenAccountsMessage {
    pub source_sol: [u8; 32],
    pub source_olas: [u8; 32],
    pub destination: [u8; 32]
}

impl AnchorSerialize for TransferTokenAccountsMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        PAYLOAD_TRANSFER_TOKEN_ACCOUNTS.write(writer)?;
        self.source_sol.write(writer)?;
        self.source_olas.write(writer)?;
        self.destination.write(writer)?;

        Ok(())
    }
}

impl AnchorDeserialize for TransferTokenAccountsMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let selector = u8::read(reader)?;

        match selector {
            PAYLOAD_TRANSFER_TOKEN_ACCOUNTS => Ok(TransferTokenAccountsMessage {
                source_sol: <[u8; 32]>::read(reader)?,
                source_olas: <[u8; 32]>::read(reader)?,
                destination: <[u8; 32]>::read(reader)?
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid payload ID",
            )),
        }
    }
}

#[derive(Clone)]
pub struct SetUpgradeAuthorityMessage {
    pub program_id_bytes: [u8; 32],
    pub upgrade_authority: [u8; 32]
}

impl AnchorSerialize for SetUpgradeAuthorityMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        PAYLOAD_SET_UPGRADE_AUTHORITY.write(writer)?;
        self.program_id_bytes.write(writer)?;
        self.upgrade_authority.write(writer)?;

        Ok(())
    }
}

impl AnchorDeserialize for SetUpgradeAuthorityMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let selector = u8::read(reader)?;

        match selector {
            PAYLOAD_SET_UPGRADE_AUTHORITY => Ok(SetUpgradeAuthorityMessage {
                program_id_bytes: <[u8; 32]>::read(reader)?,
                upgrade_authority: <[u8; 32]>::read(reader)?
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid payload ID",
            )),
        }
    }
}

#[derive(Clone)]
pub struct UpgradeProgramMessage {
    pub program_id_bytes: [u8; 32],
    pub buffer_account_bytes: [u8; 32],
    pub spill_account_bytes: [u8; 32]
}

impl AnchorSerialize for UpgradeProgramMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        PAYLOAD_UPGRADE_PROGRAM.write(writer)?;
        self.program_id_bytes.write(writer)?;
        self.buffer_account_bytes.write(writer)?;
        self.spill_account_bytes.write(writer)?;

        Ok(())
    }
}

impl AnchorDeserialize for UpgradeProgramMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let selector = u8::read(reader)?;

        match selector {
            PAYLOAD_UPGRADE_PROGRAM => Ok(UpgradeProgramMessage {
                program_id_bytes: <[u8; 32]>::read(reader)?,
                buffer_account_bytes: <[u8; 32]>::read(reader)?,
                spill_account_bytes: <[u8; 32]>::read(reader)?
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid payload ID",
            )),
        }
    }
}

//     #[test]
//     fn test_message_hello_too_large() -> Result<()> {
//         let n: usize = 513;
//         let raw_message = {
//             let mut out = Vec::with_capacity(n);
//             for _ in 0..n {
//                 out.push(33u8)
//             }
//             String::from_utf8(out).unwrap()
//         };
//         let msg = HelloWorldMessage::Hello {
//             message: raw_message.as_bytes().to_vec(),
//         };
//
//         // Attempt to serialize message above.
//         let mut encoded = Vec::new();
//         match msg.serialize(&mut encoded) {
//             Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidInput),
//             _ => assert!(false, "not supposed to serialize"),
//         };
//
//         // Serialize manually and then attempt to deserialize.
//         encoded.push(PAYLOAD_ID_HELLO);
//         encoded.extend_from_slice(&(raw_message.len() as u16).to_be_bytes());
//         encoded.extend_from_slice(raw_message.as_bytes());
//
//         assert_eq!(
//             encoded.len(),
//             size_of::<u8>() + size_of::<u16>() + raw_message.len()
//         );
//
//         // Verify Payload ID.
//         assert_eq!(encoded[0], PAYLOAD_ID_HELLO);
//
//         // Verify message length.
//         let mut message_len_bytes = [0u8; 2];
//         message_len_bytes.copy_from_slice(&encoded[1..3]);
//         assert_eq!(
//             u16::from_be_bytes(message_len_bytes) as usize,
//             raw_message.len()
//         );
//
//         match HelloWorldMessage::deserialize(&mut encoded.as_slice()) {
//             Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidInput),
//             _ => assert!(false, "not supposed to deserialize"),
//         };
//
//         Ok(())
//     }
// }
