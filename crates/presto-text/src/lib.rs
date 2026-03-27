use presto_core::{FunctionRegistry, PrestoResult};

pub mod string {
    use std::collections::BTreeMap;

    use rust_stemmers::{Algorithm, Stemmer};
    use strsim::{jaro_winkler, levenshtein};
    use unicode_normalization::UnicodeNormalization;

    use presto_core::{PrestoError, PrestoResult};

    fn chars_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    pub fn bit_length(input: &str) -> i64 {
        (input.len() * 8) as i64
    }

    pub fn chr(n: i64) -> PrestoResult<String> {
        let n = u32::try_from(n).map_err(|_| PrestoError::InvalidArgument("invalid codepoint"))?;
        let ch = char::from_u32(n).ok_or(PrestoError::InvalidArgument("invalid codepoint"))?;
        Ok(ch.to_string())
    }

    pub fn codepoint(input: &str) -> PrestoResult<i64> {
        let mut chars = input.chars();
        let first = chars
            .next()
            .ok_or(PrestoError::InvalidArgument("input must not be empty"))?;
        if chars.next().is_some() {
            return Err(PrestoError::InvalidArgument(
                "input to codepoint must contain exactly one character",
            ));
        }
        Ok(first as i64)
    }

    pub fn concat(parts: &[&str]) -> String {
        parts.concat()
    }

    pub fn ends_with(input: &str, suffix: &str) -> bool {
        input.ends_with(suffix)
    }

    pub fn starts_with(input: &str, prefix: &str) -> bool {
        input.starts_with(prefix)
    }

    pub fn from_utf8(binary: &[u8]) -> PrestoResult<String> {
        String::from_utf8(binary.to_vec())
            .map_err(|_| PrestoError::InvalidArgument("invalid UTF-8 input"))
    }

    pub fn from_utf8_replace(binary: &[u8], replace: &str) -> String {
        match std::str::from_utf8(binary) {
            Ok(s) => s.to_string(),
            Err(_) => {
                let mut out = String::new();
                for chunk in binary.utf8_chunks() {
                    out.push_str(chunk.valid());
                    if !chunk.invalid().is_empty() {
                        out.push_str(replace);
                    }
                }
                out
            }
        }
    }

    pub fn hamming_distance(left: &str, right: &str) -> PrestoResult<i64> {
        let l = chars_vec(left);
        let r = chars_vec(right);
        if l.len() != r.len() {
            return Err(PrestoError::InvalidArgument(
                "hamming_distance requires equal-length strings",
            ));
        }
        Ok(l.iter().zip(&r).filter(|(a, b)| a != b).count() as i64)
    }

    pub fn jarowinkler_similarity(left: &str, right: &str) -> f64 {
        jaro_winkler(left, right)
    }

    pub fn key_sampling_percent(input: &str) -> f64 {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash64::with_seed(0);
        hasher.write(input.as_bytes());
        (hasher.finish() as f64) / (u64::MAX as f64)
    }

    pub fn length(input: &str) -> i64 {
        input.chars().count() as i64
    }

    pub fn levenshtein_distance(left: &str, right: &str) -> i64 {
        levenshtein(left, right) as i64
    }

    pub fn longest_common_prefix(left: &str, right: &str) -> String {
        left.chars()
            .zip(right.chars())
            .take_while(|(a, b)| a == b)
            .map(|(c, _)| c)
            .collect()
    }

    pub fn lower(input: &str) -> String {
        input.to_lowercase()
    }

    pub fn upper(input: &str) -> String {
        input.to_uppercase()
    }

    pub fn lpad(input: &str, size: i64, pad: &str) -> PrestoResult<String> {
        if size < 0 {
            return Err(PrestoError::InvalidArgument("size must be >= 0"));
        }
        let size = size as usize;
        let input_chars = chars_vec(input);
        if size <= input_chars.len() {
            return Ok(input_chars.into_iter().take(size).collect());
        }

        let pad_chars = chars_vec(pad);
        if pad_chars.is_empty() {
            return Err(PrestoError::InvalidArgument("padstring must not be empty"));
        }

        let fill = size - input_chars.len();
        let mut out: Vec<char> = pad_chars.iter().cycle().take(fill).copied().collect();
        out.extend(input_chars);
        Ok(out.into_iter().collect())
    }

    pub fn rpad(input: &str, size: i64, pad: &str) -> PrestoResult<String> {
        if size < 0 {
            return Err(PrestoError::InvalidArgument("size must be >= 0"));
        }
        let size = size as usize;
        let mut out = chars_vec(input);
        if size <= out.len() {
            return Ok(out.into_iter().take(size).collect());
        }

        let pad_chars = chars_vec(pad);
        if pad_chars.is_empty() {
            return Err(PrestoError::InvalidArgument("padstring must not be empty"));
        }

        let fill = size - out.len();
        out.extend(pad_chars.iter().cycle().take(fill).copied());
        Ok(out.into_iter().collect())
    }

    pub fn ltrim(input: &str, chars: Option<&str>) -> String {
        match chars {
            None => input.trim_start().to_string(),
            Some(chars) => input.trim_start_matches(|c| chars.contains(c)).to_string(),
        }
    }

    pub fn rtrim(input: &str, chars: Option<&str>) -> String {
        match chars {
            None => input.trim_end().to_string(),
            Some(chars) => input.trim_end_matches(|c| chars.contains(c)).to_string(),
        }
    }

    pub fn trim(input: &str, chars: Option<&str>) -> String {
        match chars {
            None => input.trim().to_string(),
            Some(chars) => input.trim_matches(|c| chars.contains(c)).to_string(),
        }
    }

    pub fn normalize(input: &str, form: Option<&str>) -> PrestoResult<String> {
        Ok(match form.unwrap_or("NFC").to_uppercase().as_str() {
            "NFC" => input.nfc().collect(),
            "NFD" => input.nfd().collect(),
            "NFKC" => input.nfkc().collect(),
            "NFKD" => input.nfkd().collect(),
            _ => {
                return Err(PrestoError::InvalidArgument(
                    "normalize form must be one of NFC,NFD,NFKC,NFKD",
                ));
            }
        })
    }

    pub fn replace(input: &str, search: &str, replacement: Option<&str>) -> String {
        input.replace(search, replacement.unwrap_or(""))
    }

    pub fn replace_first(input: &str, search: &str, replacement: &str) -> String {
        input.replacen(search, replacement, 1)
    }

    pub fn reverse(input: &str) -> String {
        input.chars().rev().collect()
    }

    pub fn split(input: &str, delimiter: &str, limit: Option<i64>) -> PrestoResult<Vec<String>> {
        if delimiter.is_empty() {
            return Err(PrestoError::InvalidArgument("delimiter must not be empty"));
        }
        let out = match limit {
            None => input.split(delimiter).map(ToOwned::to_owned).collect(),
            Some(l) if l <= 0 => {
                return Err(PrestoError::InvalidArgument("limit must be > 0"));
            }
            Some(l) => input
                .splitn(l as usize, delimiter)
                .map(ToOwned::to_owned)
                .collect(),
        };
        Ok(out)
    }

    pub fn split_part(input: &str, delimiter: &str, index: i64) -> PrestoResult<Option<String>> {
        if delimiter.is_empty() {
            return Err(PrestoError::InvalidArgument("delimiter must not be empty"));
        }
        if index <= 0 {
            return Err(PrestoError::InvalidArgument("index must be >= 1"));
        }
        Ok(input
            .split(delimiter)
            .nth((index - 1) as usize)
            .map(ToOwned::to_owned))
    }

    pub fn split_to_multimap(
        input: &str,
        entry_delimiter: &str,
        key_value_delimiter: &str,
    ) -> PrestoResult<BTreeMap<String, Vec<String>>> {
        if entry_delimiter.is_empty() || key_value_delimiter.is_empty() {
            return Err(PrestoError::InvalidArgument("delimiters must not be empty"));
        }

        let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for entry in input.split(entry_delimiter) {
            let mut parts = entry.splitn(2, key_value_delimiter);
            let key = parts.next().unwrap_or("").to_string();
            let value = parts.next().unwrap_or("").to_string();
            map.entry(key).or_default().push(value);
        }
        Ok(map)
    }

    pub fn strpos(input: &str, substring: &str, instance: Option<i64>) -> PrestoResult<i64> {
        if substring.is_empty() {
            return Ok(1);
        }
        let instance = instance.unwrap_or(1);
        if instance <= 0 {
            return Err(PrestoError::InvalidArgument("instance must be >= 1"));
        }

        let mut start = 0usize;
        let mut found = 0;
        while let Some(idx) = input[start..].find(substring) {
            found += 1;
            let abs = start + idx;
            if found == instance {
                return Ok((input[..abs].chars().count() as i64) + 1);
            }
            start = abs + substring.len();
        }
        Ok(0)
    }

    pub fn strrpos(input: &str, substring: &str, instance: Option<i64>) -> PrestoResult<i64> {
        if substring.is_empty() {
            return Ok(length(input) + 1);
        }
        let instance = instance.unwrap_or(1);
        if instance <= 0 {
            return Err(PrestoError::InvalidArgument("instance must be >= 1"));
        }

        let positions: Vec<usize> = input.match_indices(substring).map(|(i, _)| i).collect();
        if positions.is_empty() || instance as usize > positions.len() {
            return Ok(0);
        }
        let idx = positions[positions.len() - instance as usize];
        Ok((input[..idx].chars().count() as i64) + 1)
    }

    pub fn substr(input: &str, start: i64, length: Option<i64>) -> PrestoResult<String> {
        if start == 0 {
            return Err(PrestoError::InvalidArgument(
                "start index is 1-based and cannot be 0",
            ));
        }
        let chars = chars_vec(input);
        let len = chars.len() as i64;
        let start_idx = if start > 0 {
            start - 1
        } else {
            (len + start).max(0)
        } as usize;

        if start_idx >= chars.len() {
            return Ok(String::new());
        }

        let end_idx = match length {
            None => chars.len(),
            Some(n) if n <= 0 => return Ok(String::new()),
            Some(n) => (start_idx + n as usize).min(chars.len()),
        };

        Ok(chars[start_idx..end_idx].iter().collect())
    }

    pub fn to_utf8(input: &str) -> Vec<u8> {
        input.as_bytes().to_vec()
    }

    pub fn trail(input: &str, n: i64) -> PrestoResult<String> {
        if n < 0 {
            return Err(PrestoError::InvalidArgument("N must be >= 0"));
        }
        let chars = chars_vec(input);
        let n = n as usize;
        let start = chars.len().saturating_sub(n);
        Ok(chars[start..].iter().collect())
    }

    pub fn word_stem(word: &str, lang: Option<&str>) -> String {
        let algo = match lang.unwrap_or("english").to_lowercase().as_str() {
            "english" => Algorithm::English,
            "french" => Algorithm::French,
            "german" => Algorithm::German,
            "spanish" => Algorithm::Spanish,
            "italian" => Algorithm::Italian,
            "portuguese" => Algorithm::Portuguese,
            "russian" => Algorithm::Russian,
            _ => return word.to_string(),
        };
        Stemmer::create(algo).stem(word).to_string()
    }
}

pub mod regex {
    use regex::Regex;

    use presto_core::{PrestoError, PrestoResult};

    fn compile(pattern: &str) -> PrestoResult<Regex> {
        Regex::new(pattern).map_err(|_| PrestoError::InvalidArgument("invalid regex pattern"))
    }

    pub fn regexp_like(input: &str, pattern: &str) -> PrestoResult<bool> {
        Ok(compile(pattern)?.is_match(input))
    }

    pub fn regexp_extract(
        input: &str,
        pattern: &str,
        group: Option<usize>,
    ) -> PrestoResult<Option<String>> {
        let group = group.unwrap_or(0);
        Ok(compile(pattern)?
            .captures(input)
            .and_then(|caps| caps.get(group))
            .map(|m| m.as_str().to_string()))
    }

    pub fn regexp_extract_all(
        input: &str,
        pattern: &str,
        group: Option<usize>,
    ) -> PrestoResult<Vec<String>> {
        let group = group.unwrap_or(0);
        let regex = compile(pattern)?;
        Ok(regex
            .captures_iter(input)
            .filter_map(|caps| caps.get(group).map(|m| m.as_str().to_string()))
            .collect())
    }

    pub fn regexp_replace(input: &str, pattern: &str) -> PrestoResult<String> {
        let regex = compile(pattern)?;
        Ok(regex.replace_all(input, "").to_string())
    }

    pub fn regexp_replace_with(
        input: &str,
        pattern: &str,
        replacement: &str,
    ) -> PrestoResult<String> {
        let regex = compile(pattern)?;
        Ok(regex.replace_all(input, replacement).to_string())
    }

    pub fn regexp_split(input: &str, pattern: &str) -> PrestoResult<Vec<String>> {
        let regex = compile(pattern)?;
        Ok(regex.split(input).map(ToOwned::to_owned).collect())
    }
}

pub fn register(registry: &mut FunctionRegistry) -> PrestoResult<()> {
    let scalar_functions = [
        "bit_length",
        "chr",
        "codepoint",
        "concat",
        "ends_with",
        "from_utf8",
        "hamming_distance",
        "jarowinkler_similarity",
        "key_sampling_percent",
        "length",
        "levenshtein_distance",
        "longest_common_prefix",
        "lower",
        "lpad",
        "ltrim",
        "normalize",
        "replace",
        "replace_first",
        "reverse",
        "rpad",
        "rtrim",
        "split",
        "split_part",
        "split_to_multimap",
        "starts_with",
        "strpos",
        "strrpos",
        "substr",
        "to_utf8",
        "trail",
        "trim",
        "upper",
        "word_stem",
        "regexp_extract",
        "regexp_extract_all",
        "regexp_like",
        "regexp_replace",
        "regexp_split",
    ];

    for name in scalar_functions {
        registry.register_scalar(name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{regex, string};

    #[test]
    fn core_string_functions_work() {
        assert_eq!(string::length("héllo"), 5);
        assert_eq!(string::lower("HeLLo"), "hello");
        assert_eq!(string::upper("hello"), "HELLO");
        assert_eq!(string::substr("abcdef", 2, Some(3)).unwrap(), "bcd");
        assert_eq!(string::strpos("abcabc", "ab", None).unwrap(), 1);
        assert_eq!(string::strrpos("abcabc", "ab", None).unwrap(), 4);
    }

    #[test]
    fn split_and_trim_work() {
        assert_eq!(
            string::split("a,b,c", ",", None).unwrap(),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            string::split_part("a,b,c", ",", 2).unwrap(),
            Some("b".into())
        );
        assert_eq!(string::trim("  hi  ", None), "hi");
    }

    #[test]
    fn regex_functions_work() {
        assert!(regex::regexp_like("abc123", "[a-z]+\\d+").unwrap());
        assert_eq!(
            regex::regexp_extract("abc123", "([a-z]+)(\\d+)", Some(2)).unwrap(),
            Some("123".into())
        );
        assert_eq!(
            regex::regexp_replace_with("abc123", "\\d+", "X").unwrap(),
            "abcX"
        );
    }
}
