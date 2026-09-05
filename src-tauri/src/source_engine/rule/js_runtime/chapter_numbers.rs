

pub(super) fn normalize_chapter_numbers(value: &str) -> String {
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
