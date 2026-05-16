//! Various serializers for dialogue storages.

use serde::{de::DeserializeOwned, ser::Serialize};

/// A serializer for memory storages.
pub trait Serializer<D> {
    type Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error>;
    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error>;
}

/// The JSON serializer for memory storages.
pub struct Json;

impl<D> Serializer<D> for Json
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_json::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_json::from_slice(data)
    }
}

/// The [CBOR] serializer for memory storages.
///
/// [CBOR]: https://en.wikipedia.org/wiki/CBOR
#[cfg(feature = "cbor-serializer")]
pub struct Cbor;

#[cfg(feature = "cbor-serializer")]
impl<D> Serializer<D> for Cbor
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_cbor::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_cbor::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_cbor::from_slice(data)
    }
}

/// The [Bincode] serializer for memory storages.
///
/// Can't serialize if the length is unknown, [see this issue](https://github.com/bincode-org/bincode/issues/167).
/// Use [`Cbor`] or [`Json`] if you are storing structs like [`Message`].
///
/// [Bincode]: https://github.com/servo/bincode
/// [`Cbor`]: crate::dispatching::dialogue::serializer::Cbor
/// [`Json`]: crate::dispatching::dialogue::serializer::Json
/// [`Message`]: crate::types::Message
#[cfg(feature = "bincode-serializer")]
pub struct Bincode;

/// Error type for the [`Bincode`] serializer covering both encoding and
/// decoding failures from `bincode-next` v3.
#[cfg(feature = "bincode-serializer")]
#[derive(Debug, thiserror::Error)]
pub enum BincodeError {
    #[error(transparent)]
    Encode(#[from] bincode_next::error::EncodeError),
    #[error(transparent)]
    Decode(#[from] bincode_next::error::DecodeError),
}

#[cfg(feature = "bincode-serializer")]
impl<D> Serializer<D> for Bincode
where
    D: Serialize + DeserializeOwned,
{
    type Error = BincodeError;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        let config = bincode_next::config::standard();
        bincode_next::serde::encode_to_vec(val, config).map_err(Into::into)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        let config = bincode_next::config::standard();
        bincode_next::serde::decode_from_slice(data, config).map(|(d, _)| d).map_err(Into::into)
    }
}
