use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use wormhole_io::{Readable, Writeable};

const PAYLOAD_TRANSFER: u8 = 0;
const PAYLOAD_TRANSFER_ALL: u8 = 1;
const PAYLOAD_TRANSFER_ACCOUNTS: u8 = 2;
const PAYLOAD_TRANSFER_TOKEN_ACCOUNTS: u8 = 3;
const PAYLOAD_CHANGE_UPGRADE_AUTHORITY: u8 = 4;
const PAYLOAD_UPGRADE_PROGRAM: u8 = 5;

pub const HELLO_MESSAGE_MAX_LENGTH: usize = 512;

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
    pub token: [u8; 32],
    pub source: [u8; 32],
    pub destination: [u8; 32],
    pub amount: u64
}

impl AnchorSerialize for TransferMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        PAYLOAD_TRANSFER.write(writer)?;
        self.token.write(writer)?;
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
                token: <[u8; 32]>::read(reader)?,
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
