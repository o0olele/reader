use super::super::js_error;
use crate::error::AppError;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use rquickjs::{Ctx, Function, Object};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use uuid::Uuid;

pub(super) fn install<'js>(ctx: Ctx<'js>, java: &Object<'js>) -> Result<(), AppError> {
    java.set(
        "md5Encode",
        Function::new(ctx.clone(), |value: String| {
            let mut digest = Md5::new();
            digest.update(value.as_bytes());
            format!("{:x}", digest.finalize())
        }),
    )
    .map_err(js_error)?;
    java.set(
        "digestHex",
        Function::new(ctx.clone(), |data: String, algorithm: String| {
            digest_bytes(&algorithm, data.as_bytes())
                .map(|bytes| bytes_to_hex(&bytes))
                .map_err(|error| rquickjs::Error::new_from_js_message("Digest", "String", error))
        }),
    )
    .map_err(js_error)?;
    java.set(
        "HMacHex",
        Function::new(
            ctx.clone(),
            |data: String, algorithm: String, key: String| {
                hmac_bytes(&algorithm, key.as_bytes(), data.as_bytes())
                    .map(|bytes| bytes_to_hex(&bytes))
                    .map_err(|error| rquickjs::Error::new_from_js_message("HMac", "String", error))
            },
        ),
    )
    .map_err(js_error)?;
    java.set(
        "HMacBase64",
        Function::new(
            ctx.clone(),
            |data: String, algorithm: String, key: String| {
                hmac_bytes(&algorithm, key.as_bytes(), data.as_bytes())
                    .map(|bytes| STANDARD.encode(bytes))
                    .map_err(|error| rquickjs::Error::new_from_js_message("HMac", "String", error))
            },
        ),
    )
    .map_err(js_error)?;
    java.set(
        "randomUUID",
        Function::new(ctx.clone(), || Uuid::new_v4().to_string()),
    )
    .map_err(js_error)?;
    Ok(())
}

pub(super) fn stable_android_id(base_url: &str) -> String {
    let mut digest = Md5::new();
    digest.update(b"reader-desktop/android-id/");
    digest.update(base_url.as_bytes());
    format!("{:x}", digest.finalize())[..16].to_owned()
}

fn canonical_algorithm(algorithm: &str) -> String {
    algorithm
        .trim()
        .to_ascii_uppercase()
        .replace(['-', '_', '/'], "")
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn digest_bytes(algorithm: &str, data: &[u8]) -> Result<Vec<u8>, String> {
    match canonical_algorithm(algorithm).as_str() {
        "MD5" => Ok(Md5::digest(data).to_vec()),
        "SHA1" => Ok(Sha1::digest(data).to_vec()),
        "SHA256" => Ok(Sha256::digest(data).to_vec()),
        "SHA512" => Ok(Sha512::digest(data).to_vec()),
        other => Err(format!("unsupported digest algorithm: {other}")),
    }
}

fn hmac_bytes(algorithm: &str, key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let normalized = canonical_algorithm(algorithm);
    let digest_name = normalized.strip_prefix("HMAC").unwrap_or(&normalized);
    match digest_name {
        "MD5" => {
            let mut mac = Hmac::<Md5>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA1" => {
            let mut mac = Hmac::<Sha1>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA256" => {
            let mut mac = Hmac::<Sha256>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA512" => {
            let mut mac = Hmac::<Sha512>::new_from_slice(key)
                .map_err(|error| format!("invalid HMAC key: {error}"))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        other => Err(format!("unsupported HMAC algorithm: {other}")),
    }
}
