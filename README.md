# sqlite-presto-ext

`sqlite-presto-ext` is a loadable SQLite extension that exposes a large MVP subset of Presto-style functions to SQLite.

## What it does (brief)

- Builds a SQLite extension library from Rust (`libsqlite_presto_ext.{so,dylib}`).
- Registers Presto-like functions into SQLite using the `p_<presto_function_name>` naming convention.
- Covers numeric, bitwise, text/regex, binary/hash, URL/IP, UUID, and partial window/vector function surfaces.

## Quick start

```bash
cargo build -p sqlite-presto-ext --release
sqlite3 :memory: ".load ./target/release/libsqlite_presto_ext" "select p_lower('HeLLo');"
```

> On Linux use `.so`; on macOS use `.dylib`.

## Presto function catalog and SQLite support

This section tracks the **full current Presto function list** from the official docs and maps each function to SQLite extension support status.

- `Supported`: exposed and bridged in this SQLite extension
- `Stubbed`: exposed as `p_<function>` but SQLite bridge is not implemented yet
- `No`: not currently exposed by this extension

> Note: if a function appears in multiple Presto categories, this document keeps a single row using its first category in the Presto functions index.

### Comparison Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`greatest`](https://prestodb.io/docs/current/functions/comparison.html#greatest-value1-value2-...-valueN-same-as-input) | No | Not currently exposed by this extension. |
| [`least`](https://prestodb.io/docs/current/functions/comparison.html#least-value1-value2-...-valueN-same-as-input) | No | Not currently exposed by this extension. |

### Conditional Expressions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`coalesce`](https://prestodb.io/docs/current/functions/conditional.html) | No | Not currently exposed by this extension. |
| [`if`](https://prestodb.io/docs/current/functions/conditional.html) | No | Not currently exposed by this extension. |
| [`nullif`](https://prestodb.io/docs/current/functions/conditional.html) | No | Not currently exposed by this extension. |
| [`try`](https://prestodb.io/docs/current/functions/conditional.html) | No | Not currently exposed by this extension. |

### Conversion Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`cast`](https://prestodb.io/docs/current/functions/conversion.html#cast-value-AS-type-type) | No | Not currently exposed by this extension. |
| [`parse_presto_data_size`](https://prestodb.io/docs/current/functions/conversion.html#parse_presto_data_size) | No | Not currently exposed by this extension. |
| [`try_cast`](https://prestodb.io/docs/current/functions/conversion.html#try_cast-value-AS-type-type) | No | Not currently exposed by this extension. |
| [`typeof`](https://prestodb.io/docs/current/functions/conversion.html#typeof-expr-varchar) | No | Not currently exposed by this extension. |

### Mathematical Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`abs`](https://prestodb.io/docs/current/functions/math.html#abs-x-same-as-input) | Supported | — |
| [`acos`](https://prestodb.io/docs/current/functions/math.html#acos-x-double) | Supported | — |
| [`asin`](https://prestodb.io/docs/current/functions/math.html#asin-x-double) | Supported | — |
| [`atan`](https://prestodb.io/docs/current/functions/math.html#atan-x-double) | Supported | — |
| [`atan2`](https://prestodb.io/docs/current/functions/math.html#atan2-y-x-double) | Supported | — |
| [`beta_cdf`](https://prestodb.io/docs/current/functions/math.html#beta_cdf-a-b-value-double) | Supported | — |
| [`binomial_cdf`](https://prestodb.io/docs/current/functions/math.html#binomial_cdf-numberOfTrials-successProbability-value-double) | Supported | — |
| [`cauchy_cdf`](https://prestodb.io/docs/current/functions/math.html#cauchy_cdf-median-scale-value-double) | Supported | — |
| [`cbrt`](https://prestodb.io/docs/current/functions/math.html#cbrt-x-double) | Supported | — |
| [`ceil`](https://prestodb.io/docs/current/functions/math.html#ceil-x-same-as-input) | Supported | — |
| [`ceiling`](https://prestodb.io/docs/current/functions/math.html#ceiling-x-same-as-input) | Supported | — |
| [`chi_squared_cdf`](https://prestodb.io/docs/current/functions/math.html#chi_squared_cdf-df-value-double) | Supported | — |
| [`cos`](https://prestodb.io/docs/current/functions/math.html#cos-x-double) | Supported | — |
| [`cosh`](https://prestodb.io/docs/current/functions/math.html#cosh-x-double) | Supported | — |
| [`cosine_similarity`](https://prestodb.io/docs/current/functions/math.html#cosine_similarity-x-y-double) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`degrees`](https://prestodb.io/docs/current/functions/math.html#degrees-x-double) | Supported | — |
| [`dot_product`](https://prestodb.io/docs/current/functions/math.html#dot_product-array-real-array-real-real) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`e`](https://prestodb.io/docs/current/functions/math.html#e-double) | Supported | — |
| [`exp`](https://prestodb.io/docs/current/functions/math.html#exp-x-double) | Supported | — |
| [`f_cdf`](https://prestodb.io/docs/current/functions/math.html#f_cdf-df1-df2-value-double) | Supported | — |
| [`factorial`](https://prestodb.io/docs/current/functions/math.html#factorial-x-bigint) | Supported | — |
| [`floor`](https://prestodb.io/docs/current/functions/math.html#floor-x-same-as-input) | Supported | — |
| [`from_base`](https://prestodb.io/docs/current/functions/math.html#from_base-string-radix-bigint) | Supported | — |
| [`gamma_cdf`](https://prestodb.io/docs/current/functions/math.html#gamma_cdf-shape-scale-value-double) | Supported | — |
| [`infinity`](https://prestodb.io/docs/current/functions/math.html#infinity-double) | Supported | — |
| [`inverse_beta_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_beta_cdf-a-b-p-double) | Supported | — |
| [`inverse_binomial_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_binomial_cdf-numberOfTrials-successProbability-p-int) | Supported | — |
| [`inverse_cauchy_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_cauchy_cdf-median-scale-p-double) | Supported | — |
| [`inverse_chi_squared_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_chi_squared_cdf-df-p-double) | Supported | — |
| [`inverse_f_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_f_cdf-df1-df2-p-double) | Supported | — |
| [`inverse_gamma_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_gamma_cdf-shape-scale-p-double) | Supported | — |
| [`inverse_laplace_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_laplace_cdf-mean-scale-p-double) | Supported | — |
| [`inverse_normal_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_normal_cdf-mean-sd-p-double) | Supported | — |
| [`inverse_poisson_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_poisson_cdf-lambda-p-integer) | Supported | — |
| [`inverse_t_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_t_cdf-df-p-double) | Supported | — |
| [`inverse_weibull_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_weibull_cdf-a-b-p-double) | Supported | — |
| [`is_finite`](https://prestodb.io/docs/current/functions/math.html#is_finite-x-boolean) | Supported | — |
| [`is_infinite`](https://prestodb.io/docs/current/functions/math.html#is_infinite-x-boolean) | Supported | — |
| [`is_nan`](https://prestodb.io/docs/current/functions/math.html#is_nan-x-boolean) | Supported | — |
| [`l2_squared`](https://prestodb.io/docs/current/functions/math.html#l2_squared-array-real-array-real-real) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`laplace_cdf`](https://prestodb.io/docs/current/functions/math.html#laplace_cdf-mean-scale-value-double) | Supported | — |
| [`ln`](https://prestodb.io/docs/current/functions/math.html#ln-x-double) | Supported | — |
| [`log10`](https://prestodb.io/docs/current/functions/math.html#log10-x-double) | Supported | — |
| [`log2`](https://prestodb.io/docs/current/functions/math.html#log2-x-double) | Supported | — |
| [`mod`](https://prestodb.io/docs/current/functions/math.html#mod-n-m-same-as-input) | Supported | — |
| [`nan`](https://prestodb.io/docs/current/functions/math.html#nan-double) | Supported | — |
| [`normal_cdf`](https://prestodb.io/docs/current/functions/math.html#normal_cdf-mean-sd-value-double) | Supported | — |
| [`pi`](https://prestodb.io/docs/current/functions/math.html#pi-double) | Supported | — |
| [`poisson_cdf`](https://prestodb.io/docs/current/functions/math.html#poisson_cdf-lambda-value-double) | Supported | — |
| [`pow`](https://prestodb.io/docs/current/functions/math.html#pow-x-p-double) | Supported | — |
| [`power`](https://prestodb.io/docs/current/functions/math.html#power-x-p-double) | Supported | — |
| [`radians`](https://prestodb.io/docs/current/functions/math.html#radians-x-double) | Supported | — |
| [`rand`](https://prestodb.io/docs/current/functions/math.html#rand-double) | Supported | Non-deterministic random output. |
| [`random`](https://prestodb.io/docs/current/functions/math.html#random-double) | Supported | Non-deterministic random output. |
| [`round`](https://prestodb.io/docs/current/functions/math.html#round-x-same-as-input) | Supported | — |
| [`secure_rand`](https://prestodb.io/docs/current/functions/math.html#secure_rand-double) | Supported | Non-deterministic random output (OS RNG). |
| [`secure_random`](https://prestodb.io/docs/current/functions/math.html#secure_random-double) | Supported | Non-deterministic random output (OS RNG). |
| [`sign`](https://prestodb.io/docs/current/functions/math.html#sign-x-same-as-input) | Supported | — |
| [`sin`](https://prestodb.io/docs/current/functions/math.html#sin-x-double) | Supported | — |
| [`sqrt`](https://prestodb.io/docs/current/functions/math.html#sqrt-x-double) | Supported | — |
| [`t_cdf`](https://prestodb.io/docs/current/functions/math.html#t_cdf-df-value-double) | Supported | — |
| [`tan`](https://prestodb.io/docs/current/functions/math.html#tan-x-double) | Supported | — |
| [`tanh`](https://prestodb.io/docs/current/functions/math.html#tanh-x-double) | Supported | — |
| [`to_base`](https://prestodb.io/docs/current/functions/math.html#to_base-x-radix-varchar) | Supported | — |
| [`truncate`](https://prestodb.io/docs/current/functions/math.html#truncate-x-double) | Supported | — |
| [`weibull_cdf`](https://prestodb.io/docs/current/functions/math.html#weibull_cdf-a-b-value-double) | Supported | — |
| [`width_bucket`](https://prestodb.io/docs/current/functions/math.html#width_bucket-x-bound1-bound2-n-bigint) | Supported | — |
| [`wilson_interval_lower`](https://prestodb.io/docs/current/functions/math.html#wilson_interval_lower-successes-trials-z-double) | Supported | — |
| [`wilson_interval_upper`](https://prestodb.io/docs/current/functions/math.html#wilson_interval_upper-successes-trials-z-double) | Supported | — |

### Bitwise Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`bit_count`](https://prestodb.io/docs/current/functions/bitwise.html#bit_count-x-bits-bigint) | Supported | — |
| [`bitwise_and`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_and-x-y-bigint) | Supported | — |
| [`bitwise_arithmetic_shift_right`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_arithmetic_shift_right-x-shift-bigint) | Supported | — |
| [`bitwise_left_shift`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_left_shift-value-shift-same-as-value) | Supported | — |
| [`bitwise_logical_shift_right`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_logical_shift_right-x-shift-bits-bigint) | Supported | — |
| [`bitwise_not`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_not-x-bigint) | Supported | — |
| [`bitwise_or`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_or-x-y-bigint) | Supported | — |
| [`bitwise_right_shift`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_right_shift-value-shift-same-as-value) | Supported | — |
| [`bitwise_right_shift_arithmetic`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_right_shift_arithmetic-value-shift-same-as-value) | Supported | — |
| [`bitwise_shift_left`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_shift_left-x-shift-bits-bigint) | Supported | — |
| [`bitwise_xor`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_xor-x-y-bigint) | Supported | — |

### String Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`bit_length`](https://prestodb.io/docs/current/functions/string.html#bit_length-string-boolean) | Supported | — |
| [`chr`](https://prestodb.io/docs/current/functions/string.html#chr-n-varchar) | Supported | — |
| [`codepoint`](https://prestodb.io/docs/current/functions/string.html#codepoint-string-integer) | Supported | — |
| [`concat`](https://prestodb.io/docs/current/functions/string.html#concat-string1-...-stringN-varchar) | Supported | Single p_concat bridge handles both text and blob forms. |
| [`ends_with`](https://prestodb.io/docs/current/functions/string.html#ends_with-string-substring-boolean) | Supported | — |
| [`from_utf8`](https://prestodb.io/docs/current/functions/string.html#from_utf8-binary-varchar) | Supported | — |
| [`hamming_distance`](https://prestodb.io/docs/current/functions/string.html#hamming_distance-string1-string2-bigint) | Supported | — |
| [`jarowinkler_similarity`](https://prestodb.io/docs/current/functions/string.html#jarowinkler_similarity-string1-string2-double) | Supported | — |
| [`key_sampling_percent`](https://prestodb.io/docs/current/functions/string.html#key_sampling_percent-varchar-double) | Supported | — |
| [`length`](https://prestodb.io/docs/current/functions/string.html#length-string-bigint) | Supported | Single p_length bridge handles both text and blob forms. |
| [`levenshtein_distance`](https://prestodb.io/docs/current/functions/string.html#levenshtein_distance-string1-string2-bigint) | Supported | — |
| [`longest_common_prefix`](https://prestodb.io/docs/current/functions/string.html#longest_common_prefix-string1-string2-varchar) | Supported | — |
| [`lower`](https://prestodb.io/docs/current/functions/string.html#lower-string-varchar) | Supported | — |
| [`lpad`](https://prestodb.io/docs/current/functions/string.html#lpad-string-size-padstring-varchar) | Supported | Single p_lpad bridge handles both text and blob forms. |
| [`ltrim`](https://prestodb.io/docs/current/functions/string.html#ltrim-string-varchar) | Supported | — |
| [`normalize`](https://prestodb.io/docs/current/functions/string.html#normalize-string-varchar) | Supported | — |
| [`position`](https://prestodb.io/docs/current/functions/string.html#position-substring-IN-string-bigint) | No | Not currently exposed by this extension. |
| [`replace`](https://prestodb.io/docs/current/functions/string.html#replace-string-search-varchar) | Supported | — |
| [`replace_first`](https://prestodb.io/docs/current/functions/string.html#replace_first-string-search-replace-varchar) | Supported | — |
| [`reverse`](https://prestodb.io/docs/current/functions/string.html#reverse-string-varchar) | Supported | Single p_reverse bridge handles both text and blob forms. |
| [`rpad`](https://prestodb.io/docs/current/functions/string.html#rpad-string-size-padstring-varchar) | Supported | Single p_rpad bridge handles both text and blob forms. |
| [`rtrim`](https://prestodb.io/docs/current/functions/string.html#rtrim-string-varchar) | Supported | — |
| [`split`](https://prestodb.io/docs/current/functions/string.html#split) | Supported | Returns comma-joined text instead of array in SQLite bridge. |
| [`split_part`](https://prestodb.io/docs/current/functions/string.html#split_part-string-delimiter-index-varchar) | Supported | — |
| [`split_to_map`](https://prestodb.io/docs/current/functions/string.html#split_to_map-string-entryDelimiter-keyValueDelimiter-map-varchar-varchar) | No | Not currently exposed by this extension. |
| [`split_to_multimap`](https://prestodb.io/docs/current/functions/string.html#split_to_multimap) | Supported | Returns encoded text (key=v1\|v2;...) instead of map type. |
| [`starts_with`](https://prestodb.io/docs/current/functions/string.html#starts_with-string-substring-boolean) | Supported | — |
| [`strpos`](https://prestodb.io/docs/current/functions/string.html#strpos-string-substring-bigint) | Supported | — |
| [`strrpos`](https://prestodb.io/docs/current/functions/string.html#strrpos-string-substring-bigint) | Supported | — |
| [`substr`](https://prestodb.io/docs/current/functions/string.html#substr-string-start-varchar) | Supported | Single p_substr bridge handles both text and blob forms. |
| [`to_utf8`](https://prestodb.io/docs/current/functions/string.html#to_utf8-string-varbinary) | Supported | — |
| [`trail`](https://prestodb.io/docs/current/functions/string.html#trail-string-N-varchar) | Supported | — |
| [`trim`](https://prestodb.io/docs/current/functions/string.html#trim-string-varchar) | Supported | — |
| [`upper`](https://prestodb.io/docs/current/functions/string.html#upper-string-varchar) | Supported | — |
| [`word_stem`](https://prestodb.io/docs/current/functions/string.html#word_stem-word-varchar) | Supported | — |

### Regular Expression Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`regexp_extract`](https://prestodb.io/docs/current/functions/regexp.html#regexp_extract-string-pattern-varchar) | Supported | Uses Rust regex engine (not full Java/Presto regex semantics). |
| [`regexp_extract_all`](https://prestodb.io/docs/current/functions/regexp.html#regexp_extract_all) | Supported | Returns comma-joined text instead of array in SQLite bridge. |
| [`regexp_like`](https://prestodb.io/docs/current/functions/regexp.html#regexp_like-string-pattern-boolean) | Supported | Uses Rust regex engine (not full Java/Presto regex semantics). |
| [`regexp_replace`](https://prestodb.io/docs/current/functions/regexp.html#regexp_replace-string-pattern-varchar) | Supported | Lambda/function replacement overload is not implemented. |
| [`regexp_split`](https://prestodb.io/docs/current/functions/regexp.html#regexp_split) | Supported | Returns comma-joined text instead of array in SQLite bridge. |

### Binary Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`crc32`](https://prestodb.io/docs/current/functions/binary.html#crc32-binary-bigint) | Supported | — |
| [`from_base32`](https://prestodb.io/docs/current/functions/binary.html#from_base32-string-varbinary) | Supported | — |
| [`from_base64`](https://prestodb.io/docs/current/functions/binary.html#from_base64-string-varbinary) | Supported | — |
| [`from_base64url`](https://prestodb.io/docs/current/functions/binary.html#from_base64url-string-varbinary) | Supported | — |
| [`from_big_endian_32`](https://prestodb.io/docs/current/functions/binary.html#from_big_endian_32-binary-integer) | Supported | — |
| [`from_big_endian_64`](https://prestodb.io/docs/current/functions/binary.html#from_big_endian_64-binary-bigint) | Supported | — |
| [`from_hex`](https://prestodb.io/docs/current/functions/binary.html#from_hex-string-varbinary) | Supported | — |
| [`from_ieee754_32`](https://prestodb.io/docs/current/functions/binary.html#from_ieee754_32-binary-real) | Supported | — |
| [`from_ieee754_64`](https://prestodb.io/docs/current/functions/binary.html#from_ieee754_64-binary-double) | Supported | — |
| [`hmac_md5`](https://prestodb.io/docs/current/functions/binary.html#hmac_md5-binary-key-varbinary) | Supported | — |
| [`hmac_sha1`](https://prestodb.io/docs/current/functions/binary.html#hmac_sha1-binary-key-varbinary) | Supported | — |
| [`hmac_sha256`](https://prestodb.io/docs/current/functions/binary.html#hmac_sha256-binary-key-varbinary) | Supported | — |
| [`hmac_sha512`](https://prestodb.io/docs/current/functions/binary.html#hmac_sha512-binary-key-varbinary) | Supported | — |
| [`md5`](https://prestodb.io/docs/current/functions/binary.html#md5-binary-varbinary) | Supported | — |
| [`murmur3_x64_128`](https://prestodb.io/docs/current/functions/binary.html#murmur3_x64_128-binary-varbinary) | Supported | — |
| [`sha1`](https://prestodb.io/docs/current/functions/binary.html#sha1-binary-varbinary) | Supported | — |
| [`sha256`](https://prestodb.io/docs/current/functions/binary.html#sha256-binary-varbinary) | Supported | — |
| [`sha512`](https://prestodb.io/docs/current/functions/binary.html#sha512-binary-varbinary) | Supported | — |
| [`spooky_hash_v2_32`](https://prestodb.io/docs/current/functions/binary.html#spooky_hash_v2_32-binary-varbinary) | Supported | — |
| [`spooky_hash_v2_64`](https://prestodb.io/docs/current/functions/binary.html#spooky_hash_v2_64-binary-varbinary) | Supported | — |
| [`to_base32`](https://prestodb.io/docs/current/functions/binary.html#to_base32-binary-varchar) | Supported | — |
| [`to_base64`](https://prestodb.io/docs/current/functions/binary.html#to_base64-binary-varchar) | Supported | — |
| [`to_base64url`](https://prestodb.io/docs/current/functions/binary.html#to_base64url-binary-varchar) | Supported | — |
| [`to_big_endian_32`](https://prestodb.io/docs/current/functions/binary.html#to_big_endian_32-integer-varbinary) | Supported | — |
| [`to_big_endian_64`](https://prestodb.io/docs/current/functions/binary.html#to_big_endian_64-bigint-varbinary) | Supported | — |
| [`to_hex`](https://prestodb.io/docs/current/functions/binary.html#to_hex-binary-varchar) | Supported | — |
| [`to_ieee754_32`](https://prestodb.io/docs/current/functions/binary.html#to_ieee754_32-real-varbinary) | Supported | — |
| [`to_ieee754_64`](https://prestodb.io/docs/current/functions/binary.html#to_ieee754_64-double-varbinary) | Supported | — |
| [`xxhash64`](https://prestodb.io/docs/current/functions/binary.html#xxhash64-binary-varbinary) | Supported | — |

### JSON Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`is_json_scalar`](https://prestodb.io/docs/current/functions/json.html#is_json_scalar-json-boolean) | No | Not currently exposed by this extension. |
| [`json_array_contains`](https://prestodb.io/docs/current/functions/json.html#json_array_contains-json-value-boolean) | No | Not currently exposed by this extension. |
| [`json_array_get`](https://prestodb.io/docs/current/functions/json.html#json_array_get-json_array-index-json) | No | Not currently exposed by this extension. |
| [`json_array_length`](https://prestodb.io/docs/current/functions/json.html#json_array_length-json-bigint) | No | Not currently exposed by this extension. |
| [`json_extract`](https://prestodb.io/docs/current/functions/json.html#json_extract-json-json_path-json) | No | Not currently exposed by this extension. |
| [`json_extract_scalar`](https://prestodb.io/docs/current/functions/json.html#json_extract_scalar-json-json_path-varchar) | No | Not currently exposed by this extension. |
| [`json_format`](https://prestodb.io/docs/current/functions/json.html#json_format-json-varchar) | No | Not currently exposed by this extension. |
| [`json_parse`](https://prestodb.io/docs/current/functions/json.html#json_parse-string-json) | No | Not currently exposed by this extension. |
| [`json_size`](https://prestodb.io/docs/current/functions/json.html#json_size-json-json_path-bigint) | No | Not currently exposed by this extension. |

### Date and Time Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`current_date`](https://prestodb.io/docs/current/functions/datetime.html#current_date-date) | No | Not currently exposed by this extension. |
| [`current_time`](https://prestodb.io/docs/current/functions/datetime.html#current_time-time-with-time-zone) | No | Not currently exposed by this extension. |
| [`current_timestamp`](https://prestodb.io/docs/current/functions/datetime.html#current_timestamp-timestamp-with-time-zone) | No | Not currently exposed by this extension. |
| [`current_timezone`](https://prestodb.io/docs/current/functions/datetime.html#current_timezone-varchar) | No | Not currently exposed by this extension. |
| [`date`](https://prestodb.io/docs/current/functions/datetime.html#date-x-date) | No | Not currently exposed by this extension. |
| [`date_add`](https://prestodb.io/docs/current/functions/datetime.html#date_add-unit-value-timestamp-same-as-input) | No | Not currently exposed by this extension. |
| [`date_diff`](https://prestodb.io/docs/current/functions/datetime.html#date_diff-unit-timestamp1-timestamp2-bigint) | No | Not currently exposed by this extension. |
| [`date_format`](https://prestodb.io/docs/current/functions/datetime.html#date_format-timestamp-format-varchar) | No | Not currently exposed by this extension. |
| [`date_parse`](https://prestodb.io/docs/current/functions/datetime.html#date_parse-string-format-timestamp) | No | Not currently exposed by this extension. |
| [`date_trunc`](https://prestodb.io/docs/current/functions/datetime.html#date_trunc-unit-x-same-as-input) | No | Not currently exposed by this extension. |
| [`day`](https://prestodb.io/docs/current/functions/datetime.html#day-x-bigint) | No | Not currently exposed by this extension. |
| [`day_of_month`](https://prestodb.io/docs/current/functions/datetime.html#day_of_month-x-bigint) | No | Not currently exposed by this extension. |
| [`day_of_week`](https://prestodb.io/docs/current/functions/datetime.html#day_of_week-x-bigint) | No | Not currently exposed by this extension. |
| [`day_of_year`](https://prestodb.io/docs/current/functions/datetime.html#day_of_year-x-bigint) | No | Not currently exposed by this extension. |
| [`dow`](https://prestodb.io/docs/current/functions/datetime.html#dow-x-bigint) | No | Not currently exposed by this extension. |
| [`doy`](https://prestodb.io/docs/current/functions/datetime.html#doy-x-bigint) | No | Not currently exposed by this extension. |
| [`extract`](https://prestodb.io/docs/current/functions/datetime.html#extract-field-FROM-x-bigint) | No | Not currently exposed by this extension. |
| [`format_datetime`](https://prestodb.io/docs/current/functions/datetime.html#format_datetime-timestamp-format-varchar) | No | Not currently exposed by this extension. |
| [`from_iso8601_date`](https://prestodb.io/docs/current/functions/datetime.html#from_iso8601_date-string-date) | No | Not currently exposed by this extension. |
| [`from_iso8601_timestamp`](https://prestodb.io/docs/current/functions/datetime.html#from_iso8601_timestamp-string-timestamp-with-time-zone) | No | Not currently exposed by this extension. |
| [`from_unixtime`](https://prestodb.io/docs/current/functions/datetime.html#from_unixtime-unixtime-timestamp) | No | Not currently exposed by this extension. |
| [`hour`](https://prestodb.io/docs/current/functions/datetime.html#hour-x-bigint) | No | Not currently exposed by this extension. |
| [`last_day_of_month`](https://prestodb.io/docs/current/functions/datetime.html#last_day_of_month-x-date) | No | Not currently exposed by this extension. |
| [`localtime`](https://prestodb.io/docs/current/functions/datetime.html#localtime-time) | No | Not currently exposed by this extension. |
| [`localtimestamp`](https://prestodb.io/docs/current/functions/datetime.html#localtimestamp-timestamp) | No | Not currently exposed by this extension. |
| [`millisecond`](https://prestodb.io/docs/current/functions/datetime.html#millisecond-x-bigint) | No | Not currently exposed by this extension. |
| [`minute`](https://prestodb.io/docs/current/functions/datetime.html#minute-x-bigint) | No | Not currently exposed by this extension. |
| [`month`](https://prestodb.io/docs/current/functions/datetime.html#month-x-bigint) | No | Not currently exposed by this extension. |
| [`now`](https://prestodb.io/docs/current/functions/datetime.html#now-timestamp-with-time-zone) | No | Not currently exposed by this extension. |
| [`parse_datetime`](https://prestodb.io/docs/current/functions/datetime.html#parse_datetime-string-format-timestamp-with-time-zone) | No | Not currently exposed by this extension. |
| [`parse_duration`](https://prestodb.io/docs/current/functions/datetime.html#parse_duration-string-interval) | No | Not currently exposed by this extension. |
| [`quarter`](https://prestodb.io/docs/current/functions/datetime.html#quarter-x-bigint) | No | Not currently exposed by this extension. |
| [`second`](https://prestodb.io/docs/current/functions/datetime.html#second-x-bigint) | No | Not currently exposed by this extension. |
| [`timezone_hour`](https://prestodb.io/docs/current/functions/datetime.html#timezone_hour-timestamp-bigint) | No | Not currently exposed by this extension. |
| [`timezone_minute`](https://prestodb.io/docs/current/functions/datetime.html#timezone_minute-timestamp-bigint) | No | Not currently exposed by this extension. |
| [`to_iso8601`](https://prestodb.io/docs/current/functions/datetime.html#to_iso8601-x-varchar) | No | Not currently exposed by this extension. |
| [`to_milliseconds`](https://prestodb.io/docs/current/functions/datetime.html#to_milliseconds-interval-bigint) | No | Not currently exposed by this extension. |
| [`to_unixtime`](https://prestodb.io/docs/current/functions/datetime.html#to_unixtime-timestamp-double) | No | Not currently exposed by this extension. |
| [`week`](https://prestodb.io/docs/current/functions/datetime.html#week-x-bigint) | No | Not currently exposed by this extension. |
| [`week_of_year`](https://prestodb.io/docs/current/functions/datetime.html#week_of_year-x-bigint) | No | Not currently exposed by this extension. |
| [`year`](https://prestodb.io/docs/current/functions/datetime.html#year-x-bigint) | No | Not currently exposed by this extension. |
| [`year_of_week`](https://prestodb.io/docs/current/functions/datetime.html#year_of_week-x-bigint) | No | Not currently exposed by this extension. |
| [`yow`](https://prestodb.io/docs/current/functions/datetime.html#yow-x-bigint) | No | Not currently exposed by this extension. |

### Aggregate Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`any_value`](https://prestodb.io/docs/current/functions/aggregate.html#any_value-x-same-as-input) | No | Not currently exposed by this extension. |
| [`approx_distinct`](https://prestodb.io/docs/current/functions/aggregate.html#approx_distinct-x-bigint) | No | Not currently exposed by this extension. |
| [`approx_most_frequent`](https://prestodb.io/docs/current/functions/aggregate.html#approx_most_frequent-buckets-value-capacity-map-same-as-value-bigint) | No | Not currently exposed by this extension. |
| [`approx_percentile`](https://prestodb.io/docs/current/functions/aggregate.html#approx_percentile-x-percentage-same-as-x) | No | Not currently exposed by this extension. |
| [`approx_set`](https://prestodb.io/docs/current/functions/aggregate.html) | No | Not currently exposed by this extension. |
| [`arbitrary`](https://prestodb.io/docs/current/functions/aggregate.html#arbitrary-x-same-as-input) | No | Not currently exposed by this extension. |
| [`array_agg`](https://prestodb.io/docs/current/functions/aggregate.html#array_agg-x-array-same-as-input) | No | Not currently exposed by this extension. |
| [`avg`](https://prestodb.io/docs/current/functions/aggregate.html#avg-x-double) | No | Not currently exposed by this extension. |
| [`bitwise_and_agg`](https://prestodb.io/docs/current/functions/aggregate.html#bitwise_and_agg-x-bigint) | No | Not currently exposed by this extension. |
| [`bitwise_or_agg`](https://prestodb.io/docs/current/functions/aggregate.html#bitwise_or_agg-x-bigint) | No | Not currently exposed by this extension. |
| [`bitwise_xor_agg`](https://prestodb.io/docs/current/functions/aggregate.html#bitwise_xor_agg-x-bigint) | No | Not currently exposed by this extension. |
| [`bool_and`](https://prestodb.io/docs/current/functions/aggregate.html#bool_and-boolean-boolean) | No | Not currently exposed by this extension. |
| [`bool_or`](https://prestodb.io/docs/current/functions/aggregate.html#bool_or-boolean-boolean) | No | Not currently exposed by this extension. |
| [`checksum`](https://prestodb.io/docs/current/functions/aggregate.html#checksum-x-varbinary) | No | Not currently exposed by this extension. |
| [`classification_fall_out`](https://prestodb.io/docs/current/functions/aggregate.html#classification_fall_out-buckets-y-x-weight-array-double) | No | Not currently exposed by this extension. |
| [`classification_miss_rate`](https://prestodb.io/docs/current/functions/aggregate.html#classification_miss_rate-buckets-y-x-weight-array-double) | No | Not currently exposed by this extension. |
| [`classification_precision`](https://prestodb.io/docs/current/functions/aggregate.html#classification_precision-buckets-y-x-weight-array-double) | No | Not currently exposed by this extension. |
| [`classification_recall`](https://prestodb.io/docs/current/functions/aggregate.html#classification_recall-buckets-y-x-weight-array-double) | No | Not currently exposed by this extension. |
| [`classification_thresholds`](https://prestodb.io/docs/current/functions/aggregate.html#classification_thresholds-buckets-y-x-array-double) | No | Not currently exposed by this extension. |
| [`corr`](https://prestodb.io/docs/current/functions/aggregate.html#corr-y-x-double) | No | Not currently exposed by this extension. |
| [`count`](https://prestodb.io/docs/current/functions/aggregate.html#count-bigint) | No | Not currently exposed by this extension. |
| [`count_if`](https://prestodb.io/docs/current/functions/aggregate.html#count_if-x-bigint) | No | Not currently exposed by this extension. |
| [`covar_pop`](https://prestodb.io/docs/current/functions/aggregate.html#covar_pop-y-x-double) | No | Not currently exposed by this extension. |
| [`covar_samp`](https://prestodb.io/docs/current/functions/aggregate.html#covar_samp-y-x-double) | No | Not currently exposed by this extension. |
| [`differential_entropy`](https://prestodb.io/docs/current/functions/aggregate.html#differential_entropy) | No | Not currently exposed by this extension. |
| [`entropy`](https://prestodb.io/docs/current/functions/aggregate.html#entropy-c-double) | No | Not currently exposed by this extension. |
| [`every`](https://prestodb.io/docs/current/functions/aggregate.html#every-boolean-boolean) | No | Not currently exposed by this extension. |
| [`geometric_mean`](https://prestodb.io/docs/current/functions/aggregate.html#geometric_mean-bigint-double) | No | Not currently exposed by this extension. |
| [`histogram`](https://prestodb.io/docs/current/functions/aggregate.html#histogram) | No | Not currently exposed by this extension. |
| [`khyperloglog_agg`](https://prestodb.io/docs/current/functions/aggregate.html) | No | Not currently exposed by this extension. |
| [`kurtosis`](https://prestodb.io/docs/current/functions/aggregate.html#kurtosis-x-double) | No | Not currently exposed by this extension. |
| [`map_agg`](https://prestodb.io/docs/current/functions/aggregate.html#map_agg) | No | Not currently exposed by this extension. |
| [`map_union`](https://prestodb.io/docs/current/functions/aggregate.html#map_union) | No | Not currently exposed by this extension. |
| [`map_union_sum`](https://prestodb.io/docs/current/functions/aggregate.html#map_union_sum) | No | Not currently exposed by this extension. |
| [`max`](https://prestodb.io/docs/current/functions/aggregate.html#max-x-same-as-input) | No | Not currently exposed by this extension. |
| [`max_by`](https://prestodb.io/docs/current/functions/aggregate.html#max_by-x-y-same-as-x) | No | Not currently exposed by this extension. |
| [`merge`](https://prestodb.io/docs/current/functions/aggregate.html) | No | Not currently exposed by this extension. |
| [`min`](https://prestodb.io/docs/current/functions/aggregate.html#min-x-same-as-input) | No | Not currently exposed by this extension. |
| [`min_by`](https://prestodb.io/docs/current/functions/aggregate.html#min_by-x-y-same-as-x) | No | Not currently exposed by this extension. |
| [`multimap_agg`](https://prestodb.io/docs/current/functions/aggregate.html#multimap_agg) | No | Not currently exposed by this extension. |
| [`numeric_histogram`](https://prestodb.io/docs/current/functions/aggregate.html#numeric_histogram-buckets-value-weight-map-double-double) | No | Not currently exposed by this extension. |
| [`qdigest_agg`](https://prestodb.io/docs/current/functions/aggregate.html) | No | Not currently exposed by this extension. |
| [`reduce_agg`](https://prestodb.io/docs/current/functions/aggregate.html#reduce_agg-inputValue-T-initialState-S-inputFunction-S-T-S-combineFunction-S-S-S-S) | No | Not currently exposed by this extension. |
| [`regr_avgx`](https://prestodb.io/docs/current/functions/aggregate.html#regr_avgx-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_avgy`](https://prestodb.io/docs/current/functions/aggregate.html#regr_avgy-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_count`](https://prestodb.io/docs/current/functions/aggregate.html#regr_count-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_intercept`](https://prestodb.io/docs/current/functions/aggregate.html#regr_intercept-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_r2`](https://prestodb.io/docs/current/functions/aggregate.html#regr_r2-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_slope`](https://prestodb.io/docs/current/functions/aggregate.html#regr_slope-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_sxx`](https://prestodb.io/docs/current/functions/aggregate.html#regr_sxx-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_sxy`](https://prestodb.io/docs/current/functions/aggregate.html#regr_sxy-y-x-double) | No | Not currently exposed by this extension. |
| [`regr_syy`](https://prestodb.io/docs/current/functions/aggregate.html#regr_syy-y-x-double) | No | Not currently exposed by this extension. |
| [`reservoir_sample`](https://prestodb.io/docs/current/functions/aggregate.html#reservoir_sample) | No | Not currently exposed by this extension. |
| [`set_agg`](https://prestodb.io/docs/current/functions/aggregate.html#set_agg-x-array-same-as-input) | No | Not currently exposed by this extension. |
| [`set_union`](https://prestodb.io/docs/current/functions/aggregate.html#set_union) | No | Not currently exposed by this extension. |
| [`skewness`](https://prestodb.io/docs/current/functions/aggregate.html#skewness-x-double) | No | Not currently exposed by this extension. |
| [`stddev`](https://prestodb.io/docs/current/functions/aggregate.html#stddev-x-double) | No | Not currently exposed by this extension. |
| [`stddev_pop`](https://prestodb.io/docs/current/functions/aggregate.html#stddev_pop-x-double) | No | Not currently exposed by this extension. |
| [`stddev_samp`](https://prestodb.io/docs/current/functions/aggregate.html#stddev_samp-x-double) | No | Not currently exposed by this extension. |
| [`sum`](https://prestodb.io/docs/current/functions/aggregate.html#sum-x-same-as-input) | No | Not currently exposed by this extension. |
| [`var_pop`](https://prestodb.io/docs/current/functions/aggregate.html#var_pop-x-double) | No | Not currently exposed by this extension. |
| [`var_samp`](https://prestodb.io/docs/current/functions/aggregate.html#var_samp-x-double) | No | Not currently exposed by this extension. |
| [`variance`](https://prestodb.io/docs/current/functions/aggregate.html#variance-x-double) | No | Not currently exposed by this extension. |

### Noisy Aggregate Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`cardinality`](https://prestodb.io/docs/current/functions/noisy.html#cardinality-SfmSketch-bigint) | No | Not currently exposed by this extension. |
| [`merge_sfm`](https://prestodb.io/docs/current/functions/noisy.html#merge_sfm-ARRAY-SfmSketch-...-SfmSketch) | No | Not currently exposed by this extension. |
| [`noisy_approx_distinct_sfm`](https://prestodb.io/docs/current/functions/noisy.html#noisy_approx_distinct_sfm-col-epsilon-buckets-precision-bigint) | No | Not currently exposed by this extension. |
| [`noisy_approx_set_sfm`](https://prestodb.io/docs/current/functions/noisy.html#noisy_approx_set_sfm-col-epsilon-buckets-precision-SfmSketch) | No | Not currently exposed by this extension. |
| [`noisy_approx_set_sfm_from_index_and_zeros`](https://prestodb.io/docs/current/functions/noisy.html#noisy_approx_set_sfm_from_index_and_zeros-col_index-col_zeros-epsilon-buckets-precision-SfmSketch) | No | Not currently exposed by this extension. |
| [`noisy_avg_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_avg_gaussian-col-noise_scale-lower-upper-random_seed-double) | No | Not currently exposed by this extension. |
| [`noisy_count_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_count_gaussian-col-noise_scale-random_seed-bigint) | No | Not currently exposed by this extension. |
| [`noisy_count_if_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_count_if_gaussian-col-noise_scale-random_seed-bigint) | No | Not currently exposed by this extension. |
| [`noisy_empty_approx_set_sfm`](https://prestodb.io/docs/current/functions/noisy.html#noisy_empty_approx_set_sfm-epsilon-buckets-precision-SfmSketch) | No | Not currently exposed by this extension. |
| [`noisy_sum_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_sum_gaussian-col-noise_scale-lower-upper-random_seed-double) | No | Not currently exposed by this extension. |

### Window Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`cume_dist`](https://prestodb.io/docs/current/functions/window.html#cume_dist-double) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`dense_rank`](https://prestodb.io/docs/current/functions/window.html#dense_rank-bigint) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`first_value`](https://prestodb.io/docs/current/functions/window.html#first_value-x-same-as-input) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`lag`](https://prestodb.io/docs/current/functions/window.html#lag-x-offset-default_value-same-as-input) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`last_value`](https://prestodb.io/docs/current/functions/window.html#last_value-x-same-as-input) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`lead`](https://prestodb.io/docs/current/functions/window.html#lead-x-offset-default_value-same-as-input) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`nth_value`](https://prestodb.io/docs/current/functions/window.html#nth_value-x-offset-same-as-input) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`ntile`](https://prestodb.io/docs/current/functions/window.html#ntile-n-bigint) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`percent_rank`](https://prestodb.io/docs/current/functions/window.html#percent_rank-double) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`rank`](https://prestodb.io/docs/current/functions/window.html#rank-bigint) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| [`row_number`](https://prestodb.io/docs/current/functions/window.html#row_number-bigint) | Stubbed | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |

### Array Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`all_match`](https://prestodb.io/docs/current/functions/array.html#all_match-array-T-function-T-boolean-boolean) | No | Not currently exposed by this extension. |
| [`any_match`](https://prestodb.io/docs/current/functions/array.html#any_match-array-T-function-T-boolean-boolean) | No | Not currently exposed by this extension. |
| [`array_average`](https://prestodb.io/docs/current/functions/array.html#array_average-array-double-double) | No | Not currently exposed by this extension. |
| [`array_cum_sum`](https://prestodb.io/docs/current/functions/array.html#array_cum_sum) | No | Not currently exposed by this extension. |
| [`array_distinct`](https://prestodb.io/docs/current/functions/array.html#array_distinct-x-array) | No | Not currently exposed by this extension. |
| [`array_duplicates`](https://prestodb.io/docs/current/functions/array.html#array_duplicates) | No | Not currently exposed by this extension. |
| [`array_except`](https://prestodb.io/docs/current/functions/array.html#array_except-x-y-array) | No | Not currently exposed by this extension. |
| [`array_frequency`](https://prestodb.io/docs/current/functions/array.html#array_frequency) | No | Not currently exposed by this extension. |
| [`array_has_duplicates`](https://prestodb.io/docs/current/functions/array.html#array_has_duplicates-array-T-boolean) | No | Not currently exposed by this extension. |
| [`array_intersect`](https://prestodb.io/docs/current/functions/array.html#array_intersect-x-y-array) | No | Not currently exposed by this extension. |
| [`array_join`](https://prestodb.io/docs/current/functions/array.html#array_join-x-delimiter-null_replacement-varchar) | No | Not currently exposed by this extension. |
| [`array_least_frequent`](https://prestodb.io/docs/current/functions/array.html#array_least_frequent) | No | Not currently exposed by this extension. |
| [`array_max`](https://prestodb.io/docs/current/functions/array.html#array_max-x-x) | No | Not currently exposed by this extension. |
| [`array_max_by`](https://prestodb.io/docs/current/functions/array.html#array_max_by-array-T-function-T-U-T) | No | Not currently exposed by this extension. |
| [`array_min`](https://prestodb.io/docs/current/functions/array.html#array_min-x-x) | No | Not currently exposed by this extension. |
| [`array_min_by`](https://prestodb.io/docs/current/functions/array.html#array_min_by-array-T-function-T-U-T) | No | Not currently exposed by this extension. |
| [`array_normalize`](https://prestodb.io/docs/current/functions/array.html#array_normalize-x-p-array) | No | Not currently exposed by this extension. |
| [`array_position`](https://prestodb.io/docs/current/functions/array.html#array_position-x-element-bigint) | No | Not currently exposed by this extension. |
| [`array_remove`](https://prestodb.io/docs/current/functions/array.html#array_remove-x-element-array) | No | Not currently exposed by this extension. |
| [`array_sort`](https://prestodb.io/docs/current/functions/array.html#array_sort-x-array) | No | Not currently exposed by this extension. |
| [`array_sort_desc`](https://prestodb.io/docs/current/functions/array.html#array_sort_desc-x-array) | No | Not currently exposed by this extension. |
| [`array_split_into_chunks`](https://prestodb.io/docs/current/functions/array.html#array_split_into_chunks) | No | Not currently exposed by this extension. |
| [`array_sum`](https://prestodb.io/docs/current/functions/array.html#array_sum-array-T-bigint-double) | No | Not currently exposed by this extension. |
| [`array_top_n`](https://prestodb.io/docs/current/functions/array.html#array_top_n) | No | Not currently exposed by this extension. |
| [`array_transpose`](https://prestodb.io/docs/current/functions/array.html#array_transpose) | No | Not currently exposed by this extension. |
| [`array_union`](https://prestodb.io/docs/current/functions/array.html#array_union-x-y-array) | No | Not currently exposed by this extension. |
| [`arrays_overlap`](https://prestodb.io/docs/current/functions/array.html#arrays_overlap-x-y-boolean) | No | Not currently exposed by this extension. |
| [`combinations`](https://prestodb.io/docs/current/functions/array.html#combinations) | No | Not currently exposed by this extension. |
| [`contains`](https://prestodb.io/docs/current/functions/array.html#contains-x-element-boolean) | No | Not currently exposed by this extension. |
| [`element_at`](https://prestodb.io/docs/current/functions/array.html#element_at-array-E-index-E) | No | Not currently exposed by this extension. |
| [`filter`](https://prestodb.io/docs/current/functions/array.html#filter) | No | Not currently exposed by this extension. |
| [`find_first`](https://prestodb.io/docs/current/functions/array.html#find_first-array-E-function-T-boolean-E) | No | Not currently exposed by this extension. |
| [`find_first_index`](https://prestodb.io/docs/current/functions/array.html#find_first_index-array-E-function-T-boolean-BIGINT) | No | Not currently exposed by this extension. |
| [`flatten`](https://prestodb.io/docs/current/functions/array.html#flatten-x-array) | No | Not currently exposed by this extension. |
| [`ngrams`](https://prestodb.io/docs/current/functions/array.html#ngrams) | No | Not currently exposed by this extension. |
| [`none_match`](https://prestodb.io/docs/current/functions/array.html#none_match-array-T-function-T-boolean-boolean) | No | Not currently exposed by this extension. |
| [`reduce`](https://prestodb.io/docs/current/functions/array.html#reduce-array-T-initialState-S-inputFunction-S-T-S-outputFunction-S-R-R) | No | Not currently exposed by this extension. |
| [`remove_nulls`](https://prestodb.io/docs/current/functions/array.html#remove_nulls-array-T-array) | No | Not currently exposed by this extension. |
| [`repeat`](https://prestodb.io/docs/current/functions/array.html#repeat-element-count-array) | No | Not currently exposed by this extension. |
| [`sequence`](https://prestodb.io/docs/current/functions/array.html#sequence) | No | Not currently exposed by this extension. |
| [`shuffle`](https://prestodb.io/docs/current/functions/array.html#shuffle-x-array) | No | Not currently exposed by this extension. |
| [`slice`](https://prestodb.io/docs/current/functions/array.html#slice-x-start-length-array) | No | Not currently exposed by this extension. |
| [`transform`](https://prestodb.io/docs/current/functions/array.html#transform) | No | Not currently exposed by this extension. |
| [`trim_array`](https://prestodb.io/docs/current/functions/array.html#trim_array-x-n-array) | No | Not currently exposed by this extension. |
| [`zip`](https://prestodb.io/docs/current/functions/array.html#zip) | No | Not currently exposed by this extension. |
| [`zip_with`](https://prestodb.io/docs/current/functions/array.html#zip_with) | No | Not currently exposed by this extension. |

### Map Functions and Operators

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`all_keys_match`](https://prestodb.io/docs/current/functions/map.html#all_keys_match-x-K-V-function-K-boolean-boolean) | No | Not currently exposed by this extension. |
| [`any_keys_match`](https://prestodb.io/docs/current/functions/map.html#any_keys_match-x-K-V-function-K-boolean-boolean) | No | Not currently exposed by this extension. |
| [`any_values_match`](https://prestodb.io/docs/current/functions/map.html#any_values_match-x-K-V-function-V-boolean-boolean) | No | Not currently exposed by this extension. |
| [`array_to_map_int_keys`](https://prestodb.io/docs/current/functions/map.html#array_to_map_int_keys) | No | Not currently exposed by this extension. |
| [`map`](https://prestodb.io/docs/current/functions/map.html#map-map-unknown-unknown) | No | Not currently exposed by this extension. |
| [`map_concat`](https://prestodb.io/docs/current/functions/map.html#map_concat) | No | Not currently exposed by this extension. |
| [`map_entries`](https://prestodb.io/docs/current/functions/map.html#map_entries) | No | Not currently exposed by this extension. |
| [`map_filter`](https://prestodb.io/docs/current/functions/map.html#map_filter) | No | Not currently exposed by this extension. |
| [`map_from_entries`](https://prestodb.io/docs/current/functions/map.html#map_from_entries) | No | Not currently exposed by this extension. |
| [`map_int_keys_to_array`](https://prestodb.io/docs/current/functions/map.html#map_int_keys_to_array) | No | Not currently exposed by this extension. |
| [`map_key_exists`](https://prestodb.io/docs/current/functions/map.html#map_key_exists-x-K-V-k-boolean) | No | Not currently exposed by this extension. |
| [`map_keys`](https://prestodb.io/docs/current/functions/map.html#map_keys) | No | Not currently exposed by this extension. |
| [`map_keys_by_top_n_values`](https://prestodb.io/docs/current/functions/map.html#map_keys_by_top_n_values) | No | Not currently exposed by this extension. |
| [`map_normalize`](https://prestodb.io/docs/current/functions/map.html#map_normalize) | No | Not currently exposed by this extension. |
| [`map_remove_null_values`](https://prestodb.io/docs/current/functions/map.html#map_remove_null_values) | No | Not currently exposed by this extension. |
| [`map_subset`](https://prestodb.io/docs/current/functions/map.html#map_subset) | No | Not currently exposed by this extension. |
| [`map_top_n`](https://prestodb.io/docs/current/functions/map.html#map_top_n) | No | Not currently exposed by this extension. |
| [`map_top_n_keys`](https://prestodb.io/docs/current/functions/map.html#map_top_n_keys) | No | Not currently exposed by this extension. |
| [`map_top_n_values`](https://prestodb.io/docs/current/functions/map.html#map_top_n_values) | No | Not currently exposed by this extension. |
| [`map_values`](https://prestodb.io/docs/current/functions/map.html#map_values) | No | Not currently exposed by this extension. |
| [`map_zip_with`](https://prestodb.io/docs/current/functions/map.html#map_zip_with) | No | Not currently exposed by this extension. |
| [`multimap_from_entries`](https://prestodb.io/docs/current/functions/map.html#multimap_from_entries) | No | Not currently exposed by this extension. |
| [`no_keys_match`](https://prestodb.io/docs/current/functions/map.html#no_keys_match-x-K-V-function-K-boolean-boolean) | No | Not currently exposed by this extension. |
| [`no_values_match`](https://prestodb.io/docs/current/functions/map.html#no_values_match-x-K-V-function-V-boolean-boolean) | No | Not currently exposed by this extension. |
| [`transform_keys`](https://prestodb.io/docs/current/functions/map.html#transform_keys) | No | Not currently exposed by this extension. |
| [`transform_values`](https://prestodb.io/docs/current/functions/map.html#transform_values) | No | Not currently exposed by this extension. |

### URL Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`url_decode`](https://prestodb.io/docs/current/functions/url.html#url_decode-value-varchar) | Supported | — |
| [`url_encode`](https://prestodb.io/docs/current/functions/url.html#url_encode-value-varchar) | Supported | — |
| [`url_extract_fragment`](https://prestodb.io/docs/current/functions/url.html#url_extract_fragment-url-varchar) | Supported | — |
| [`url_extract_host`](https://prestodb.io/docs/current/functions/url.html#url_extract_host-url-varchar) | Supported | — |
| [`url_extract_parameter`](https://prestodb.io/docs/current/functions/url.html#url_extract_parameter-url-name-varchar) | Supported | — |
| [`url_extract_path`](https://prestodb.io/docs/current/functions/url.html#url_extract_path-url-varchar) | Supported | — |
| [`url_extract_port`](https://prestodb.io/docs/current/functions/url.html#url_extract_port-url-bigint) | Supported | — |
| [`url_extract_protocol`](https://prestodb.io/docs/current/functions/url.html#url_extract_protocol-url-varchar) | Supported | — |
| [`url_extract_query`](https://prestodb.io/docs/current/functions/url.html#url_extract_query-url-varchar) | Supported | — |

### IP Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`ip_prefix`](https://prestodb.io/docs/current/functions/ip.html#ip_prefix-ip_address-prefix_bits-ipprefix) | Supported | — |
| [`ip_prefix_collapse`](https://prestodb.io/docs/current/functions/ip.html#ip_prefix_collapse) | Supported | Input/output arrays are represented as comma-joined text. |
| [`ip_prefix_subnets`](https://prestodb.io/docs/current/functions/ip.html#ip_prefix_subnets) | Supported | Returns comma-joined text instead of array type. |
| [`ip_subnet_max`](https://prestodb.io/docs/current/functions/ip.html#ip_subnet_max-ip_prefix-ip_address) | Supported | — |
| [`ip_subnet_min`](https://prestodb.io/docs/current/functions/ip.html#ip_subnet_min-ip_prefix-ip_address) | Supported | — |
| [`ip_subnet_range`](https://prestodb.io/docs/current/functions/ip.html#ip_subnet_range) | Supported | Returns comma-joined text instead of array type. |
| [`is_private_ip`](https://prestodb.io/docs/current/functions/ip.html#is_private_ip-ip_address-boolean) | Supported | — |
| [`is_subnet_of`](https://prestodb.io/docs/current/functions/ip.html#is_subnet_of-ip_prefix-ip_address-boolean) | Supported | — |

### Geospatial Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`bing_tile`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile-x-y-zoom_level-BingTile) | No | Not currently exposed by this extension. |
| [`bing_tile_at`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_at-latitude-longitude-zoom_level-BingTile) | No | Not currently exposed by this extension. |
| [`bing_tile_children`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_children) | No | Not currently exposed by this extension. |
| [`bing_tile_coordinates`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_coordinates-tile-row-x-y) | No | Not currently exposed by this extension. |
| [`bing_tile_parent`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_parent-tile-BingTile) | No | Not currently exposed by this extension. |
| [`bing_tile_polygon`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_polygon-tile-Geometry) | No | Not currently exposed by this extension. |
| [`bing_tile_quadkey`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_quadkey-tile-varchar) | No | Not currently exposed by this extension. |
| [`bing_tile_zoom_level`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_zoom_level-tile-tinyint) | No | Not currently exposed by this extension. |
| [`bing_tiles_around`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tiles_around) | No | Not currently exposed by this extension. |
| [`convex_hull_agg`](https://prestodb.io/docs/current/functions/geospatial.html#convex_hull_agg-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`expand_envelope`](https://prestodb.io/docs/current/functions/geospatial.html#expand_envelope-Geometry-double-Geometry) | No | Not currently exposed by this extension. |
| [`flatten_geometry_collections`](https://prestodb.io/docs/current/functions/geospatial.html#flatten_geometry_collections) | No | Not currently exposed by this extension. |
| [`geometry_as_geojson`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_as_geojson-Geometry-varchar) | No | Not currently exposed by this extension. |
| [`geometry_from_geojson`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_from_geojson-varchar-Geometry) | No | Not currently exposed by this extension. |
| [`geometry_invalid_reason`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_invalid_reason-Geometry-varchar) | No | Not currently exposed by this extension. |
| [`geometry_nearest_points`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_nearest_points) | No | Not currently exposed by this extension. |
| [`geometry_to_bing_tiles`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_to_bing_tiles) | No | Not currently exposed by this extension. |
| [`geometry_to_dissolved_bing_tiles`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_to_dissolved_bing_tiles) | No | Not currently exposed by this extension. |
| [`geometry_union`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_union-array-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`geometry_union_agg`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_union_agg-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`great_circle_distance`](https://prestodb.io/docs/current/functions/geospatial.html#great_circle_distance-latitude1-longitude1-latitude2-longitude2-double) | No | Not currently exposed by this extension. |
| [`line_interpolate_point`](https://prestodb.io/docs/current/functions/geospatial.html#line_interpolate_point-LineString-double-Geometry) | No | Not currently exposed by this extension. |
| [`line_locate_point`](https://prestodb.io/docs/current/functions/geospatial.html#line_locate_point-LineString-Point-double) | No | Not currently exposed by this extension. |
| [`simplify_geometry`](https://prestodb.io/docs/current/functions/geospatial.html#simplify_geometry-Geometry-double-Geometry) | No | Not currently exposed by this extension. |
| [`st_area`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Area-Geometry-double) | No | Not currently exposed by this extension. |
| [`st_asbinary`](https://prestodb.io/docs/current/functions/geospatial.html#ST_AsBinary-Geometry-varbinary) | No | Not currently exposed by this extension. |
| [`st_astext`](https://prestodb.io/docs/current/functions/geospatial.html#ST_AsText-Geometry-varchar) | No | Not currently exposed by this extension. |
| [`st_boundary`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Boundary-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_buffer`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Buffer-Geometry-distance-Geometry) | No | Not currently exposed by this extension. |
| [`st_centroid`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Centroid-Geometry-Point) | No | Not currently exposed by this extension. |
| [`st_contains`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Contains-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_convexhull`](https://prestodb.io/docs/current/functions/geospatial.html#ST_ConvexHull-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_coorddim`](https://prestodb.io/docs/current/functions/geospatial.html#ST_CoordDim-Geometry-tinyint) | No | Not currently exposed by this extension. |
| [`st_crosses`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Crosses-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_difference`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Difference-Geometry-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_dimension`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Dimension-Geometry-bigint) | No | Not currently exposed by this extension. |
| [`st_disjoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Disjoint-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_distance`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Distance-Geometry-Geometry-double) | No | Not currently exposed by this extension. |
| [`st_endpoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_EndPoint-Geometry-point) | No | Not currently exposed by this extension. |
| [`st_envelope`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Envelope-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_envelopeaspts`](https://prestodb.io/docs/current/functions/geospatial.html#ST_EnvelopeAsPts) | No | Not currently exposed by this extension. |
| [`st_equals`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Equals-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_exteriorring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_ExteriorRing-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_geometries`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Geometries) | No | Not currently exposed by this extension. |
| [`st_geometryfromtext`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeometryFromText-varchar-Geometry) | No | Not currently exposed by this extension. |
| [`st_geometryn`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeometryN-Geometry-index-Geometry) | No | Not currently exposed by this extension. |
| [`st_geometrytype`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeometryType-Geometry-varchar) | No | Not currently exposed by this extension. |
| [`st_geomfrombinary`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeomFromBinary-varbinary-Geometry) | No | Not currently exposed by this extension. |
| [`st_interiorringn`](https://prestodb.io/docs/current/functions/geospatial.html#ST_InteriorRingN-Geometry-index-Geometry) | No | Not currently exposed by this extension. |
| [`st_interiorrings`](https://prestodb.io/docs/current/functions/geospatial.html#ST_InteriorRings) | No | Not currently exposed by this extension. |
| [`st_intersection`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Intersection-Geometry-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_intersects`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Intersects-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_isclosed`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsClosed-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_isempty`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsEmpty-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_isring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsRing-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_issimple`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsSimple-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_isvalid`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsValid-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_length`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Length-Geometry-double) | No | Not currently exposed by this extension. |
| [`st_linefromtext`](https://prestodb.io/docs/current/functions/geospatial.html#ST_LineFromText-varchar-LineString) | No | Not currently exposed by this extension. |
| [`st_linestring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_LineString-array-Point-LineString) | No | Not currently exposed by this extension. |
| [`st_multipoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_MultiPoint-array-Point-MultiPoint) | No | Not currently exposed by this extension. |
| [`st_numgeometries`](https://prestodb.io/docs/current/functions/geospatial.html#ST_NumGeometries-Geometry-bigint) | No | Not currently exposed by this extension. |
| [`st_numinteriorring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_NumInteriorRing-Geometry-bigint) | No | Not currently exposed by this extension. |
| [`st_numpoints`](https://prestodb.io/docs/current/functions/geospatial.html#ST_NumPoints-Geometry-bigint) | No | Not currently exposed by this extension. |
| [`st_overlaps`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Overlaps-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_point`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Point-x-y-Point) | No | Not currently exposed by this extension. |
| [`st_pointn`](https://prestodb.io/docs/current/functions/geospatial.html#ST_PointN-LineString-index-Point) | No | Not currently exposed by this extension. |
| [`st_points`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Points) | No | Not currently exposed by this extension. |
| [`st_polygon`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Polygon-varchar-Polygon) | No | Not currently exposed by this extension. |
| [`st_relate`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Relate-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_startpoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_StartPoint-Geometry-point) | No | Not currently exposed by this extension. |
| [`st_symdifference`](https://prestodb.io/docs/current/functions/geospatial.html#ST_SymDifference-Geometry-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_touches`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Touches-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_union`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Union-Geometry-Geometry-Geometry) | No | Not currently exposed by this extension. |
| [`st_within`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Within-Geometry-Geometry-boolean) | No | Not currently exposed by this extension. |
| [`st_x`](https://prestodb.io/docs/current/functions/geospatial.html#ST_X-Point-double) | No | Not currently exposed by this extension. |
| [`st_xmax`](https://prestodb.io/docs/current/functions/geospatial.html#ST_XMax-Geometry-double) | No | Not currently exposed by this extension. |
| [`st_xmin`](https://prestodb.io/docs/current/functions/geospatial.html#ST_XMin-Geometry-double) | No | Not currently exposed by this extension. |
| [`st_y`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Y-Point-double) | No | Not currently exposed by this extension. |
| [`st_ymax`](https://prestodb.io/docs/current/functions/geospatial.html#ST_YMax-Geometry-double) | No | Not currently exposed by this extension. |
| [`st_ymin`](https://prestodb.io/docs/current/functions/geospatial.html#ST_YMin-Geometry-double) | No | Not currently exposed by this extension. |
| [`to_geometry`](https://prestodb.io/docs/current/functions/geospatial.html#to_geometry-SphericalGeography-Geometry) | No | Not currently exposed by this extension. |
| [`to_spherical_geography`](https://prestodb.io/docs/current/functions/geospatial.html#to_spherical_geography-Geometry-SphericalGeography) | No | Not currently exposed by this extension. |

### HyperLogLog Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`empty_approx_set`](https://prestodb.io/docs/current/functions/hyperloglog.html#empty_approx_set-HyperLogLog) | No | Not currently exposed by this extension. |
| [`merge_hll`](https://prestodb.io/docs/current/functions/hyperloglog.html#merge_hll-array-HyperLogLog-HyperLogLog) | No | Not currently exposed by this extension. |

### KHyperLogLog Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`intersection_cardinality`](https://prestodb.io/docs/current/functions/khyperloglog.html#intersection_cardinality-khll1-khll2-bigint) | No | Not currently exposed by this extension. |
| [`jaccard_index`](https://prestodb.io/docs/current/functions/khyperloglog.html#jaccard_index-khll1-khll2-double) | No | Not currently exposed by this extension. |
| [`merge_khll`](https://prestodb.io/docs/current/functions/khyperloglog.html#merge_khll-array-khll-KHyperLogLog) | No | Not currently exposed by this extension. |
| [`reidentification_potential`](https://prestodb.io/docs/current/functions/khyperloglog.html#reidentification_potential-khll-threshold-double) | No | Not currently exposed by this extension. |
| [`uniqueness_distribution`](https://prestodb.io/docs/current/functions/khyperloglog.html#uniqueness_distribution-khll-map-bigint-double) | No | Not currently exposed by this extension. |

### Quantile Digest Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`quantile_at_value`](https://prestodb.io/docs/current/functions/qdigest.html#quantile_at_value-qdigest-T-T-quantile) | No | Not currently exposed by this extension. |
| [`scale_qdigest`](https://prestodb.io/docs/current/functions/qdigest.html#scale_qdigest) | No | Not currently exposed by this extension. |
| [`value_at_quantile`](https://prestodb.io/docs/current/functions/qdigest.html#value_at_quantile-qdigest-T-quantile-T) | No | Not currently exposed by this extension. |
| [`values_at_quantiles`](https://prestodb.io/docs/current/functions/qdigest.html#values_at_quantiles-qdigest-T-quantiles-T) | No | Not currently exposed by this extension. |

### UUID functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`uuid`](https://prestodb.io/docs/current/functions/uuid.html#uuid-uuid) | Supported | Non-deterministic (generates a new UUID each call). |

### T-Digest Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`construct_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#construct_tdigest-centroid_means-array-double-centroid_weights-array-double-compression-double-min-double-max-double-sum-double-count-bigint-tdigest-double) | No | Not currently exposed by this extension. |
| [`destructure_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#destructure_tdigest-tdigest-double-row-centroid_means-array-double-centroid_weights-array-integer-compression-double-min-double-max-double-sum-double-count-bigint) | No | Not currently exposed by this extension. |
| [`merge_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#merge_tdigest-array-tdigest-double-tdigest-double) | No | Not currently exposed by this extension. |
| [`quantiles_at_values`](https://prestodb.io/docs/current/functions/tdigest.html#quantiles_at_values-tdigest-double-values-array-double) | No | Not currently exposed by this extension. |
| [`scale_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#scale_tdigest-tdigest-double-scale_factor-tdigest-double) | No | Not currently exposed by this extension. |
| [`tdigest_agg`](https://prestodb.io/docs/current/functions/tdigest.html#tdigest_agg-x-tdigest-double) | No | Not currently exposed by this extension. |
| [`trimmed_mean`](https://prestodb.io/docs/current/functions/tdigest.html#trimmed_mean-tdigest-double-lower_quantile-upper_quantile-double) | No | Not currently exposed by this extension. |

### Color Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`bar`](https://prestodb.io/docs/current/functions/color.html#bar-x-width-varchar) | No | Not currently exposed by this extension. |
| [`color`](https://prestodb.io/docs/current/functions/color.html#color-string-color) | No | Not currently exposed by this extension. |
| [`render`](https://prestodb.io/docs/current/functions/color.html#render-x-color-varchar) | No | Not currently exposed by this extension. |
| [`rgb`](https://prestodb.io/docs/current/functions/color.html#rgb-red-green-blue-color) | No | Not currently exposed by this extension. |

### Session Information

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`current_user`](https://prestodb.io/docs/current/functions/session.html#current_user-varchar) | No | Not currently exposed by this extension. |

### Teradata Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`char2hexint`](https://prestodb.io/docs/current/functions/teradata.html#char2hexint-string-varchar) | No | Not currently exposed by this extension. |
| [`index`](https://prestodb.io/docs/current/functions/teradata.html#index-string-substring-bigint) | No | Not currently exposed by this extension. |
| [`substring`](https://prestodb.io/docs/current/functions/teradata.html#substring-string-start-varchar) | No | Not currently exposed by this extension. |
| [`to_char`](https://prestodb.io/docs/current/functions/teradata.html#to_char-timestamp-format-varchar) | No | Not currently exposed by this extension. |
| [`to_date`](https://prestodb.io/docs/current/functions/teradata.html#to_date-string-format-date) | No | Not currently exposed by this extension. |
| [`to_timestamp`](https://prestodb.io/docs/current/functions/teradata.html#to_timestamp-string-format-timestamp) | No | Not currently exposed by this extension. |

### Internationalization Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`myanmar_font_encoding`](https://prestodb.io/docs/current/functions/internationalization.html#myanmar_font_encoding-text-varchar) | No | Not currently exposed by this extension. |
| [`myanmar_normalize_unicode`](https://prestodb.io/docs/current/functions/internationalization.html#myanmar_normalize_unicode-text-varchar) | No | Not currently exposed by this extension. |

### Set Digest functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`hash_counts`](https://prestodb.io/docs/current/functions/setdigest.html#hash_counts) | No | Not currently exposed by this extension. |
| [`make_set_digest`](https://prestodb.io/docs/current/functions/setdigest.html#make_set_digest-x-setdigest) | No | Not currently exposed by this extension. |
| [`merge_set_digest`](https://prestodb.io/docs/current/functions/setdigest.html#merge_set_digest-setdigest-setdigest) | No | Not currently exposed by this extension. |

### Sketch Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`sketch_kll`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll-T-x-T-kllsketch-T) | No | Not currently exposed by this extension. |
| [`sketch_kll_quantile`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll_quantile-T-sketch-kllsketch-T-rank-double-inclusivity-boolean-T) | No | Not currently exposed by this extension. |
| [`sketch_kll_rank`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll_rank-T-sketch-kllsketch-T-quantile-T-inclusivity-boolean-double) | No | Not currently exposed by this extension. |
| [`sketch_kll_with_k`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll_with_k-T-x-T-k-int-kllsketch-T) | No | Not currently exposed by this extension. |
| [`sketch_theta`](https://prestodb.io/docs/current/functions/sketch.html#sketch_theta-x-varbinary) | No | Not currently exposed by this extension. |
| [`sketch_theta_estimate`](https://prestodb.io/docs/current/functions/sketch.html#sketch_theta_estimate-sketch-double) | No | Not currently exposed by this extension. |
| [`sketch_theta_summary`](https://prestodb.io/docs/current/functions/sketch.html#sketch_theta_summary) | No | Not currently exposed by this extension. |

### Pinot Functions

| Function (Presto docs) | SQLite extension status | Comments |
|---|---|---|
| [`pinot_binary_decimal_to_double`](https://prestodb.io/docs/current/functions/pinot.html#pinot_binary_decimal_to_double-binary-bigIntegerRadix-scale-returnZeroOnNull-double) | No | Not currently exposed by this extension. |

**Summary:** 510 Presto functions total, 162 supported, 14 stubbed, 334 not exposed.
