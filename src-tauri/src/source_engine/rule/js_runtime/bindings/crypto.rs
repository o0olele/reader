fn nested_rule_values(
    rule: &str,
    input: &str,
    want: Extraction,
    variables: &Arc<Mutex<HashMap<String, String>>>,
    http: Option<JsHttpContext>,
) -> Result<Vec<String>, String> {
    let snapshot = variables
        .lock()
        .map(|values| values.clone())
        .unwrap_or_default();
    let mut context = RuleContext::new(snapshot);
    if let Some(http) = http {
        context.with_http(http);
    }
    let values = evaluate(rule, input, want, &mut context).map_err(|error| error.to_string())?;
    if let Ok(mut shared) = variables.lock() {
        shared.extend(context.snapshot());
    }
    Ok(values)
}

fn rule_js_error(error: String) -> rquickjs::Error {
    rquickjs::Error::new_from_js_message("Rule", "String", error)
}

fn stable_android_id(base_url: &str) -> String {
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

fn normalize_chapter_numbers(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut chinese = String::new();
    let flush = |output: &mut String, chinese: &mut String| {
        if chinese.is_empty() {
            return;
        }
        if let Some(number) = chinese_number(chinese) {
            output.push_str(&number.to_string());
        } else {
            output.push_str(chinese);
        }
        chinese.clear();
    };
    for character in value.chars() {
        if chinese_digit(character).is_some() || chinese_unit(character).is_some() {
            chinese.push(character);
        } else {
            flush(&mut output, &mut chinese);
            output.push(character);
        }
    }
    flush(&mut output, &mut chinese);
    output
}

fn chinese_number(value: &str) -> Option<u64> {
    let has_unit = value
        .chars()
        .any(|character| chinese_unit(character).is_some());
    if !has_unit {
        return value.chars().try_fold(0_u64, |number, character| {
            Some(number * 10 + chinese_digit(character)?)
        });
    }
    let mut total = 0_u64;
    let mut section = 0_u64;
    let mut digit = 0_u64;
    for character in value.chars() {
        if let Some(value) = chinese_digit(character) {
            digit = value;
            continue;
        }
        let unit = chinese_unit(character)?;
        if unit == 10_000 {
            section = (section + digit) * unit;
            total += section;
            section = 0;
        } else {
            section += digit.max(1) * unit;
        }
        digit = 0;
    }
    Some(total + section + digit)
}

fn chinese_digit(character: char) -> Option<u64> {
    match character {
        '零' | '〇' => Some(0),
        '一' => Some(1),
        '二' | '两' => Some(2),
        '三' => Some(3),
        '四' => Some(4),
        '五' => Some(5),
        '六' => Some(6),
        '七' => Some(7),
        '八' => Some(8),
        '九' => Some(9),
        _ => None,
    }
}

fn chinese_unit(character: char) -> Option<u64> {
    match character {
        '十' => Some(10),
        '百' => Some(100),
        '千' => Some(1_000),
        '万' => Some(10_000),
        _ => None,
    }
}

