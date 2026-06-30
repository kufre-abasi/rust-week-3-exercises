use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        // TODO: Construct a CompactSize from a u64 value
        CompactSize { value }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Encode according to Bitcoin's CompactSize format:
        // [0x00–0xFC] => 1 byte
        // [0xFDxxxx] => 0xFD + u16 (2 bytes)
        // [0xFExxxxxxxx] => 0xFE + u32 (4 bytes)
        // [0xFFxxxxxxxxxxxxxxxx] => 0xFF + u64 (8 bytes)
        let mut bytes = Vec::new();
        if self.value <= 0xFC {
            bytes.push(self.value as u8);
        } else if self.value <= 0xFFFF {
            bytes.push(0xFD);
            bytes.extend_from_slice(&(self.value as u16).to_le_bytes());
        } else if self.value <= 0xFFFFFFFF {
            bytes.push(0xFE);
            bytes.extend_from_slice(&(self.value as u32).to_le_bytes());
        } else {
            bytes.push(0xFF);
            bytes.extend_from_slice(&self.value.to_le_bytes());
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Decode CompactSize, returning value and number of bytes consumed.
        // First check if bytes is empty.
        // Check that enough bytes are available based on prefix.
        if bytes.is_empty() {
            return Err(BitcoinError::InsufficientBytes);
        }
        let prefix = bytes[0];
        if prefix <= 0xFC {
            Ok((
                CompactSize {
                    value: prefix as u64,
                },
                1,
            ))
        } else if prefix == 0xFD {
            if bytes.len() < 3 {
                return Err(BitcoinError::InsufficientBytes);
            }
            let val = u16::from_le_bytes([bytes[1], bytes[2]]);
            Ok((CompactSize { value: val as u64 }, 3))
        } else if prefix == 0xFE {
            if bytes.len() < 5 {
                return Err(BitcoinError::InsufficientBytes);
            }
            let val = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
            Ok((CompactSize { value: val as u64 }, 5))
        } else {
            if bytes.len() < 9 {
                return Err(BitcoinError::InsufficientBytes);
            }
            let val = u64::from_le_bytes([
                bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
            ]);
            Ok((CompactSize { value: val }, 9))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Serialize as a hex-encoded string (32 bytes => 64 hex characters)
        let hex_str = hex::encode(self.0);
        serializer.serialize_str(&hex_str)
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Parse hex string into 32-byte array
        // Use `hex::decode`, validate length = 32
        let hex_str = String::deserialize(deserializer)?;
        let decoded = hex::decode(&hex_str).map_err(serde::de::Error::custom)?;
        if decoded.len() != 32 {
            return Err(serde::de::Error::custom(
                "Txid must be exactly 32 bytes i.e 64 hex characters",
            ));
        }
        let mut txid = [0u8; 32];
        txid.copy_from_slice(&decoded);
        Ok(Txid(txid))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        // TODO: Create an OutPoint from raw txid bytes and output index
        OutPoint {
            txid: Txid(txid),
            vout,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize as: txid (32 bytes) + vout (4 bytes, little-endian)
        let mut bytes = Vec::with_capacity(36);
        bytes.extend_from_slice(&self.txid.0);
        bytes.extend_from_slice(&self.vout.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize 36 bytes: txid[0..32], vout[32..36]
        // Return error if insufficient bytes
        if bytes.len() < 36 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let mut txid_bytes = [0u8; 32];
        txid_bytes.copy_from_slice(&bytes[0..32]);
        let vout = u32::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]);
        Ok((
            OutPoint {
                txid: Txid(txid_bytes),
                vout,
            },
            36,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self {
        // TODO: Simple constructor
        Script { bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Prefix with CompactSize (length), then raw bytes
        let len_compact = CompactSize::new(self.bytes.len() as u64);
        let mut result = len_compact.to_bytes();
        result.extend_from_slice(&self.bytes);
        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Parse CompactSize prefix, then read that many bytes
        // Return error if not enough bytes
        let (len_compact, consumed) = CompactSize::from_bytes(bytes)?;
        let len = len_compact.value as usize;
        if bytes.len() < consumed + len {
            return Err(BitcoinError::InsufficientBytes);
        }
        let script_bytes = bytes[consumed..consumed + len].to_vec();
        Ok((
            Script {
                bytes: script_bytes,
            },
            consumed + len,
        ))
    }
}

impl Deref for Script {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        // TODO: Allow &Script to be used as &[u8]
        &self.bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self {
        // TODO: Basic constructor
        TransactionInput {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize: OutPoint + Script (with CompactSize) + sequence (4 bytes LE)
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.previous_output.to_bytes());
        bytes.extend_from_slice(&self.script_sig.to_bytes());
        bytes.extend_from_slice(&self.sequence.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize in order:
        // - OutPoint (36 bytes)
        // - Script (with CompactSize)
        // - Sequence (4 bytes)
        let mut consumed = 0;

        let (previous_output, outpoint_len) = OutPoint::from_bytes(&bytes[consumed..])?;
        consumed += outpoint_len;

        let (script_sig, script_len) = Script::from_bytes(&bytes[consumed..])?;
        consumed += script_len;

        if bytes.len() - consumed < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let sequence = u32::from_le_bytes([
            bytes[consumed],
            bytes[consumed + 1],
            bytes[consumed + 2],
            bytes[consumed + 3],
        ]);
        consumed += 4;

        Ok((
            TransactionInput {
                previous_output,
                script_sig,
                sequence,
            },
            consumed,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self {
        // TODO: Construct a transaction from parts
        BitcoinTransaction {
            version,
            inputs,
            lock_time,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Format:
        // - version (4 bytes LE)
        // - CompactSize (number of inputs)
        // - each input serialized
        // - lock_time (4 bytes LE)
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.version.to_le_bytes());

        let inputs_count = CompactSize::new(self.inputs.len() as u64);
        bytes.extend_from_slice(&inputs_count.to_bytes());

        for input in &self.inputs {
            bytes.extend_from_slice(&input.to_bytes());
        }

        bytes.extend_from_slice(&self.lock_time.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Read version, CompactSize for input count
        // Parse inputs one by one
        // Read final 4 bytes for lock_time
        let mut consumed = 0;

        if bytes.len() < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        consumed += 4;

        let (inputs_count, count_len) = CompactSize::from_bytes(&bytes[consumed..])?;
        consumed += count_len;

        let mut inputs = Vec::with_capacity(inputs_count.value as usize);
        for _ in 0..inputs_count.value {
            let (input, input_len) = TransactionInput::from_bytes(&bytes[consumed..])?;
            consumed += input_len;
            inputs.push(input);
        }

        if bytes.len() - consumed < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let lock_time = u32::from_le_bytes([
            bytes[consumed],
            bytes[consumed + 1],
            bytes[consumed + 2],
            bytes[consumed + 3],
        ]);
        consumed += 4;

        Ok((
            BitcoinTransaction {
                version,
                inputs,
                lock_time,
            },
            consumed,
        ))
    }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format a user-friendly string showing version, inputs, lock_time
        // Display scriptSig length and bytes, and previous output info
        writeln!(f, "BitcoinTransaction:")?;
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Lock Time: {}", self.lock_time)?;
        writeln!(f, "Inputs ({}):", self.inputs.len())?;
        for (i, input) in self.inputs.iter().enumerate() {
            writeln!(f, "    Input {}:", i)?;
            writeln!(
                f,
                "      Previous Output Txid: {}",
                hex::encode(input.previous_output.txid.0)
            )?;
            writeln!(
                f,
                "      Previous Output Vout: {}",
                input.previous_output.vout
            )?;
            writeln!(
                f,
                "      ScriptSig Length: {}",
                input.script_sig.bytes.len()
            )?;
            writeln!(
                f,
                "      ScriptSig Bytes: {}",
                hex::encode(&input.script_sig.bytes)
            )?;
            writeln!(f, "      Sequence: {:#010X}", input.sequence)?;
        }
        Ok(())
    }
}
