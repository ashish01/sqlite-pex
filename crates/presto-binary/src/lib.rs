use presto_core::{FunctionRegistry, PrestoResult};

pub mod encoding {
    use base64::Engine;
    use presto_core::{PrestoError, PrestoResult};

    pub fn to_hex(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }

    pub fn from_hex(input: &str) -> PrestoResult<Vec<u8>> {
        hex::decode(input).map_err(|_| PrestoError::InvalidArgument("invalid hex input"))
    }

    pub fn to_base64(bytes: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    pub fn from_base64(input: &str) -> PrestoResult<Vec<u8>> {
        base64::engine::general_purpose::STANDARD
            .decode(input)
            .map_err(|_| PrestoError::InvalidArgument("invalid base64 input"))
    }

    pub fn to_base64url(bytes: &[u8]) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    pub fn from_base64url(input: &str) -> PrestoResult<Vec<u8>> {
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(input)
            .map_err(|_| PrestoError::InvalidArgument("invalid base64url input"))
    }

    pub fn to_base32(bytes: &[u8]) -> String {
        data_encoding::BASE32.encode(bytes)
    }

    pub fn from_base32(input: &str) -> PrestoResult<Vec<u8>> {
        data_encoding::BASE32
            .decode(input.as_bytes())
            .map_err(|_| PrestoError::InvalidArgument("invalid base32 input"))
    }
}

pub mod blob {
    use presto_core::{PrestoError, PrestoResult};

    pub fn concat(parts: &[&[u8]]) -> Vec<u8> {
        let len = parts.iter().map(|p| p.len()).sum();
        let mut out = Vec::with_capacity(len);
        for part in parts {
            out.extend_from_slice(part);
        }
        out
    }

    pub fn length(bytes: &[u8]) -> i64 {
        bytes.len() as i64
    }

    pub fn reverse(bytes: &[u8]) -> Vec<u8> {
        let mut out = bytes.to_vec();
        out.reverse();
        out
    }

    pub fn lpad(input: &[u8], size: i64, pad: &[u8]) -> PrestoResult<Vec<u8>> {
        if size < 0 {
            return Err(PrestoError::InvalidArgument("size must be >= 0"));
        }
        if pad.is_empty() && size as usize > input.len() {
            return Err(PrestoError::InvalidArgument(
                "pad binary must not be empty when extending",
            ));
        }

        let size = size as usize;
        if size <= input.len() {
            return Ok(input[..size].to_vec());
        }

        let fill = size - input.len();
        let mut out = Vec::with_capacity(size);
        out.extend(pad.iter().cycle().take(fill).copied());
        out.extend_from_slice(input);
        Ok(out)
    }

    pub fn rpad(input: &[u8], size: i64, pad: &[u8]) -> PrestoResult<Vec<u8>> {
        if size < 0 {
            return Err(PrestoError::InvalidArgument("size must be >= 0"));
        }
        if pad.is_empty() && size as usize > input.len() {
            return Err(PrestoError::InvalidArgument(
                "pad binary must not be empty when extending",
            ));
        }

        let size = size as usize;
        if size <= input.len() {
            return Ok(input[..size].to_vec());
        }

        let fill = size - input.len();
        let mut out = input.to_vec();
        out.extend(pad.iter().cycle().take(fill).copied());
        Ok(out)
    }

    pub fn substr(input: &[u8], start: i64, length: Option<i64>) -> PrestoResult<Vec<u8>> {
        let len = input.len() as i64;
        if start == 0 {
            return Err(PrestoError::InvalidArgument(
                "start index is 1-based and cannot be 0",
            ));
        }

        let start_idx = if start > 0 {
            start - 1
        } else {
            (len + start).max(0)
        } as usize;

        if start_idx >= input.len() {
            return Ok(Vec::new());
        }

        let end_idx = match length {
            None => input.len(),
            Some(n) if n <= 0 => return Ok(Vec::new()),
            Some(n) => (start_idx + n as usize).min(input.len()),
        };

        Ok(input[start_idx..end_idx].to_vec())
    }

    fn read_bytes<const N: usize>(binary: &[u8], err_msg: &'static str) -> PrestoResult<[u8; N]> {
        binary
            .try_into()
            .map_err(|_| PrestoError::InvalidArgument(err_msg))
    }

    pub fn from_big_endian_32(binary: &[u8]) -> PrestoResult<i32> {
        Ok(i32::from_be_bytes(read_bytes::<4>(
            binary,
            "from_big_endian_32 expects 4 bytes",
        )?))
    }

    pub fn from_big_endian_64(binary: &[u8]) -> PrestoResult<i64> {
        Ok(i64::from_be_bytes(read_bytes::<8>(
            binary,
            "from_big_endian_64 expects 8 bytes",
        )?))
    }

    pub fn to_big_endian_32(value: i32) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    pub fn to_big_endian_64(value: i64) -> Vec<u8> {
        value.to_be_bytes().to_vec()
    }

    pub fn from_ieee754_32(binary: &[u8]) -> PrestoResult<f32> {
        Ok(f32::from_bits(u32::from_be_bytes(read_bytes::<4>(
            binary,
            "from_ieee754_32 expects 4 bytes",
        )?)))
    }

    pub fn from_ieee754_64(binary: &[u8]) -> PrestoResult<f64> {
        Ok(f64::from_bits(u64::from_be_bytes(read_bytes::<8>(
            binary,
            "from_ieee754_64 expects 8 bytes",
        )?)))
    }

    pub fn to_ieee754_32(value: f32) -> Vec<u8> {
        value.to_bits().to_be_bytes().to_vec()
    }

    pub fn to_ieee754_64(value: f64) -> Vec<u8> {
        value.to_bits().to_be_bytes().to_vec()
    }

    pub fn crc32(binary: &[u8]) -> i64 {
        i64::from(crc32fast::hash(binary))
    }
}

pub mod hash {
    use std::hash::Hasher;

    use hmac::{Hmac, Mac};
    use sha1::Sha1;
    use sha2::{Digest, Sha256, Sha512};

    use presto_core::{PrestoError, PrestoResult};

    macro_rules! define_hmac {
        ($fn_name:ident, $hash_type:ty) => {
            pub fn $fn_name(binary: &[u8], key: &[u8]) -> PrestoResult<Vec<u8>> {
                let mut mac = Hmac::<$hash_type>::new_from_slice(key)
                    .map_err(|_| PrestoError::InvalidArgument("invalid HMAC key"))?;
                mac.update(binary);
                Ok(mac.finalize().into_bytes().to_vec())
            }
        };
    }

    pub fn md5(binary: &[u8]) -> Vec<u8> {
        md5::Md5::digest(binary).to_vec()
    }

    pub fn sha1(binary: &[u8]) -> Vec<u8> {
        Sha1::digest(binary).to_vec()
    }

    pub fn sha256(binary: &[u8]) -> Vec<u8> {
        Sha256::digest(binary).to_vec()
    }

    pub fn sha512(binary: &[u8]) -> Vec<u8> {
        Sha512::digest(binary).to_vec()
    }

    define_hmac!(hmac_md5, md5::Md5);
    define_hmac!(hmac_sha1, Sha1);
    define_hmac!(hmac_sha256, Sha256);
    define_hmac!(hmac_sha512, Sha512);

    pub fn murmur3_x64_128(binary: &[u8]) -> PrestoResult<Vec<u8>> {
        let mut cursor = std::io::Cursor::new(binary);
        let hash = murmur3::murmur3_x64_128(&mut cursor, 0)
            .map_err(|_| PrestoError::Internal("murmur3 hashing failed".into()))?;
        Ok(hash.to_be_bytes().to_vec())
    }

    pub fn xxhash64(binary: &[u8], seed: Option<i64>) -> Vec<u8> {
        let mut hasher = twox_hash::XxHash64::with_seed(seed.unwrap_or(0) as u64);
        hasher.write(binary);
        hasher.finish().to_be_bytes().to_vec()
    }

    pub fn spooky_hash_v2_32(binary: &[u8]) -> Vec<u8> {
        let value = hashers::jenkins::spooky_hash::spooky(binary) as u32;
        value.to_be_bytes().to_vec()
    }

    pub fn spooky_hash_v2_64(binary: &[u8]) -> Vec<u8> {
        let value = hashers::jenkins::spooky_hash::spooky(binary);
        value.to_be_bytes().to_vec()
    }
}

pub fn register(registry: &mut FunctionRegistry) -> PrestoResult<()> {
    let scalar_functions = [
        "concat",
        "crc32",
        "from_base32",
        "from_base64",
        "from_base64url",
        "from_big_endian_32",
        "from_big_endian_64",
        "from_hex",
        "from_ieee754_32",
        "from_ieee754_64",
        "hmac_md5",
        "hmac_sha1",
        "hmac_sha256",
        "hmac_sha512",
        "length",
        "lpad",
        "md5",
        "murmur3_x64_128",
        "reverse",
        "rpad",
        "sha1",
        "sha256",
        "sha512",
        "spooky_hash_v2_32",
        "spooky_hash_v2_64",
        "substr",
        "to_base32",
        "to_base64",
        "to_base64url",
        "to_big_endian_32",
        "to_big_endian_64",
        "to_hex",
        "to_ieee754_32",
        "to_ieee754_64",
        "xxhash64",
    ];

    for name in scalar_functions {
        registry.register_scalar(name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{blob, encoding, hash};

    #[test]
    fn encoding_round_trips() {
        let bytes = b"hello";
        let hex = encoding::to_hex(bytes);
        assert_eq!(encoding::from_hex(&hex).unwrap(), bytes);

        let b64 = encoding::to_base64(bytes);
        assert_eq!(encoding::from_base64(&b64).unwrap(), bytes);

        let b32 = encoding::to_base32(bytes);
        assert_eq!(encoding::from_base32(&b32).unwrap(), bytes);
    }

    #[test]
    fn blob_operations_work() {
        assert_eq!(blob::concat(&[b"a", b"bc"]), b"abc");
        assert_eq!(blob::reverse(b"abcd"), b"dcba");
        assert_eq!(blob::length(b"abc"), 3);
        assert_eq!(blob::substr(b"abcdef", 2, Some(3)).unwrap(), b"bcd");
    }

    #[test]
    fn endian_and_float_helpers_work() {
        let be = blob::to_big_endian_32(42);
        assert_eq!(blob::from_big_endian_32(&be).unwrap(), 42);

        let f = blob::to_ieee754_64(1.25);
        assert_eq!(blob::from_ieee754_64(&f).unwrap(), 1.25);
    }

    #[test]
    fn hash_functions_return_expected_lengths() {
        let bytes = b"abc";
        assert_eq!(hash::md5(bytes).len(), 16);
        assert_eq!(hash::sha1(bytes).len(), 20);
        assert_eq!(hash::sha256(bytes).len(), 32);
        assert_eq!(hash::sha512(bytes).len(), 64);
        assert_eq!(hash::hmac_sha256(bytes, b"key").unwrap().len(), 32);
        assert_eq!(hash::murmur3_x64_128(bytes).unwrap().len(), 16);
        assert_eq!(hash::xxhash64(bytes, None).len(), 8);
    }
}
