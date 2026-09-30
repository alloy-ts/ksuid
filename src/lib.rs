use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::{Ksuid as RawKsuid, KsuidMs as RawKsuidMs, KsuidLike};

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
          let inner = svix_ksuid::Ksuid::from_base62(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
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
  pub fn to_string(&self) -> String {
    self.inner.to_string()
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
          let inner = svix_ksuid::KsuidMs::from_base62(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
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
  pub fn to_string(&self) -> String {
    self.inner.to_string()
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
