//! Position filters used by Legado's private JSoup syntax.

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum PositionFilter {
    All,
    Include(Vec<Position>),
    Exclude(Vec<Position>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Position {
    Index(i32),
    Range {
        start: Option<i32>,
        end: Option<i32>,
        step: i32,
    },
}

/// Removes a trailing legacy (`.0:2` / `!0:-1`) or bracketed (`[0:3,-1]`)
/// position expression. CSS attribute selectors are left untouched.
pub(super) fn split_position(value: &str) -> (&str, PositionFilter) {
    let value = value.trim();
    if let Some(parsed) = parse_bracket(value) {
        return parsed;
    }
    parse_legacy(value).unwrap_or((value, PositionFilter::All))
}

pub(super) fn apply_positions<T>(items: Vec<T>, filter: &PositionFilter) -> Vec<T> {
    match filter {
        PositionFilter::All => items,
        PositionFilter::Include(positions) => {
            let indexes = resolve_positions(positions, items.len());
            let mut slots = items.into_iter().map(Some).collect::<Vec<_>>();
            indexes
                .into_iter()
                .filter_map(|index| slots[index].take())
                .collect()
        }
        PositionFilter::Exclude(positions) => {
            let indexes = resolve_positions(positions, items.len());
            let mut excluded = vec![false; items.len()];
            for index in indexes {
                excluded[index] = true;
            }
            items
                .into_iter()
                .enumerate()
                .filter_map(|(index, item)| (!excluded[index]).then_some(item))
                .collect()
        }
    }
}

fn parse_bracket(value: &str) -> Option<(&str, PositionFilter)> {
    let content = value.strip_suffix(']')?;
    let open = content.rfind('[')?;
    let mut expression = content[open + 1..].trim();
    let exclude = expression.starts_with('!');
    if exclude {
        expression = expression[1..].trim_start();
    }
    if expression.is_empty() {
        return None;
    }
    let positions = expression
        .split(',')
        .map(str::trim)
        .map(parse_position)
        .collect::<Option<Vec<_>>>()?;
    let before = content[..open].trim_end();
    Some((
        before,
        if exclude {
            PositionFilter::Exclude(positions)
        } else {
            PositionFilter::Include(positions)
        },
    ))
}

fn parse_position(value: &str) -> Option<Position> {
    if !value.contains(':') {
        return value.parse().ok().map(Position::Index);
    }
    let parts = value.split(':').collect::<Vec<_>>();
    if !(2..=3).contains(&parts.len()) {
        return None;
    }
    let optional = |part: &str| {
        if part.trim().is_empty() {
            Some(None)
        } else {
            part.trim().parse().ok().map(Some)
        }
    };
    Some(Position::Range {
        start: optional(parts[0])?,
        end: optional(parts[1])?,
        step: if parts.len() == 3 {
            parts[2].trim().parse().ok()?
        } else {
            1
        },
    })
}

fn parse_legacy(value: &str) -> Option<(&str, PositionFilter)> {
    if let Some(bang) = value.rfind('!') {
        if let Some(positions) = parse_index_list(&value[bang + 1..]) {
            return Some((
                value[..bang].trim_end_matches('.').trim_end(),
                PositionFilter::Exclude(positions),
            ));
        }
    }
    for (dot, _) in value.match_indices('.').rev() {
        if let Some(positions) = parse_index_list(&value[dot + 1..]) {
            let before = value[..dot].trim_end();
            return Some((before, PositionFilter::Include(positions)));
        }
    }
    None
}

fn parse_index_list(value: &str) -> Option<Vec<Position>> {
    let indexes = value
        .split(':')
        .map(str::trim)
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    (!indexes.is_empty()).then(|| indexes.into_iter().map(Position::Index).collect())
}

fn resolve_positions(positions: &[Position], length: usize) -> Vec<usize> {
    let mut indexes = Vec::new();
    for position in positions {
        match *position {
            Position::Index(index) => {
                if let Some(index) = resolve_index(index, length) {
                    push_unique(&mut indexes, index);
                }
            }
            Position::Range { start, end, step } => {
                append_range(&mut indexes, length, start, end, step)
            }
        }
    }
    indexes
}

fn resolve_index(index: i32, length: usize) -> Option<usize> {
    let resolved = if index < 0 {
        length.checked_sub(index.unsigned_abs() as usize)?
    } else {
        index as usize
    };
    (resolved < length).then_some(resolved)
}

fn append_range(
    indexes: &mut Vec<usize>,
    length: usize,
    start: Option<i32>,
    end: Option<i32>,
    raw_step: i32,
) {
    if length == 0 {
        return;
    }
    let length_i64 = length as i64;
    let normalize = |value: i32| {
        let value = i64::from(value);
        if value < 0 {
            value + length_i64
        } else {
            value
        }
    };
    let mut start = start.map_or(0, normalize);
    let mut end = end.map_or(length_i64 - 1, normalize);
    if (start < 0 && end < 0) || (start >= length_i64 && end >= length_i64) {
        return;
    }
    start = start.clamp(0, length_i64 - 1);
    end = end.clamp(0, length_i64 - 1);
    if start == end || i64::from(raw_step) >= length_i64 {
        push_unique(indexes, start as usize);
        return;
    }
    let step = if raw_step > 0 {
        i64::from(raw_step)
    } else if i64::from(raw_step).unsigned_abs() < length_i64 as u64 {
        i64::from(raw_step) + length_i64
    } else {
        1
    };
    let mut current = start;
    if start < end {
        while current <= end {
            push_unique(indexes, current as usize);
            current += step;
        }
    } else {
        while current >= end {
            push_unique(indexes, current as usize);
            current -= step;
        }
    }
}

fn push_unique(indexes: &mut Vec<usize>, index: usize) {
    if !indexes.contains(&index) {
        indexes.push(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filtered(raw: &str) -> Vec<i32> {
        let (_, filter) = split_position(raw);
        apply_positions((0..6).collect(), &filter)
    }

    #[test]
    fn parses_legacy_includes_and_excludes() {
        assert_eq!(filtered("tag.li.0:2:-1"), vec![0, 2, 5]);
        assert_eq!(filtered("tag.li!0:2:-1"), vec![1, 3, 4]);
        assert_eq!(filtered(".1:3"), vec![1, 3]);
        assert_eq!(split_position(".col-2 a").1, PositionFilter::All);
    }

    #[test]
    fn parses_bracket_lists_ranges_and_omitted_bounds() {
        assert_eq!(filtered("tag.li[1:3,-1]"), vec![1, 2, 3, 5]);
        assert_eq!(filtered("tag.li[:2]"), vec![0, 1, 2]);
        assert_eq!(filtered("tag.li[4:]"), vec![4, 5]);
        assert_eq!(filtered("tag.li[!1:3,-1]"), vec![0, 4]);
    }

    #[test]
    fn supports_reverse_ranges_and_negative_steps() {
        assert_eq!(filtered("tag.li[-1:0]"), vec![5, 4, 3, 2, 1, 0]);
        assert_eq!(filtered("tag.li[0:5:-4]"), vec![0, 2, 4]);
    }

    #[test]
    fn leaves_css_attribute_selectors_untouched() {
        assert_eq!(split_position("a[href]").1, PositionFilter::All);
        assert_eq!(split_position("li:nth-child(2)").1, PositionFilter::All);
    }
}
