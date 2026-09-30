use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid as RawKsuid, KsuidMs as RawKsuidMs, KsuidLike};

pub const DEFAULT_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn parse_alphabet(alphabet: Option<String>) -> Result<[u8; 32]> {
  if let Some(a) = alphabet {
    let bytes = a.as_bytes();
    if bytes.len() != 32 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Alphabet must be exactly 32 characters, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);
    Ok(arr)
  } else {
    Ok(*DEFAULT_CROCKFORD_ALPHABET)
  }
}

pub fn shuffle_alphabet_with_seed(seed: &str) -> String {
  let mut bytes = *DEFAULT_CROCKFORD_ALPHABET;
  let mut hash: u64 = 5381;
  for b in seed.bytes() {
    hash = hash.wrapping_mul(33).wrapping_add(b as u64);
  }
  let mut rng_state = hash;
  for i in (1..32).rev() {
    rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let j = (rng_state >> 32) as usize % (i + 1);
    bytes.swap(i, j);
  }
  String::from_utf8(bytes.to_vec()).unwrap()
}

pub fn encode_crockford(bytes: &[u8], alphabet: &[u8; 32]) -> String {
  let mut result = String::with_capacity((bytes.len() * 8 + 4) / 5);
  let mut buffer: u64 = 0;
  let mut bits_in_buffer = 0;

  for &byte in bytes {
    buffer = (buffer << 8) | (byte as u64);
    bits_in_buffer += 8;

    while bits_in_buffer >= 5 {
      bits_in_buffer -= 5;
      let index = ((buffer >> bits_in_buffer) & 0x1f) as usize;
      result.push(alphabet[index] as char);
    }
  }

  if bits_in_buffer > 0 {
    let index = ((buffer << (5 - bits_in_buffer)) & 0x1f) as usize;
    result.push(alphabet[index] as char);
  }

  result
}

pub fn decode_crockford(input: &str, alphabet: &[u8; 32]) -> Result<Vec<u8>> {
  let mut char_map = [255u8; 256];
  for (i, &b) in alphabet.iter().enumerate() {
    char_map[b as usize] = i as u8;
    let ch = b as char;
    if ch.is_ascii_uppercase() {
      char_map[ch.to_ascii_lowercase() as usize] = i as u8;
    } else if ch.is_ascii_lowercase() {
      char_map[ch.to_ascii_uppercase() as usize] = i as u8;
    }
  }

  if alphabet == DEFAULT_CROCKFORD_ALPHABET {
    char_map[b'O' as usize] = char_map[b'0' as usize];
    char_map[b'o' as usize] = char_map[b'0' as usize];
    char_map[b'I' as usize] = char_map[b'1' as usize];
    char_map[b'i' as usize] = char_map[b'1' as usize];
    char_map[b'L' as usize] = char_map[b'1' as usize];
    char_map[b'l' as usize] = char_map[b'1' as usize];
  }

  let mut buffer: u64 = 0;
  let mut bits_in_buffer = 0;
  let mut bytes = Vec::new();

  for byte in input.bytes() {
    if byte == b'-' || byte == b' ' {
      continue;
    }
    let val = char_map[byte as usize];
    if val == 255 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid Crockford Base32 character: '{}'", byte as char),
      ));
    }
    buffer = (buffer << 5) | (val as u64);
    bits_in_buffer += 5;

    if bits_in_buffer >= 8 {
      bits_in_buffer -= 8;
      let b = ((buffer >> bits_in_buffer) & 0xff) as u8;
      bytes.push(b);
    }
  }

  Ok(bytes)
}

#[napi(object)]
#[derive(Default)]
pub struct ToStringOptions {
  pub enc: Option<String>,
  pub alphabet: Option<String>,
}

#[napi]
pub struct CrockfordBase32 {
  alphabet: [u8; 32],
}

#[napi]
impl CrockfordBase32 {
  #[napi]
  pub fn default_alphabet() -> String {
    String::from_utf8(DEFAULT_CROCKFORD_ALPHABET.to_vec()).unwrap()
  }

  #[napi(constructor)]
  pub fn new(alphabet: Option<String>) -> Result<Self> {
    let alphabet_bytes = parse_alphabet(alphabet)?;
    Ok(CrockfordBase32 {
      alphabet: alphabet_bytes,
    })
  }

  #[napi]
  pub fn shuffle_alphabet(seed: Either<String, f64>) -> String {
    let seed_str = match seed {
      Either::A(s) => s,
      Either::B(n) => n.to_string(),
    };
    shuffle_alphabet_with_seed(&seed_str)
  }

  #[napi]
  pub fn encode(&self, bytes: Buffer) -> String {
    encode_crockford(bytes.as_ref(), &self.alphabet)
  }

  #[napi]
  pub fn decode(&self, input: String) -> Result<Buffer> {
    let vec = decode_crockford(&input, &self.alphabet)?;
    Ok(Buffer::from(vec))
  }

  #[napi]
  pub fn encode_number(&self, num: i64) -> String {
    let mut n = num as u64;
    if n == 0 {
      return (self.alphabet[0] as char).to_string();
    }
    let mut chars = Vec::new();
    while n > 0 {
      let index = (n & 0x1f) as usize;
      chars.push(self.alphabet[index] as char);
      n >>= 5;
    }
    chars.into_iter().rev().collect()
  }

  #[napi]
  pub fn decode_number(&self, input: String) -> Result<i64> {
    let mut char_map = [255u8; 256];
    for (i, &b) in self.alphabet.iter().enumerate() {
      char_map[b as usize] = i as u8;
      let ch = b as char;
      if ch.is_ascii_uppercase() {
        char_map[ch.to_ascii_lowercase() as usize] = i as u8;
      } else if ch.is_ascii_lowercase() {
        char_map[ch.to_ascii_uppercase() as usize] = i as u8;
      }
    }
    let mut n: u64 = 0;
    for byte in input.bytes() {
      if byte == b'-' || byte == b' ' {
        continue;
      }
      let val = char_map[byte as usize];
      if val == 255 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid Crockford Base32 character: '{}'", byte as char),
        ));
      }
      n = (n << 5) | (val as u64);
    }
    Ok(n as i64)
  }

  #[napi]
  pub fn encode_ksuid(&self, ksuid: &Ksuid) -> String {
    encode_crockford(ksuid.inner.bytes().as_ref(), &self.alphabet)
  }
}

#[napi]
pub struct Ksuid {
  inner: RawKsuid,
}

#[napi]
impl Ksuid {
  #[napi(getter)]
  pub fn payload_bytes() -> u32 {
    16
  }

  #[napi(getter)]
  pub fn bytes_len() -> u32 {
    20
  }

  #[napi(getter)]
  pub fn string_encoded_size() -> u32 {
    27
  }

  #[napi(constructor)]
  pub fn new(input: Option<Either<String, Buffer>>) -> Result<Self> {
    if let Some(val) = input {
      match val {
        Either::A(s) => {
          let inner = if s.len() == 32 {
            let bytes = decode_crockford(&s, DEFAULT_CROCKFORD_ALPHABET)?;
            if bytes.len() != 20 {
              return Err(Error::new(
                Status::InvalidArg,
                format!("Expected 20 bytes from Crockford Base32, got {}", bytes.len()),
              ));
            }
            let mut arr = [0u8; 20];
            arr.copy_from_slice(&bytes);
            RawKsuid::from_bytes(arr)
          } else {
            svix_ksuid::Ksuid::from_base62(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?
          };
          Ok(Ksuid { inner })
        }
        Either::B(b) => {
          let slice: &[u8] = b.as_ref();
          if slice.len() != 20 {
            return Err(Error::new(
              Status::InvalidArg,
              format!("Expected 20 bytes for Ksuid, got {}", slice.len()),
            ));
          }
          let mut arr = [0u8; 20];
          arr.copy_from_slice(slice);
          Ok(Ksuid {
            inner: RawKsuid::from_bytes(arr),
          })
        }
      }
    } else {
      Ok(Ksuid {
        inner: RawKsuid::now(None),
      })
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Buffer>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|b| b.as_ref());
    Ok(Ksuid {
      inner: RawKsuid::now(payload_ref),
    })
  }

  #[napi(factory)]
  pub fn from_seconds(seconds: Option<i64>, payload: Option<Buffer>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|b| b.as_ref());
    Ok(Ksuid {
      inner: RawKsuid::from_seconds(seconds, payload_ref),
    })
  }

  #[napi(factory)]
  pub fn create(seconds: Option<i64>, payload: Option<Buffer>) -> Result<Self> {
    Self::from_seconds(seconds, payload)
  }

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = RawKsuid::from_base62(&base62).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
    Ok(Ksuid { inner })
  }

  #[napi(factory)]
  pub fn from_str(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(factory)]
  pub fn from_crockford_base32(encoded: String, alphabet: Option<String>) -> Result<Self> {
    let alph = parse_alphabet(alphabet)?;
    let vec = decode_crockford(&encoded, &alph)?;
    if vec.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes, got {}", vec.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&vec);
    Ok(Ksuid {
      inner: RawKsuid::from_bytes(arr),
    })
  }

  #[napi(factory)]
  pub fn from_base32(encoded: String, alphabet: Option<String>) -> Result<Self> {
    Self::from_crockford_base32(encoded, alphabet)
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Buffer) -> Result<Self> {
    let slice: &[u8] = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    Ok(Ksuid {
      inner: RawKsuid::from_bytes(arr),
    })
  }

  #[napi]
  pub fn is_valid(base62: String) -> bool {
    RawKsuid::from_base62(&base62).is_ok()
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    let alph = parse_alphabet(alphabet)?;
    Ok(encode_crockford(self.inner.bytes().as_ref(), &alph))
  }

  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> Result<String> {
    self.to_crockford_base32(alphabet)
  }

  #[napi]
  pub fn to_string(&self, options: Option<Either<String, ToStringOptions>>) -> Result<String> {
    if let Some(opts) = options {
      match opts {
        Either::A(enc) => {
          if enc == "base32" || enc == "crockford" {
            self.to_crockford_base32(None)
          } else {
            Ok(self.inner.to_string())
          }
        }
        Either::B(opt_obj) => {
          let enc = opt_obj.enc.as_deref().unwrap_or("base62");
          if enc == "base32" || enc == "crockford" {
            self.to_crockford_base32(opt_obj.alphabet)
          } else {
            Ok(self.inner.to_string())
          }
        }
      }
    } else {
      Ok(self.inner.to_string())
    }
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(self.inner.bytes().as_ref())
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    Buffer::from(self.inner.payload().as_ref())
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds()
  }

  #[napi]
  pub fn compare(&self, other: &Ksuid) -> i32 {
    if self.inner < other.inner {
      -1
    } else if self.inner > other.inner {
      1
    } else {
      0
    }
  }

  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.inner == other.inner
  }
}

#[napi]
pub struct KsuidMs {
  inner: RawKsuidMs,
}

#[napi]
impl KsuidMs {
  #[napi(getter)]
  pub fn payload_bytes() -> u32 {
    15
  }

  #[napi(getter)]
  pub fn bytes_len() -> u32 {
    20
  }

  #[napi(constructor)]
  pub fn new(input: Option<Either<String, Buffer>>) -> Result<Self> {
    if let Some(val) = input {
      match val {
        Either::A(s) => {
          let inner = if s.len() == 32 {
            let bytes = decode_crockford(&s, DEFAULT_CROCKFORD_ALPHABET)?;
            if bytes.len() != 20 {
              return Err(Error::new(
                Status::InvalidArg,
                format!("Expected 20 bytes from Crockford Base32, got {}", bytes.len()),
              ));
            }
            let mut arr = [0u8; 20];
            arr.copy_from_slice(&bytes);
            RawKsuidMs::from_bytes(arr)
          } else {
            svix_ksuid::KsuidMs::from_base62(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?
          };
          Ok(KsuidMs { inner })
        }
        Either::B(b) => {
          let slice: &[u8] = b.as_ref();
          if slice.len() != 20 {
            return Err(Error::new(
              Status::InvalidArg,
              format!("Expected 20 bytes for KsuidMs, got {}", slice.len()),
            ));
          }
          let mut arr = [0u8; 20];
          arr.copy_from_slice(slice);
          Ok(KsuidMs {
            inner: RawKsuidMs::from_bytes(arr),
          })
        }
      }
    } else {
      Ok(KsuidMs {
        inner: RawKsuidMs::now(None),
      })
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Buffer>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|b| b.as_ref());
    Ok(KsuidMs {
      inner: RawKsuidMs::now(payload_ref),
    })
  }

  #[napi(factory)]
  pub fn from_millis(ms: Option<i64>, payload: Option<Buffer>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|b| b.as_ref());
    Ok(KsuidMs {
      inner: RawKsuidMs::from_millis(ms, payload_ref),
    })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = RawKsuidMs::from_base62(&base62).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
    Ok(KsuidMs { inner })
  }

  #[napi(factory)]
  pub fn from_str(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(factory)]
  pub fn from_crockford_base32(encoded: String, alphabet: Option<String>) -> Result<Self> {
    let alph = parse_alphabet(alphabet)?;
    let vec = decode_crockford(&encoded, &alph)?;
    if vec.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes, got {}", vec.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&vec);
    Ok(KsuidMs {
      inner: RawKsuidMs::from_bytes(arr),
    })
  }

  #[napi(factory)]
  pub fn from_base32(encoded: String, alphabet: Option<String>) -> Result<Self> {
    Self::from_crockford_base32(encoded, alphabet)
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Buffer) -> Result<Self> {
    let slice: &[u8] = bytes.as_ref();
    if slice.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Expected 20 bytes, got {}", slice.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(slice);
    Ok(KsuidMs {
      inner: RawKsuidMs::from_bytes(arr),
    })
  }

  #[napi]
  pub fn is_valid(base62: String) -> bool {
    RawKsuidMs::from_base62(&base62).is_ok()
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    let alph = parse_alphabet(alphabet)?;
    Ok(encode_crockford(self.inner.bytes().as_ref(), &alph))
  }

  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> Result<String> {
    self.to_crockford_base32(alphabet)
  }

  #[napi]
  pub fn to_string(&self, options: Option<Either<String, ToStringOptions>>) -> Result<String> {
    if let Some(opts) = options {
      match opts {
        Either::A(enc) => {
          if enc == "base32" || enc == "crockford" {
            self.to_crockford_base32(None)
          } else {
            Ok(self.inner.to_string())
          }
        }
        Either::B(opt_obj) => {
          let enc = opt_obj.enc.as_deref().unwrap_or("base62");
          if enc == "base32" || enc == "crockford" {
            self.to_crockford_base32(opt_obj.alphabet)
          } else {
            Ok(self.inner.to_string())
          }
        }
      }
    } else {
      Ok(self.inner.to_string())
    }
  }

  #[napi]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(self.inner.bytes().as_ref())
  }

  #[napi]
  pub fn payload(&self) -> Buffer {
    Buffer::from(self.inner.payload().as_ref())
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds()
  }

  #[napi]
  pub fn compare(&self, other: &KsuidMs) -> i32 {
    if self.inner < other.inner {
      -1
    } else if self.inner > other.inner {
      1
    } else {
      0
    }
  }

  #[napi]
  pub fn equals(&self, other: &KsuidMs) -> bool {
    self.inner == other.inner
  }
}
