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

This table tracks the **full current Presto function list** from the official docs and maps each function to SQLite extension support status.

- `Supported`: exposed and bridged in this SQLite extension
- `Stubbed`: exposed as `p_<function>` but SQLite bridge is not implemented yet
- `No`: not currently exposed by this extension

> Note: if a function appears in multiple Presto categories, this table keeps a single row using its first category in the Presto functions index.

| Category | Function (Presto docs) | SQLite extension status |
|---|---|---|
| Comparison Functions and Operators | [`greatest`](https://prestodb.io/docs/current/functions/comparison.html#greatest-value1-value2-...-valueN-same-as-input) | No |
| Comparison Functions and Operators | [`least`](https://prestodb.io/docs/current/functions/comparison.html#least-value1-value2-...-valueN-same-as-input) | No |
| Conditional Expressions | [`coalesce`](https://prestodb.io/docs/current/functions/conditional.html) | No |
| Conditional Expressions | [`if`](https://prestodb.io/docs/current/functions/conditional.html) | No |
| Conditional Expressions | [`nullif`](https://prestodb.io/docs/current/functions/conditional.html) | No |
| Conditional Expressions | [`try`](https://prestodb.io/docs/current/functions/conditional.html) | No |
| Conversion Functions | [`cast`](https://prestodb.io/docs/current/functions/conversion.html#cast-value-AS-type-type) | No |
| Conversion Functions | [`parse_presto_data_size`](https://prestodb.io/docs/current/functions/conversion.html#parse_presto_data_size) | No |
| Conversion Functions | [`try_cast`](https://prestodb.io/docs/current/functions/conversion.html#try_cast-value-AS-type-type) | No |
| Conversion Functions | [`typeof`](https://prestodb.io/docs/current/functions/conversion.html#typeof-expr-varchar) | No |
| Mathematical Functions and Operators | [`abs`](https://prestodb.io/docs/current/functions/math.html#abs-x-same-as-input) | Supported |
| Mathematical Functions and Operators | [`acos`](https://prestodb.io/docs/current/functions/math.html#acos-x-double) | Supported |
| Mathematical Functions and Operators | [`asin`](https://prestodb.io/docs/current/functions/math.html#asin-x-double) | Supported |
| Mathematical Functions and Operators | [`atan`](https://prestodb.io/docs/current/functions/math.html#atan-x-double) | Supported |
| Mathematical Functions and Operators | [`atan2`](https://prestodb.io/docs/current/functions/math.html#atan2-y-x-double) | Supported |
| Mathematical Functions and Operators | [`beta_cdf`](https://prestodb.io/docs/current/functions/math.html#beta_cdf-a-b-value-double) | Supported |
| Mathematical Functions and Operators | [`binomial_cdf`](https://prestodb.io/docs/current/functions/math.html#binomial_cdf-numberOfTrials-successProbability-value-double) | Supported |
| Mathematical Functions and Operators | [`cauchy_cdf`](https://prestodb.io/docs/current/functions/math.html#cauchy_cdf-median-scale-value-double) | Supported |
| Mathematical Functions and Operators | [`cbrt`](https://prestodb.io/docs/current/functions/math.html#cbrt-x-double) | Supported |
| Mathematical Functions and Operators | [`ceil`](https://prestodb.io/docs/current/functions/math.html#ceil-x-same-as-input) | Supported |
| Mathematical Functions and Operators | [`ceiling`](https://prestodb.io/docs/current/functions/math.html#ceiling-x-same-as-input) | Supported |
| Mathematical Functions and Operators | [`chi_squared_cdf`](https://prestodb.io/docs/current/functions/math.html#chi_squared_cdf-df-value-double) | Supported |
| Mathematical Functions and Operators | [`cos`](https://prestodb.io/docs/current/functions/math.html#cos-x-double) | Supported |
| Mathematical Functions and Operators | [`cosh`](https://prestodb.io/docs/current/functions/math.html#cosh-x-double) | Supported |
| Mathematical Functions and Operators | [`cosine_similarity`](https://prestodb.io/docs/current/functions/math.html#cosine_similarity-x-y-double) | Stubbed |
| Mathematical Functions and Operators | [`degrees`](https://prestodb.io/docs/current/functions/math.html#degrees-x-double) | Supported |
| Mathematical Functions and Operators | [`dot_product`](https://prestodb.io/docs/current/functions/math.html#dot_product-array-real-array-real-real) | Stubbed |
| Mathematical Functions and Operators | [`e`](https://prestodb.io/docs/current/functions/math.html#e-double) | Supported |
| Mathematical Functions and Operators | [`exp`](https://prestodb.io/docs/current/functions/math.html#exp-x-double) | Supported |
| Mathematical Functions and Operators | [`f_cdf`](https://prestodb.io/docs/current/functions/math.html#f_cdf-df1-df2-value-double) | Supported |
| Mathematical Functions and Operators | [`factorial`](https://prestodb.io/docs/current/functions/math.html#factorial-x-bigint) | Supported |
| Mathematical Functions and Operators | [`floor`](https://prestodb.io/docs/current/functions/math.html#floor-x-same-as-input) | Supported |
| Mathematical Functions and Operators | [`from_base`](https://prestodb.io/docs/current/functions/math.html#from_base-string-radix-bigint) | Supported |
| Mathematical Functions and Operators | [`gamma_cdf`](https://prestodb.io/docs/current/functions/math.html#gamma_cdf-shape-scale-value-double) | Supported |
| Mathematical Functions and Operators | [`infinity`](https://prestodb.io/docs/current/functions/math.html#infinity-double) | Supported |
| Mathematical Functions and Operators | [`inverse_beta_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_beta_cdf-a-b-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_binomial_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_binomial_cdf-numberOfTrials-successProbability-p-int) | Supported |
| Mathematical Functions and Operators | [`inverse_cauchy_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_cauchy_cdf-median-scale-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_chi_squared_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_chi_squared_cdf-df-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_f_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_f_cdf-df1-df2-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_gamma_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_gamma_cdf-shape-scale-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_laplace_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_laplace_cdf-mean-scale-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_normal_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_normal_cdf-mean-sd-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_poisson_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_poisson_cdf-lambda-p-integer) | Supported |
| Mathematical Functions and Operators | [`inverse_t_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_t_cdf-df-p-double) | Supported |
| Mathematical Functions and Operators | [`inverse_weibull_cdf`](https://prestodb.io/docs/current/functions/math.html#inverse_weibull_cdf-a-b-p-double) | Supported |
| Mathematical Functions and Operators | [`is_finite`](https://prestodb.io/docs/current/functions/math.html#is_finite-x-boolean) | Supported |
| Mathematical Functions and Operators | [`is_infinite`](https://prestodb.io/docs/current/functions/math.html#is_infinite-x-boolean) | Supported |
| Mathematical Functions and Operators | [`is_nan`](https://prestodb.io/docs/current/functions/math.html#is_nan-x-boolean) | Supported |
| Mathematical Functions and Operators | [`l2_squared`](https://prestodb.io/docs/current/functions/math.html#l2_squared-array-real-array-real-real) | Stubbed |
| Mathematical Functions and Operators | [`laplace_cdf`](https://prestodb.io/docs/current/functions/math.html#laplace_cdf-mean-scale-value-double) | Supported |
| Mathematical Functions and Operators | [`ln`](https://prestodb.io/docs/current/functions/math.html#ln-x-double) | Supported |
| Mathematical Functions and Operators | [`log10`](https://prestodb.io/docs/current/functions/math.html#log10-x-double) | Supported |
| Mathematical Functions and Operators | [`log2`](https://prestodb.io/docs/current/functions/math.html#log2-x-double) | Supported |
| Mathematical Functions and Operators | [`mod`](https://prestodb.io/docs/current/functions/math.html#mod-n-m-same-as-input) | Supported |
| Mathematical Functions and Operators | [`nan`](https://prestodb.io/docs/current/functions/math.html#nan-double) | Supported |
| Mathematical Functions and Operators | [`normal_cdf`](https://prestodb.io/docs/current/functions/math.html#normal_cdf-mean-sd-value-double) | Supported |
| Mathematical Functions and Operators | [`pi`](https://prestodb.io/docs/current/functions/math.html#pi-double) | Supported |
| Mathematical Functions and Operators | [`poisson_cdf`](https://prestodb.io/docs/current/functions/math.html#poisson_cdf-lambda-value-double) | Supported |
| Mathematical Functions and Operators | [`pow`](https://prestodb.io/docs/current/functions/math.html#pow-x-p-double) | Supported |
| Mathematical Functions and Operators | [`power`](https://prestodb.io/docs/current/functions/math.html#power-x-p-double) | Supported |
| Mathematical Functions and Operators | [`radians`](https://prestodb.io/docs/current/functions/math.html#radians-x-double) | Supported |
| Mathematical Functions and Operators | [`rand`](https://prestodb.io/docs/current/functions/math.html#rand-double) | Supported |
| Mathematical Functions and Operators | [`random`](https://prestodb.io/docs/current/functions/math.html#random-double) | Supported |
| Mathematical Functions and Operators | [`round`](https://prestodb.io/docs/current/functions/math.html#round-x-same-as-input) | Supported |
| Mathematical Functions and Operators | [`secure_rand`](https://prestodb.io/docs/current/functions/math.html#secure_rand-double) | Supported |
| Mathematical Functions and Operators | [`secure_random`](https://prestodb.io/docs/current/functions/math.html#secure_random-double) | Supported |
| Mathematical Functions and Operators | [`sign`](https://prestodb.io/docs/current/functions/math.html#sign-x-same-as-input) | Supported |
| Mathematical Functions and Operators | [`sin`](https://prestodb.io/docs/current/functions/math.html#sin-x-double) | Supported |
| Mathematical Functions and Operators | [`sqrt`](https://prestodb.io/docs/current/functions/math.html#sqrt-x-double) | Supported |
| Mathematical Functions and Operators | [`t_cdf`](https://prestodb.io/docs/current/functions/math.html#t_cdf-df-value-double) | Supported |
| Mathematical Functions and Operators | [`tan`](https://prestodb.io/docs/current/functions/math.html#tan-x-double) | Supported |
| Mathematical Functions and Operators | [`tanh`](https://prestodb.io/docs/current/functions/math.html#tanh-x-double) | Supported |
| Mathematical Functions and Operators | [`to_base`](https://prestodb.io/docs/current/functions/math.html#to_base-x-radix-varchar) | Supported |
| Mathematical Functions and Operators | [`truncate`](https://prestodb.io/docs/current/functions/math.html#truncate-x-double) | Supported |
| Mathematical Functions and Operators | [`weibull_cdf`](https://prestodb.io/docs/current/functions/math.html#weibull_cdf-a-b-value-double) | Supported |
| Mathematical Functions and Operators | [`width_bucket`](https://prestodb.io/docs/current/functions/math.html#width_bucket-x-bound1-bound2-n-bigint) | Supported |
| Mathematical Functions and Operators | [`wilson_interval_lower`](https://prestodb.io/docs/current/functions/math.html#wilson_interval_lower-successes-trials-z-double) | Supported |
| Mathematical Functions and Operators | [`wilson_interval_upper`](https://prestodb.io/docs/current/functions/math.html#wilson_interval_upper-successes-trials-z-double) | Supported |
| Bitwise Functions | [`bit_count`](https://prestodb.io/docs/current/functions/bitwise.html#bit_count-x-bits-bigint) | Supported |
| Bitwise Functions | [`bitwise_and`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_and-x-y-bigint) | Supported |
| Bitwise Functions | [`bitwise_arithmetic_shift_right`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_arithmetic_shift_right-x-shift-bigint) | Supported |
| Bitwise Functions | [`bitwise_left_shift`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_left_shift-value-shift-same-as-value) | Supported |
| Bitwise Functions | [`bitwise_logical_shift_right`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_logical_shift_right-x-shift-bits-bigint) | Supported |
| Bitwise Functions | [`bitwise_not`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_not-x-bigint) | Supported |
| Bitwise Functions | [`bitwise_or`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_or-x-y-bigint) | Supported |
| Bitwise Functions | [`bitwise_right_shift`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_right_shift-value-shift-same-as-value) | Supported |
| Bitwise Functions | [`bitwise_right_shift_arithmetic`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_right_shift_arithmetic-value-shift-same-as-value) | Supported |
| Bitwise Functions | [`bitwise_shift_left`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_shift_left-x-shift-bits-bigint) | Supported |
| Bitwise Functions | [`bitwise_xor`](https://prestodb.io/docs/current/functions/bitwise.html#bitwise_xor-x-y-bigint) | Supported |
| String Functions and Operators | [`bit_length`](https://prestodb.io/docs/current/functions/string.html#bit_length-string-boolean) | Supported |
| String Functions and Operators | [`chr`](https://prestodb.io/docs/current/functions/string.html#chr-n-varchar) | Supported |
| String Functions and Operators | [`codepoint`](https://prestodb.io/docs/current/functions/string.html#codepoint-string-integer) | Supported |
| String Functions and Operators | [`concat`](https://prestodb.io/docs/current/functions/string.html#concat-string1-...-stringN-varchar) | Supported |
| String Functions and Operators | [`ends_with`](https://prestodb.io/docs/current/functions/string.html#ends_with-string-substring-boolean) | Supported |
| String Functions and Operators | [`from_utf8`](https://prestodb.io/docs/current/functions/string.html#from_utf8-binary-varchar) | Supported |
| String Functions and Operators | [`hamming_distance`](https://prestodb.io/docs/current/functions/string.html#hamming_distance-string1-string2-bigint) | Supported |
| String Functions and Operators | [`jarowinkler_similarity`](https://prestodb.io/docs/current/functions/string.html#jarowinkler_similarity-string1-string2-double) | Supported |
| String Functions and Operators | [`key_sampling_percent`](https://prestodb.io/docs/current/functions/string.html#key_sampling_percent-varchar-double) | Supported |
| String Functions and Operators | [`length`](https://prestodb.io/docs/current/functions/string.html#length-string-bigint) | Supported |
| String Functions and Operators | [`levenshtein_distance`](https://prestodb.io/docs/current/functions/string.html#levenshtein_distance-string1-string2-bigint) | Supported |
| String Functions and Operators | [`longest_common_prefix`](https://prestodb.io/docs/current/functions/string.html#longest_common_prefix-string1-string2-varchar) | Supported |
| String Functions and Operators | [`lower`](https://prestodb.io/docs/current/functions/string.html#lower-string-varchar) | Supported |
| String Functions and Operators | [`lpad`](https://prestodb.io/docs/current/functions/string.html#lpad-string-size-padstring-varchar) | Supported |
| String Functions and Operators | [`ltrim`](https://prestodb.io/docs/current/functions/string.html#ltrim-string-varchar) | Supported |
| String Functions and Operators | [`normalize`](https://prestodb.io/docs/current/functions/string.html#normalize-string-varchar) | Supported |
| String Functions and Operators | [`position`](https://prestodb.io/docs/current/functions/string.html#position-substring-IN-string-bigint) | No |
| String Functions and Operators | [`replace`](https://prestodb.io/docs/current/functions/string.html#replace-string-search-varchar) | Supported |
| String Functions and Operators | [`replace_first`](https://prestodb.io/docs/current/functions/string.html#replace_first-string-search-replace-varchar) | Supported |
| String Functions and Operators | [`reverse`](https://prestodb.io/docs/current/functions/string.html#reverse-string-varchar) | Supported |
| String Functions and Operators | [`rpad`](https://prestodb.io/docs/current/functions/string.html#rpad-string-size-padstring-varchar) | Supported |
| String Functions and Operators | [`rtrim`](https://prestodb.io/docs/current/functions/string.html#rtrim-string-varchar) | Supported |
| String Functions and Operators | [`split`](https://prestodb.io/docs/current/functions/string.html#split) | Supported |
| String Functions and Operators | [`split_part`](https://prestodb.io/docs/current/functions/string.html#split_part-string-delimiter-index-varchar) | Supported |
| String Functions and Operators | [`split_to_map`](https://prestodb.io/docs/current/functions/string.html#split_to_map-string-entryDelimiter-keyValueDelimiter-map-varchar-varchar) | No |
| String Functions and Operators | [`split_to_multimap`](https://prestodb.io/docs/current/functions/string.html#split_to_multimap) | Supported |
| String Functions and Operators | [`starts_with`](https://prestodb.io/docs/current/functions/string.html#starts_with-string-substring-boolean) | Supported |
| String Functions and Operators | [`strpos`](https://prestodb.io/docs/current/functions/string.html#strpos-string-substring-bigint) | Supported |
| String Functions and Operators | [`strrpos`](https://prestodb.io/docs/current/functions/string.html#strrpos-string-substring-bigint) | Supported |
| String Functions and Operators | [`substr`](https://prestodb.io/docs/current/functions/string.html#substr-string-start-varchar) | Supported |
| String Functions and Operators | [`to_utf8`](https://prestodb.io/docs/current/functions/string.html#to_utf8-string-varbinary) | Supported |
| String Functions and Operators | [`trail`](https://prestodb.io/docs/current/functions/string.html#trail-string-N-varchar) | Supported |
| String Functions and Operators | [`trim`](https://prestodb.io/docs/current/functions/string.html#trim-string-varchar) | Supported |
| String Functions and Operators | [`upper`](https://prestodb.io/docs/current/functions/string.html#upper-string-varchar) | Supported |
| String Functions and Operators | [`word_stem`](https://prestodb.io/docs/current/functions/string.html#word_stem-word-varchar) | Supported |
| Regular Expression Functions | [`regexp_extract`](https://prestodb.io/docs/current/functions/regexp.html#regexp_extract-string-pattern-varchar) | Supported |
| Regular Expression Functions | [`regexp_extract_all`](https://prestodb.io/docs/current/functions/regexp.html#regexp_extract_all) | Supported |
| Regular Expression Functions | [`regexp_like`](https://prestodb.io/docs/current/functions/regexp.html#regexp_like-string-pattern-boolean) | Supported |
| Regular Expression Functions | [`regexp_replace`](https://prestodb.io/docs/current/functions/regexp.html#regexp_replace-string-pattern-varchar) | Supported |
| Regular Expression Functions | [`regexp_split`](https://prestodb.io/docs/current/functions/regexp.html#regexp_split) | Supported |
| Binary Functions and Operators | [`crc32`](https://prestodb.io/docs/current/functions/binary.html#crc32-binary-bigint) | Supported |
| Binary Functions and Operators | [`from_base32`](https://prestodb.io/docs/current/functions/binary.html#from_base32-string-varbinary) | Supported |
| Binary Functions and Operators | [`from_base64`](https://prestodb.io/docs/current/functions/binary.html#from_base64-string-varbinary) | Supported |
| Binary Functions and Operators | [`from_base64url`](https://prestodb.io/docs/current/functions/binary.html#from_base64url-string-varbinary) | Supported |
| Binary Functions and Operators | [`from_big_endian_32`](https://prestodb.io/docs/current/functions/binary.html#from_big_endian_32-binary-integer) | Supported |
| Binary Functions and Operators | [`from_big_endian_64`](https://prestodb.io/docs/current/functions/binary.html#from_big_endian_64-binary-bigint) | Supported |
| Binary Functions and Operators | [`from_hex`](https://prestodb.io/docs/current/functions/binary.html#from_hex-string-varbinary) | Supported |
| Binary Functions and Operators | [`from_ieee754_32`](https://prestodb.io/docs/current/functions/binary.html#from_ieee754_32-binary-real) | Supported |
| Binary Functions and Operators | [`from_ieee754_64`](https://prestodb.io/docs/current/functions/binary.html#from_ieee754_64-binary-double) | Supported |
| Binary Functions and Operators | [`hmac_md5`](https://prestodb.io/docs/current/functions/binary.html#hmac_md5-binary-key-varbinary) | Supported |
| Binary Functions and Operators | [`hmac_sha1`](https://prestodb.io/docs/current/functions/binary.html#hmac_sha1-binary-key-varbinary) | Supported |
| Binary Functions and Operators | [`hmac_sha256`](https://prestodb.io/docs/current/functions/binary.html#hmac_sha256-binary-key-varbinary) | Supported |
| Binary Functions and Operators | [`hmac_sha512`](https://prestodb.io/docs/current/functions/binary.html#hmac_sha512-binary-key-varbinary) | Supported |
| Binary Functions and Operators | [`md5`](https://prestodb.io/docs/current/functions/binary.html#md5-binary-varbinary) | Supported |
| Binary Functions and Operators | [`murmur3_x64_128`](https://prestodb.io/docs/current/functions/binary.html#murmur3_x64_128-binary-varbinary) | Supported |
| Binary Functions and Operators | [`sha1`](https://prestodb.io/docs/current/functions/binary.html#sha1-binary-varbinary) | Supported |
| Binary Functions and Operators | [`sha256`](https://prestodb.io/docs/current/functions/binary.html#sha256-binary-varbinary) | Supported |
| Binary Functions and Operators | [`sha512`](https://prestodb.io/docs/current/functions/binary.html#sha512-binary-varbinary) | Supported |
| Binary Functions and Operators | [`spooky_hash_v2_32`](https://prestodb.io/docs/current/functions/binary.html#spooky_hash_v2_32-binary-varbinary) | Supported |
| Binary Functions and Operators | [`spooky_hash_v2_64`](https://prestodb.io/docs/current/functions/binary.html#spooky_hash_v2_64-binary-varbinary) | Supported |
| Binary Functions and Operators | [`to_base32`](https://prestodb.io/docs/current/functions/binary.html#to_base32-binary-varchar) | Supported |
| Binary Functions and Operators | [`to_base64`](https://prestodb.io/docs/current/functions/binary.html#to_base64-binary-varchar) | Supported |
| Binary Functions and Operators | [`to_base64url`](https://prestodb.io/docs/current/functions/binary.html#to_base64url-binary-varchar) | Supported |
| Binary Functions and Operators | [`to_big_endian_32`](https://prestodb.io/docs/current/functions/binary.html#to_big_endian_32-integer-varbinary) | Supported |
| Binary Functions and Operators | [`to_big_endian_64`](https://prestodb.io/docs/current/functions/binary.html#to_big_endian_64-bigint-varbinary) | Supported |
| Binary Functions and Operators | [`to_hex`](https://prestodb.io/docs/current/functions/binary.html#to_hex-binary-varchar) | Supported |
| Binary Functions and Operators | [`to_ieee754_32`](https://prestodb.io/docs/current/functions/binary.html#to_ieee754_32-real-varbinary) | Supported |
| Binary Functions and Operators | [`to_ieee754_64`](https://prestodb.io/docs/current/functions/binary.html#to_ieee754_64-double-varbinary) | Supported |
| Binary Functions and Operators | [`xxhash64`](https://prestodb.io/docs/current/functions/binary.html#xxhash64-binary-varbinary) | Supported |
| JSON Functions and Operators | [`is_json_scalar`](https://prestodb.io/docs/current/functions/json.html#is_json_scalar-json-boolean) | No |
| JSON Functions and Operators | [`json_array_contains`](https://prestodb.io/docs/current/functions/json.html#json_array_contains-json-value-boolean) | No |
| JSON Functions and Operators | [`json_array_get`](https://prestodb.io/docs/current/functions/json.html#json_array_get-json_array-index-json) | No |
| JSON Functions and Operators | [`json_array_length`](https://prestodb.io/docs/current/functions/json.html#json_array_length-json-bigint) | No |
| JSON Functions and Operators | [`json_extract`](https://prestodb.io/docs/current/functions/json.html#json_extract-json-json_path-json) | No |
| JSON Functions and Operators | [`json_extract_scalar`](https://prestodb.io/docs/current/functions/json.html#json_extract_scalar-json-json_path-varchar) | No |
| JSON Functions and Operators | [`json_format`](https://prestodb.io/docs/current/functions/json.html#json_format-json-varchar) | No |
| JSON Functions and Operators | [`json_parse`](https://prestodb.io/docs/current/functions/json.html#json_parse-string-json) | No |
| JSON Functions and Operators | [`json_size`](https://prestodb.io/docs/current/functions/json.html#json_size-json-json_path-bigint) | No |
| Date and Time Functions and Operators | [`current_date`](https://prestodb.io/docs/current/functions/datetime.html#current_date-date) | No |
| Date and Time Functions and Operators | [`current_time`](https://prestodb.io/docs/current/functions/datetime.html#current_time-time-with-time-zone) | No |
| Date and Time Functions and Operators | [`current_timestamp`](https://prestodb.io/docs/current/functions/datetime.html#current_timestamp-timestamp-with-time-zone) | No |
| Date and Time Functions and Operators | [`current_timezone`](https://prestodb.io/docs/current/functions/datetime.html#current_timezone-varchar) | No |
| Date and Time Functions and Operators | [`date`](https://prestodb.io/docs/current/functions/datetime.html#date-x-date) | No |
| Date and Time Functions and Operators | [`date_add`](https://prestodb.io/docs/current/functions/datetime.html#date_add-unit-value-timestamp-same-as-input) | No |
| Date and Time Functions and Operators | [`date_diff`](https://prestodb.io/docs/current/functions/datetime.html#date_diff-unit-timestamp1-timestamp2-bigint) | No |
| Date and Time Functions and Operators | [`date_format`](https://prestodb.io/docs/current/functions/datetime.html#date_format-timestamp-format-varchar) | No |
| Date and Time Functions and Operators | [`date_parse`](https://prestodb.io/docs/current/functions/datetime.html#date_parse-string-format-timestamp) | No |
| Date and Time Functions and Operators | [`date_trunc`](https://prestodb.io/docs/current/functions/datetime.html#date_trunc-unit-x-same-as-input) | No |
| Date and Time Functions and Operators | [`day`](https://prestodb.io/docs/current/functions/datetime.html#day-x-bigint) | No |
| Date and Time Functions and Operators | [`day_of_month`](https://prestodb.io/docs/current/functions/datetime.html#day_of_month-x-bigint) | No |
| Date and Time Functions and Operators | [`day_of_week`](https://prestodb.io/docs/current/functions/datetime.html#day_of_week-x-bigint) | No |
| Date and Time Functions and Operators | [`day_of_year`](https://prestodb.io/docs/current/functions/datetime.html#day_of_year-x-bigint) | No |
| Date and Time Functions and Operators | [`dow`](https://prestodb.io/docs/current/functions/datetime.html#dow-x-bigint) | No |
| Date and Time Functions and Operators | [`doy`](https://prestodb.io/docs/current/functions/datetime.html#doy-x-bigint) | No |
| Date and Time Functions and Operators | [`extract`](https://prestodb.io/docs/current/functions/datetime.html#extract-field-FROM-x-bigint) | No |
| Date and Time Functions and Operators | [`format_datetime`](https://prestodb.io/docs/current/functions/datetime.html#format_datetime-timestamp-format-varchar) | No |
| Date and Time Functions and Operators | [`from_iso8601_date`](https://prestodb.io/docs/current/functions/datetime.html#from_iso8601_date-string-date) | No |
| Date and Time Functions and Operators | [`from_iso8601_timestamp`](https://prestodb.io/docs/current/functions/datetime.html#from_iso8601_timestamp-string-timestamp-with-time-zone) | No |
| Date and Time Functions and Operators | [`from_unixtime`](https://prestodb.io/docs/current/functions/datetime.html#from_unixtime-unixtime-timestamp) | No |
| Date and Time Functions and Operators | [`hour`](https://prestodb.io/docs/current/functions/datetime.html#hour-x-bigint) | No |
| Date and Time Functions and Operators | [`last_day_of_month`](https://prestodb.io/docs/current/functions/datetime.html#last_day_of_month-x-date) | No |
| Date and Time Functions and Operators | [`localtime`](https://prestodb.io/docs/current/functions/datetime.html#localtime-time) | No |
| Date and Time Functions and Operators | [`localtimestamp`](https://prestodb.io/docs/current/functions/datetime.html#localtimestamp-timestamp) | No |
| Date and Time Functions and Operators | [`millisecond`](https://prestodb.io/docs/current/functions/datetime.html#millisecond-x-bigint) | No |
| Date and Time Functions and Operators | [`minute`](https://prestodb.io/docs/current/functions/datetime.html#minute-x-bigint) | No |
| Date and Time Functions and Operators | [`month`](https://prestodb.io/docs/current/functions/datetime.html#month-x-bigint) | No |
| Date and Time Functions and Operators | [`now`](https://prestodb.io/docs/current/functions/datetime.html#now-timestamp-with-time-zone) | No |
| Date and Time Functions and Operators | [`parse_datetime`](https://prestodb.io/docs/current/functions/datetime.html#parse_datetime-string-format-timestamp-with-time-zone) | No |
| Date and Time Functions and Operators | [`parse_duration`](https://prestodb.io/docs/current/functions/datetime.html#parse_duration-string-interval) | No |
| Date and Time Functions and Operators | [`quarter`](https://prestodb.io/docs/current/functions/datetime.html#quarter-x-bigint) | No |
| Date and Time Functions and Operators | [`second`](https://prestodb.io/docs/current/functions/datetime.html#second-x-bigint) | No |
| Date and Time Functions and Operators | [`timezone_hour`](https://prestodb.io/docs/current/functions/datetime.html#timezone_hour-timestamp-bigint) | No |
| Date and Time Functions and Operators | [`timezone_minute`](https://prestodb.io/docs/current/functions/datetime.html#timezone_minute-timestamp-bigint) | No |
| Date and Time Functions and Operators | [`to_iso8601`](https://prestodb.io/docs/current/functions/datetime.html#to_iso8601-x-varchar) | No |
| Date and Time Functions and Operators | [`to_milliseconds`](https://prestodb.io/docs/current/functions/datetime.html#to_milliseconds-interval-bigint) | No |
| Date and Time Functions and Operators | [`to_unixtime`](https://prestodb.io/docs/current/functions/datetime.html#to_unixtime-timestamp-double) | No |
| Date and Time Functions and Operators | [`week`](https://prestodb.io/docs/current/functions/datetime.html#week-x-bigint) | No |
| Date and Time Functions and Operators | [`week_of_year`](https://prestodb.io/docs/current/functions/datetime.html#week_of_year-x-bigint) | No |
| Date and Time Functions and Operators | [`year`](https://prestodb.io/docs/current/functions/datetime.html#year-x-bigint) | No |
| Date and Time Functions and Operators | [`year_of_week`](https://prestodb.io/docs/current/functions/datetime.html#year_of_week-x-bigint) | No |
| Date and Time Functions and Operators | [`yow`](https://prestodb.io/docs/current/functions/datetime.html#yow-x-bigint) | No |
| Aggregate Functions | [`any_value`](https://prestodb.io/docs/current/functions/aggregate.html#any_value-x-same-as-input) | No |
| Aggregate Functions | [`approx_distinct`](https://prestodb.io/docs/current/functions/aggregate.html#approx_distinct-x-bigint) | No |
| Aggregate Functions | [`approx_most_frequent`](https://prestodb.io/docs/current/functions/aggregate.html#approx_most_frequent-buckets-value-capacity-map-same-as-value-bigint) | No |
| Aggregate Functions | [`approx_percentile`](https://prestodb.io/docs/current/functions/aggregate.html#approx_percentile-x-percentage-same-as-x) | No |
| Aggregate Functions | [`approx_set`](https://prestodb.io/docs/current/functions/aggregate.html) | No |
| Aggregate Functions | [`arbitrary`](https://prestodb.io/docs/current/functions/aggregate.html#arbitrary-x-same-as-input) | No |
| Aggregate Functions | [`array_agg`](https://prestodb.io/docs/current/functions/aggregate.html#array_agg-x-array-same-as-input) | No |
| Aggregate Functions | [`avg`](https://prestodb.io/docs/current/functions/aggregate.html#avg-x-double) | No |
| Aggregate Functions | [`bitwise_and_agg`](https://prestodb.io/docs/current/functions/aggregate.html#bitwise_and_agg-x-bigint) | No |
| Aggregate Functions | [`bitwise_or_agg`](https://prestodb.io/docs/current/functions/aggregate.html#bitwise_or_agg-x-bigint) | No |
| Aggregate Functions | [`bitwise_xor_agg`](https://prestodb.io/docs/current/functions/aggregate.html#bitwise_xor_agg-x-bigint) | No |
| Aggregate Functions | [`bool_and`](https://prestodb.io/docs/current/functions/aggregate.html#bool_and-boolean-boolean) | No |
| Aggregate Functions | [`bool_or`](https://prestodb.io/docs/current/functions/aggregate.html#bool_or-boolean-boolean) | No |
| Aggregate Functions | [`checksum`](https://prestodb.io/docs/current/functions/aggregate.html#checksum-x-varbinary) | No |
| Aggregate Functions | [`classification_fall_out`](https://prestodb.io/docs/current/functions/aggregate.html#classification_fall_out-buckets-y-x-weight-array-double) | No |
| Aggregate Functions | [`classification_miss_rate`](https://prestodb.io/docs/current/functions/aggregate.html#classification_miss_rate-buckets-y-x-weight-array-double) | No |
| Aggregate Functions | [`classification_precision`](https://prestodb.io/docs/current/functions/aggregate.html#classification_precision-buckets-y-x-weight-array-double) | No |
| Aggregate Functions | [`classification_recall`](https://prestodb.io/docs/current/functions/aggregate.html#classification_recall-buckets-y-x-weight-array-double) | No |
| Aggregate Functions | [`classification_thresholds`](https://prestodb.io/docs/current/functions/aggregate.html#classification_thresholds-buckets-y-x-array-double) | No |
| Aggregate Functions | [`corr`](https://prestodb.io/docs/current/functions/aggregate.html#corr-y-x-double) | No |
| Aggregate Functions | [`count`](https://prestodb.io/docs/current/functions/aggregate.html#count-bigint) | No |
| Aggregate Functions | [`count_if`](https://prestodb.io/docs/current/functions/aggregate.html#count_if-x-bigint) | No |
| Aggregate Functions | [`covar_pop`](https://prestodb.io/docs/current/functions/aggregate.html#covar_pop-y-x-double) | No |
| Aggregate Functions | [`covar_samp`](https://prestodb.io/docs/current/functions/aggregate.html#covar_samp-y-x-double) | No |
| Aggregate Functions | [`differential_entropy`](https://prestodb.io/docs/current/functions/aggregate.html#differential_entropy) | No |
| Aggregate Functions | [`entropy`](https://prestodb.io/docs/current/functions/aggregate.html#entropy-c-double) | No |
| Aggregate Functions | [`every`](https://prestodb.io/docs/current/functions/aggregate.html#every-boolean-boolean) | No |
| Aggregate Functions | [`geometric_mean`](https://prestodb.io/docs/current/functions/aggregate.html#geometric_mean-bigint-double) | No |
| Aggregate Functions | [`histogram`](https://prestodb.io/docs/current/functions/aggregate.html#histogram) | No |
| Aggregate Functions | [`khyperloglog_agg`](https://prestodb.io/docs/current/functions/aggregate.html) | No |
| Aggregate Functions | [`kurtosis`](https://prestodb.io/docs/current/functions/aggregate.html#kurtosis-x-double) | No |
| Aggregate Functions | [`map_agg`](https://prestodb.io/docs/current/functions/aggregate.html#map_agg) | No |
| Aggregate Functions | [`map_union`](https://prestodb.io/docs/current/functions/aggregate.html#map_union) | No |
| Aggregate Functions | [`map_union_sum`](https://prestodb.io/docs/current/functions/aggregate.html#map_union_sum) | No |
| Aggregate Functions | [`max`](https://prestodb.io/docs/current/functions/aggregate.html#max-x-same-as-input) | No |
| Aggregate Functions | [`max_by`](https://prestodb.io/docs/current/functions/aggregate.html#max_by-x-y-same-as-x) | No |
| Aggregate Functions | [`merge`](https://prestodb.io/docs/current/functions/aggregate.html) | No |
| Aggregate Functions | [`min`](https://prestodb.io/docs/current/functions/aggregate.html#min-x-same-as-input) | No |
| Aggregate Functions | [`min_by`](https://prestodb.io/docs/current/functions/aggregate.html#min_by-x-y-same-as-x) | No |
| Aggregate Functions | [`multimap_agg`](https://prestodb.io/docs/current/functions/aggregate.html#multimap_agg) | No |
| Aggregate Functions | [`numeric_histogram`](https://prestodb.io/docs/current/functions/aggregate.html#numeric_histogram-buckets-value-weight-map-double-double) | No |
| Aggregate Functions | [`qdigest_agg`](https://prestodb.io/docs/current/functions/aggregate.html) | No |
| Aggregate Functions | [`reduce_agg`](https://prestodb.io/docs/current/functions/aggregate.html#reduce_agg-inputValue-T-initialState-S-inputFunction-S-T-S-combineFunction-S-S-S-S) | No |
| Aggregate Functions | [`regr_avgx`](https://prestodb.io/docs/current/functions/aggregate.html#regr_avgx-y-x-double) | No |
| Aggregate Functions | [`regr_avgy`](https://prestodb.io/docs/current/functions/aggregate.html#regr_avgy-y-x-double) | No |
| Aggregate Functions | [`regr_count`](https://prestodb.io/docs/current/functions/aggregate.html#regr_count-y-x-double) | No |
| Aggregate Functions | [`regr_intercept`](https://prestodb.io/docs/current/functions/aggregate.html#regr_intercept-y-x-double) | No |
| Aggregate Functions | [`regr_r2`](https://prestodb.io/docs/current/functions/aggregate.html#regr_r2-y-x-double) | No |
| Aggregate Functions | [`regr_slope`](https://prestodb.io/docs/current/functions/aggregate.html#regr_slope-y-x-double) | No |
| Aggregate Functions | [`regr_sxx`](https://prestodb.io/docs/current/functions/aggregate.html#regr_sxx-y-x-double) | No |
| Aggregate Functions | [`regr_sxy`](https://prestodb.io/docs/current/functions/aggregate.html#regr_sxy-y-x-double) | No |
| Aggregate Functions | [`regr_syy`](https://prestodb.io/docs/current/functions/aggregate.html#regr_syy-y-x-double) | No |
| Aggregate Functions | [`reservoir_sample`](https://prestodb.io/docs/current/functions/aggregate.html#reservoir_sample) | No |
| Aggregate Functions | [`set_agg`](https://prestodb.io/docs/current/functions/aggregate.html#set_agg-x-array-same-as-input) | No |
| Aggregate Functions | [`set_union`](https://prestodb.io/docs/current/functions/aggregate.html#set_union) | No |
| Aggregate Functions | [`skewness`](https://prestodb.io/docs/current/functions/aggregate.html#skewness-x-double) | No |
| Aggregate Functions | [`stddev`](https://prestodb.io/docs/current/functions/aggregate.html#stddev-x-double) | No |
| Aggregate Functions | [`stddev_pop`](https://prestodb.io/docs/current/functions/aggregate.html#stddev_pop-x-double) | No |
| Aggregate Functions | [`stddev_samp`](https://prestodb.io/docs/current/functions/aggregate.html#stddev_samp-x-double) | No |
| Aggregate Functions | [`sum`](https://prestodb.io/docs/current/functions/aggregate.html#sum-x-same-as-input) | No |
| Aggregate Functions | [`var_pop`](https://prestodb.io/docs/current/functions/aggregate.html#var_pop-x-double) | No |
| Aggregate Functions | [`var_samp`](https://prestodb.io/docs/current/functions/aggregate.html#var_samp-x-double) | No |
| Aggregate Functions | [`variance`](https://prestodb.io/docs/current/functions/aggregate.html#variance-x-double) | No |
| Noisy Aggregate Functions | [`cardinality`](https://prestodb.io/docs/current/functions/noisy.html#cardinality-SfmSketch-bigint) | No |
| Noisy Aggregate Functions | [`merge_sfm`](https://prestodb.io/docs/current/functions/noisy.html#merge_sfm-ARRAY-SfmSketch-...-SfmSketch) | No |
| Noisy Aggregate Functions | [`noisy_approx_distinct_sfm`](https://prestodb.io/docs/current/functions/noisy.html#noisy_approx_distinct_sfm-col-epsilon-buckets-precision-bigint) | No |
| Noisy Aggregate Functions | [`noisy_approx_set_sfm`](https://prestodb.io/docs/current/functions/noisy.html#noisy_approx_set_sfm-col-epsilon-buckets-precision-SfmSketch) | No |
| Noisy Aggregate Functions | [`noisy_approx_set_sfm_from_index_and_zeros`](https://prestodb.io/docs/current/functions/noisy.html#noisy_approx_set_sfm_from_index_and_zeros-col_index-col_zeros-epsilon-buckets-precision-SfmSketch) | No |
| Noisy Aggregate Functions | [`noisy_avg_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_avg_gaussian-col-noise_scale-lower-upper-random_seed-double) | No |
| Noisy Aggregate Functions | [`noisy_count_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_count_gaussian-col-noise_scale-random_seed-bigint) | No |
| Noisy Aggregate Functions | [`noisy_count_if_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_count_if_gaussian-col-noise_scale-random_seed-bigint) | No |
| Noisy Aggregate Functions | [`noisy_empty_approx_set_sfm`](https://prestodb.io/docs/current/functions/noisy.html#noisy_empty_approx_set_sfm-epsilon-buckets-precision-SfmSketch) | No |
| Noisy Aggregate Functions | [`noisy_sum_gaussian`](https://prestodb.io/docs/current/functions/noisy.html#noisy_sum_gaussian-col-noise_scale-lower-upper-random_seed-double) | No |
| Window Functions | [`cume_dist`](https://prestodb.io/docs/current/functions/window.html#cume_dist-double) | Stubbed |
| Window Functions | [`dense_rank`](https://prestodb.io/docs/current/functions/window.html#dense_rank-bigint) | Stubbed |
| Window Functions | [`first_value`](https://prestodb.io/docs/current/functions/window.html#first_value-x-same-as-input) | Stubbed |
| Window Functions | [`lag`](https://prestodb.io/docs/current/functions/window.html#lag-x-offset-default_value-same-as-input) | Stubbed |
| Window Functions | [`last_value`](https://prestodb.io/docs/current/functions/window.html#last_value-x-same-as-input) | Stubbed |
| Window Functions | [`lead`](https://prestodb.io/docs/current/functions/window.html#lead-x-offset-default_value-same-as-input) | Stubbed |
| Window Functions | [`nth_value`](https://prestodb.io/docs/current/functions/window.html#nth_value-x-offset-same-as-input) | Stubbed |
| Window Functions | [`ntile`](https://prestodb.io/docs/current/functions/window.html#ntile-n-bigint) | Stubbed |
| Window Functions | [`percent_rank`](https://prestodb.io/docs/current/functions/window.html#percent_rank-double) | Stubbed |
| Window Functions | [`rank`](https://prestodb.io/docs/current/functions/window.html#rank-bigint) | Stubbed |
| Window Functions | [`row_number`](https://prestodb.io/docs/current/functions/window.html#row_number-bigint) | Stubbed |
| Array Functions and Operators | [`all_match`](https://prestodb.io/docs/current/functions/array.html#all_match-array-T-function-T-boolean-boolean) | No |
| Array Functions and Operators | [`any_match`](https://prestodb.io/docs/current/functions/array.html#any_match-array-T-function-T-boolean-boolean) | No |
| Array Functions and Operators | [`array_average`](https://prestodb.io/docs/current/functions/array.html#array_average-array-double-double) | No |
| Array Functions and Operators | [`array_cum_sum`](https://prestodb.io/docs/current/functions/array.html#array_cum_sum) | No |
| Array Functions and Operators | [`array_distinct`](https://prestodb.io/docs/current/functions/array.html#array_distinct-x-array) | No |
| Array Functions and Operators | [`array_duplicates`](https://prestodb.io/docs/current/functions/array.html#array_duplicates) | No |
| Array Functions and Operators | [`array_except`](https://prestodb.io/docs/current/functions/array.html#array_except-x-y-array) | No |
| Array Functions and Operators | [`array_frequency`](https://prestodb.io/docs/current/functions/array.html#array_frequency) | No |
| Array Functions and Operators | [`array_has_duplicates`](https://prestodb.io/docs/current/functions/array.html#array_has_duplicates-array-T-boolean) | No |
| Array Functions and Operators | [`array_intersect`](https://prestodb.io/docs/current/functions/array.html#array_intersect-x-y-array) | No |
| Array Functions and Operators | [`array_join`](https://prestodb.io/docs/current/functions/array.html#array_join-x-delimiter-null_replacement-varchar) | No |
| Array Functions and Operators | [`array_least_frequent`](https://prestodb.io/docs/current/functions/array.html#array_least_frequent) | No |
| Array Functions and Operators | [`array_max`](https://prestodb.io/docs/current/functions/array.html#array_max-x-x) | No |
| Array Functions and Operators | [`array_max_by`](https://prestodb.io/docs/current/functions/array.html#array_max_by-array-T-function-T-U-T) | No |
| Array Functions and Operators | [`array_min`](https://prestodb.io/docs/current/functions/array.html#array_min-x-x) | No |
| Array Functions and Operators | [`array_min_by`](https://prestodb.io/docs/current/functions/array.html#array_min_by-array-T-function-T-U-T) | No |
| Array Functions and Operators | [`array_normalize`](https://prestodb.io/docs/current/functions/array.html#array_normalize-x-p-array) | No |
| Array Functions and Operators | [`array_position`](https://prestodb.io/docs/current/functions/array.html#array_position-x-element-bigint) | No |
| Array Functions and Operators | [`array_remove`](https://prestodb.io/docs/current/functions/array.html#array_remove-x-element-array) | No |
| Array Functions and Operators | [`array_sort`](https://prestodb.io/docs/current/functions/array.html#array_sort-x-array) | No |
| Array Functions and Operators | [`array_sort_desc`](https://prestodb.io/docs/current/functions/array.html#array_sort_desc-x-array) | No |
| Array Functions and Operators | [`array_split_into_chunks`](https://prestodb.io/docs/current/functions/array.html#array_split_into_chunks) | No |
| Array Functions and Operators | [`array_sum`](https://prestodb.io/docs/current/functions/array.html#array_sum-array-T-bigint-double) | No |
| Array Functions and Operators | [`array_top_n`](https://prestodb.io/docs/current/functions/array.html#array_top_n) | No |
| Array Functions and Operators | [`array_transpose`](https://prestodb.io/docs/current/functions/array.html#array_transpose) | No |
| Array Functions and Operators | [`array_union`](https://prestodb.io/docs/current/functions/array.html#array_union-x-y-array) | No |
| Array Functions and Operators | [`arrays_overlap`](https://prestodb.io/docs/current/functions/array.html#arrays_overlap-x-y-boolean) | No |
| Array Functions and Operators | [`combinations`](https://prestodb.io/docs/current/functions/array.html#combinations) | No |
| Array Functions and Operators | [`contains`](https://prestodb.io/docs/current/functions/array.html#contains-x-element-boolean) | No |
| Array Functions and Operators | [`element_at`](https://prestodb.io/docs/current/functions/array.html#element_at-array-E-index-E) | No |
| Array Functions and Operators | [`filter`](https://prestodb.io/docs/current/functions/array.html#filter) | No |
| Array Functions and Operators | [`find_first`](https://prestodb.io/docs/current/functions/array.html#find_first-array-E-function-T-boolean-E) | No |
| Array Functions and Operators | [`find_first_index`](https://prestodb.io/docs/current/functions/array.html#find_first_index-array-E-function-T-boolean-BIGINT) | No |
| Array Functions and Operators | [`flatten`](https://prestodb.io/docs/current/functions/array.html#flatten-x-array) | No |
| Array Functions and Operators | [`ngrams`](https://prestodb.io/docs/current/functions/array.html#ngrams) | No |
| Array Functions and Operators | [`none_match`](https://prestodb.io/docs/current/functions/array.html#none_match-array-T-function-T-boolean-boolean) | No |
| Array Functions and Operators | [`reduce`](https://prestodb.io/docs/current/functions/array.html#reduce-array-T-initialState-S-inputFunction-S-T-S-outputFunction-S-R-R) | No |
| Array Functions and Operators | [`remove_nulls`](https://prestodb.io/docs/current/functions/array.html#remove_nulls-array-T-array) | No |
| Array Functions and Operators | [`repeat`](https://prestodb.io/docs/current/functions/array.html#repeat-element-count-array) | No |
| Array Functions and Operators | [`sequence`](https://prestodb.io/docs/current/functions/array.html#sequence) | No |
| Array Functions and Operators | [`shuffle`](https://prestodb.io/docs/current/functions/array.html#shuffle-x-array) | No |
| Array Functions and Operators | [`slice`](https://prestodb.io/docs/current/functions/array.html#slice-x-start-length-array) | No |
| Array Functions and Operators | [`transform`](https://prestodb.io/docs/current/functions/array.html#transform) | No |
| Array Functions and Operators | [`trim_array`](https://prestodb.io/docs/current/functions/array.html#trim_array-x-n-array) | No |
| Array Functions and Operators | [`zip`](https://prestodb.io/docs/current/functions/array.html#zip) | No |
| Array Functions and Operators | [`zip_with`](https://prestodb.io/docs/current/functions/array.html#zip_with) | No |
| Map Functions and Operators | [`all_keys_match`](https://prestodb.io/docs/current/functions/map.html#all_keys_match-x-K-V-function-K-boolean-boolean) | No |
| Map Functions and Operators | [`any_keys_match`](https://prestodb.io/docs/current/functions/map.html#any_keys_match-x-K-V-function-K-boolean-boolean) | No |
| Map Functions and Operators | [`any_values_match`](https://prestodb.io/docs/current/functions/map.html#any_values_match-x-K-V-function-V-boolean-boolean) | No |
| Map Functions and Operators | [`array_to_map_int_keys`](https://prestodb.io/docs/current/functions/map.html#array_to_map_int_keys) | No |
| Map Functions and Operators | [`map`](https://prestodb.io/docs/current/functions/map.html#map-map-unknown-unknown) | No |
| Map Functions and Operators | [`map_concat`](https://prestodb.io/docs/current/functions/map.html#map_concat) | No |
| Map Functions and Operators | [`map_entries`](https://prestodb.io/docs/current/functions/map.html#map_entries) | No |
| Map Functions and Operators | [`map_filter`](https://prestodb.io/docs/current/functions/map.html#map_filter) | No |
| Map Functions and Operators | [`map_from_entries`](https://prestodb.io/docs/current/functions/map.html#map_from_entries) | No |
| Map Functions and Operators | [`map_int_keys_to_array`](https://prestodb.io/docs/current/functions/map.html#map_int_keys_to_array) | No |
| Map Functions and Operators | [`map_key_exists`](https://prestodb.io/docs/current/functions/map.html#map_key_exists-x-K-V-k-boolean) | No |
| Map Functions and Operators | [`map_keys`](https://prestodb.io/docs/current/functions/map.html#map_keys) | No |
| Map Functions and Operators | [`map_keys_by_top_n_values`](https://prestodb.io/docs/current/functions/map.html#map_keys_by_top_n_values) | No |
| Map Functions and Operators | [`map_normalize`](https://prestodb.io/docs/current/functions/map.html#map_normalize) | No |
| Map Functions and Operators | [`map_remove_null_values`](https://prestodb.io/docs/current/functions/map.html#map_remove_null_values) | No |
| Map Functions and Operators | [`map_subset`](https://prestodb.io/docs/current/functions/map.html#map_subset) | No |
| Map Functions and Operators | [`map_top_n`](https://prestodb.io/docs/current/functions/map.html#map_top_n) | No |
| Map Functions and Operators | [`map_top_n_keys`](https://prestodb.io/docs/current/functions/map.html#map_top_n_keys) | No |
| Map Functions and Operators | [`map_top_n_values`](https://prestodb.io/docs/current/functions/map.html#map_top_n_values) | No |
| Map Functions and Operators | [`map_values`](https://prestodb.io/docs/current/functions/map.html#map_values) | No |
| Map Functions and Operators | [`map_zip_with`](https://prestodb.io/docs/current/functions/map.html#map_zip_with) | No |
| Map Functions and Operators | [`multimap_from_entries`](https://prestodb.io/docs/current/functions/map.html#multimap_from_entries) | No |
| Map Functions and Operators | [`no_keys_match`](https://prestodb.io/docs/current/functions/map.html#no_keys_match-x-K-V-function-K-boolean-boolean) | No |
| Map Functions and Operators | [`no_values_match`](https://prestodb.io/docs/current/functions/map.html#no_values_match-x-K-V-function-V-boolean-boolean) | No |
| Map Functions and Operators | [`transform_keys`](https://prestodb.io/docs/current/functions/map.html#transform_keys) | No |
| Map Functions and Operators | [`transform_values`](https://prestodb.io/docs/current/functions/map.html#transform_values) | No |
| URL Functions | [`url_decode`](https://prestodb.io/docs/current/functions/url.html#url_decode-value-varchar) | Supported |
| URL Functions | [`url_encode`](https://prestodb.io/docs/current/functions/url.html#url_encode-value-varchar) | Supported |
| URL Functions | [`url_extract_fragment`](https://prestodb.io/docs/current/functions/url.html#url_extract_fragment-url-varchar) | Supported |
| URL Functions | [`url_extract_host`](https://prestodb.io/docs/current/functions/url.html#url_extract_host-url-varchar) | Supported |
| URL Functions | [`url_extract_parameter`](https://prestodb.io/docs/current/functions/url.html#url_extract_parameter-url-name-varchar) | Supported |
| URL Functions | [`url_extract_path`](https://prestodb.io/docs/current/functions/url.html#url_extract_path-url-varchar) | Supported |
| URL Functions | [`url_extract_port`](https://prestodb.io/docs/current/functions/url.html#url_extract_port-url-bigint) | Supported |
| URL Functions | [`url_extract_protocol`](https://prestodb.io/docs/current/functions/url.html#url_extract_protocol-url-varchar) | Supported |
| URL Functions | [`url_extract_query`](https://prestodb.io/docs/current/functions/url.html#url_extract_query-url-varchar) | Supported |
| IP Functions | [`ip_prefix`](https://prestodb.io/docs/current/functions/ip.html#ip_prefix-ip_address-prefix_bits-ipprefix) | Supported |
| IP Functions | [`ip_prefix_collapse`](https://prestodb.io/docs/current/functions/ip.html#ip_prefix_collapse) | Supported |
| IP Functions | [`ip_prefix_subnets`](https://prestodb.io/docs/current/functions/ip.html#ip_prefix_subnets) | Supported |
| IP Functions | [`ip_subnet_max`](https://prestodb.io/docs/current/functions/ip.html#ip_subnet_max-ip_prefix-ip_address) | Supported |
| IP Functions | [`ip_subnet_min`](https://prestodb.io/docs/current/functions/ip.html#ip_subnet_min-ip_prefix-ip_address) | Supported |
| IP Functions | [`ip_subnet_range`](https://prestodb.io/docs/current/functions/ip.html#ip_subnet_range) | Supported |
| IP Functions | [`is_private_ip`](https://prestodb.io/docs/current/functions/ip.html#is_private_ip-ip_address-boolean) | Supported |
| IP Functions | [`is_subnet_of`](https://prestodb.io/docs/current/functions/ip.html#is_subnet_of-ip_prefix-ip_address-boolean) | Supported |
| Geospatial Functions | [`bing_tile`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile-x-y-zoom_level-BingTile) | No |
| Geospatial Functions | [`bing_tile_at`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_at-latitude-longitude-zoom_level-BingTile) | No |
| Geospatial Functions | [`bing_tile_children`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_children) | No |
| Geospatial Functions | [`bing_tile_coordinates`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_coordinates-tile-row-x-y) | No |
| Geospatial Functions | [`bing_tile_parent`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_parent-tile-BingTile) | No |
| Geospatial Functions | [`bing_tile_polygon`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_polygon-tile-Geometry) | No |
| Geospatial Functions | [`bing_tile_quadkey`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_quadkey-tile-varchar) | No |
| Geospatial Functions | [`bing_tile_zoom_level`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tile_zoom_level-tile-tinyint) | No |
| Geospatial Functions | [`bing_tiles_around`](https://prestodb.io/docs/current/functions/geospatial.html#bing_tiles_around) | No |
| Geospatial Functions | [`convex_hull_agg`](https://prestodb.io/docs/current/functions/geospatial.html#convex_hull_agg-Geometry-Geometry) | No |
| Geospatial Functions | [`expand_envelope`](https://prestodb.io/docs/current/functions/geospatial.html#expand_envelope-Geometry-double-Geometry) | No |
| Geospatial Functions | [`flatten_geometry_collections`](https://prestodb.io/docs/current/functions/geospatial.html#flatten_geometry_collections) | No |
| Geospatial Functions | [`geometry_as_geojson`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_as_geojson-Geometry-varchar) | No |
| Geospatial Functions | [`geometry_from_geojson`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_from_geojson-varchar-Geometry) | No |
| Geospatial Functions | [`geometry_invalid_reason`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_invalid_reason-Geometry-varchar) | No |
| Geospatial Functions | [`geometry_nearest_points`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_nearest_points) | No |
| Geospatial Functions | [`geometry_to_bing_tiles`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_to_bing_tiles) | No |
| Geospatial Functions | [`geometry_to_dissolved_bing_tiles`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_to_dissolved_bing_tiles) | No |
| Geospatial Functions | [`geometry_union`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_union-array-Geometry-Geometry) | No |
| Geospatial Functions | [`geometry_union_agg`](https://prestodb.io/docs/current/functions/geospatial.html#geometry_union_agg-Geometry-Geometry) | No |
| Geospatial Functions | [`great_circle_distance`](https://prestodb.io/docs/current/functions/geospatial.html#great_circle_distance-latitude1-longitude1-latitude2-longitude2-double) | No |
| Geospatial Functions | [`line_interpolate_point`](https://prestodb.io/docs/current/functions/geospatial.html#line_interpolate_point-LineString-double-Geometry) | No |
| Geospatial Functions | [`line_locate_point`](https://prestodb.io/docs/current/functions/geospatial.html#line_locate_point-LineString-Point-double) | No |
| Geospatial Functions | [`simplify_geometry`](https://prestodb.io/docs/current/functions/geospatial.html#simplify_geometry-Geometry-double-Geometry) | No |
| Geospatial Functions | [`st_area`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Area-Geometry-double) | No |
| Geospatial Functions | [`st_asbinary`](https://prestodb.io/docs/current/functions/geospatial.html#ST_AsBinary-Geometry-varbinary) | No |
| Geospatial Functions | [`st_astext`](https://prestodb.io/docs/current/functions/geospatial.html#ST_AsText-Geometry-varchar) | No |
| Geospatial Functions | [`st_boundary`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Boundary-Geometry-Geometry) | No |
| Geospatial Functions | [`st_buffer`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Buffer-Geometry-distance-Geometry) | No |
| Geospatial Functions | [`st_centroid`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Centroid-Geometry-Point) | No |
| Geospatial Functions | [`st_contains`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Contains-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_convexhull`](https://prestodb.io/docs/current/functions/geospatial.html#ST_ConvexHull-Geometry-Geometry) | No |
| Geospatial Functions | [`st_coorddim`](https://prestodb.io/docs/current/functions/geospatial.html#ST_CoordDim-Geometry-tinyint) | No |
| Geospatial Functions | [`st_crosses`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Crosses-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_difference`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Difference-Geometry-Geometry-Geometry) | No |
| Geospatial Functions | [`st_dimension`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Dimension-Geometry-bigint) | No |
| Geospatial Functions | [`st_disjoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Disjoint-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_distance`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Distance-Geometry-Geometry-double) | No |
| Geospatial Functions | [`st_endpoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_EndPoint-Geometry-point) | No |
| Geospatial Functions | [`st_envelope`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Envelope-Geometry-Geometry) | No |
| Geospatial Functions | [`st_envelopeaspts`](https://prestodb.io/docs/current/functions/geospatial.html#ST_EnvelopeAsPts) | No |
| Geospatial Functions | [`st_equals`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Equals-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_exteriorring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_ExteriorRing-Geometry-Geometry) | No |
| Geospatial Functions | [`st_geometries`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Geometries) | No |
| Geospatial Functions | [`st_geometryfromtext`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeometryFromText-varchar-Geometry) | No |
| Geospatial Functions | [`st_geometryn`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeometryN-Geometry-index-Geometry) | No |
| Geospatial Functions | [`st_geometrytype`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeometryType-Geometry-varchar) | No |
| Geospatial Functions | [`st_geomfrombinary`](https://prestodb.io/docs/current/functions/geospatial.html#ST_GeomFromBinary-varbinary-Geometry) | No |
| Geospatial Functions | [`st_interiorringn`](https://prestodb.io/docs/current/functions/geospatial.html#ST_InteriorRingN-Geometry-index-Geometry) | No |
| Geospatial Functions | [`st_interiorrings`](https://prestodb.io/docs/current/functions/geospatial.html#ST_InteriorRings) | No |
| Geospatial Functions | [`st_intersection`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Intersection-Geometry-Geometry-Geometry) | No |
| Geospatial Functions | [`st_intersects`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Intersects-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_isclosed`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsClosed-Geometry-boolean) | No |
| Geospatial Functions | [`st_isempty`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsEmpty-Geometry-boolean) | No |
| Geospatial Functions | [`st_isring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsRing-Geometry-boolean) | No |
| Geospatial Functions | [`st_issimple`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsSimple-Geometry-boolean) | No |
| Geospatial Functions | [`st_isvalid`](https://prestodb.io/docs/current/functions/geospatial.html#ST_IsValid-Geometry-boolean) | No |
| Geospatial Functions | [`st_length`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Length-Geometry-double) | No |
| Geospatial Functions | [`st_linefromtext`](https://prestodb.io/docs/current/functions/geospatial.html#ST_LineFromText-varchar-LineString) | No |
| Geospatial Functions | [`st_linestring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_LineString-array-Point-LineString) | No |
| Geospatial Functions | [`st_multipoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_MultiPoint-array-Point-MultiPoint) | No |
| Geospatial Functions | [`st_numgeometries`](https://prestodb.io/docs/current/functions/geospatial.html#ST_NumGeometries-Geometry-bigint) | No |
| Geospatial Functions | [`st_numinteriorring`](https://prestodb.io/docs/current/functions/geospatial.html#ST_NumInteriorRing-Geometry-bigint) | No |
| Geospatial Functions | [`st_numpoints`](https://prestodb.io/docs/current/functions/geospatial.html#ST_NumPoints-Geometry-bigint) | No |
| Geospatial Functions | [`st_overlaps`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Overlaps-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_point`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Point-x-y-Point) | No |
| Geospatial Functions | [`st_pointn`](https://prestodb.io/docs/current/functions/geospatial.html#ST_PointN-LineString-index-Point) | No |
| Geospatial Functions | [`st_points`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Points) | No |
| Geospatial Functions | [`st_polygon`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Polygon-varchar-Polygon) | No |
| Geospatial Functions | [`st_relate`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Relate-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_startpoint`](https://prestodb.io/docs/current/functions/geospatial.html#ST_StartPoint-Geometry-point) | No |
| Geospatial Functions | [`st_symdifference`](https://prestodb.io/docs/current/functions/geospatial.html#ST_SymDifference-Geometry-Geometry-Geometry) | No |
| Geospatial Functions | [`st_touches`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Touches-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_union`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Union-Geometry-Geometry-Geometry) | No |
| Geospatial Functions | [`st_within`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Within-Geometry-Geometry-boolean) | No |
| Geospatial Functions | [`st_x`](https://prestodb.io/docs/current/functions/geospatial.html#ST_X-Point-double) | No |
| Geospatial Functions | [`st_xmax`](https://prestodb.io/docs/current/functions/geospatial.html#ST_XMax-Geometry-double) | No |
| Geospatial Functions | [`st_xmin`](https://prestodb.io/docs/current/functions/geospatial.html#ST_XMin-Geometry-double) | No |
| Geospatial Functions | [`st_y`](https://prestodb.io/docs/current/functions/geospatial.html#ST_Y-Point-double) | No |
| Geospatial Functions | [`st_ymax`](https://prestodb.io/docs/current/functions/geospatial.html#ST_YMax-Geometry-double) | No |
| Geospatial Functions | [`st_ymin`](https://prestodb.io/docs/current/functions/geospatial.html#ST_YMin-Geometry-double) | No |
| Geospatial Functions | [`to_geometry`](https://prestodb.io/docs/current/functions/geospatial.html#to_geometry-SphericalGeography-Geometry) | No |
| Geospatial Functions | [`to_spherical_geography`](https://prestodb.io/docs/current/functions/geospatial.html#to_spherical_geography-Geometry-SphericalGeography) | No |
| HyperLogLog Functions | [`empty_approx_set`](https://prestodb.io/docs/current/functions/hyperloglog.html#empty_approx_set-HyperLogLog) | No |
| HyperLogLog Functions | [`merge_hll`](https://prestodb.io/docs/current/functions/hyperloglog.html#merge_hll-array-HyperLogLog-HyperLogLog) | No |
| KHyperLogLog Functions | [`intersection_cardinality`](https://prestodb.io/docs/current/functions/khyperloglog.html#intersection_cardinality-khll1-khll2-bigint) | No |
| KHyperLogLog Functions | [`jaccard_index`](https://prestodb.io/docs/current/functions/khyperloglog.html#jaccard_index-khll1-khll2-double) | No |
| KHyperLogLog Functions | [`merge_khll`](https://prestodb.io/docs/current/functions/khyperloglog.html#merge_khll-array-khll-KHyperLogLog) | No |
| KHyperLogLog Functions | [`reidentification_potential`](https://prestodb.io/docs/current/functions/khyperloglog.html#reidentification_potential-khll-threshold-double) | No |
| KHyperLogLog Functions | [`uniqueness_distribution`](https://prestodb.io/docs/current/functions/khyperloglog.html#uniqueness_distribution-khll-map-bigint-double) | No |
| Quantile Digest Functions | [`quantile_at_value`](https://prestodb.io/docs/current/functions/qdigest.html#quantile_at_value-qdigest-T-T-quantile) | No |
| Quantile Digest Functions | [`scale_qdigest`](https://prestodb.io/docs/current/functions/qdigest.html#scale_qdigest) | No |
| Quantile Digest Functions | [`value_at_quantile`](https://prestodb.io/docs/current/functions/qdigest.html#value_at_quantile-qdigest-T-quantile-T) | No |
| Quantile Digest Functions | [`values_at_quantiles`](https://prestodb.io/docs/current/functions/qdigest.html#values_at_quantiles-qdigest-T-quantiles-T) | No |
| UUID functions | [`uuid`](https://prestodb.io/docs/current/functions/uuid.html#uuid-uuid) | Supported |
| T-Digest Functions | [`construct_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#construct_tdigest-centroid_means-array-double-centroid_weights-array-double-compression-double-min-double-max-double-sum-double-count-bigint-tdigest-double) | No |
| T-Digest Functions | [`destructure_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#destructure_tdigest-tdigest-double-row-centroid_means-array-double-centroid_weights-array-integer-compression-double-min-double-max-double-sum-double-count-bigint) | No |
| T-Digest Functions | [`merge_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#merge_tdigest-array-tdigest-double-tdigest-double) | No |
| T-Digest Functions | [`quantiles_at_values`](https://prestodb.io/docs/current/functions/tdigest.html#quantiles_at_values-tdigest-double-values-array-double) | No |
| T-Digest Functions | [`scale_tdigest`](https://prestodb.io/docs/current/functions/tdigest.html#scale_tdigest-tdigest-double-scale_factor-tdigest-double) | No |
| T-Digest Functions | [`tdigest_agg`](https://prestodb.io/docs/current/functions/tdigest.html#tdigest_agg-x-tdigest-double) | No |
| T-Digest Functions | [`trimmed_mean`](https://prestodb.io/docs/current/functions/tdigest.html#trimmed_mean-tdigest-double-lower_quantile-upper_quantile-double) | No |
| Color Functions | [`bar`](https://prestodb.io/docs/current/functions/color.html#bar-x-width-varchar) | No |
| Color Functions | [`color`](https://prestodb.io/docs/current/functions/color.html#color-string-color) | No |
| Color Functions | [`render`](https://prestodb.io/docs/current/functions/color.html#render-x-color-varchar) | No |
| Color Functions | [`rgb`](https://prestodb.io/docs/current/functions/color.html#rgb-red-green-blue-color) | No |
| Session Information | [`current_user`](https://prestodb.io/docs/current/functions/session.html#current_user-varchar) | No |
| Teradata Functions | [`char2hexint`](https://prestodb.io/docs/current/functions/teradata.html#char2hexint-string-varchar) | No |
| Teradata Functions | [`index`](https://prestodb.io/docs/current/functions/teradata.html#index-string-substring-bigint) | No |
| Teradata Functions | [`substring`](https://prestodb.io/docs/current/functions/teradata.html#substring-string-start-varchar) | No |
| Teradata Functions | [`to_char`](https://prestodb.io/docs/current/functions/teradata.html#to_char-timestamp-format-varchar) | No |
| Teradata Functions | [`to_date`](https://prestodb.io/docs/current/functions/teradata.html#to_date-string-format-date) | No |
| Teradata Functions | [`to_timestamp`](https://prestodb.io/docs/current/functions/teradata.html#to_timestamp-string-format-timestamp) | No |
| Internationalization Functions | [`myanmar_font_encoding`](https://prestodb.io/docs/current/functions/internationalization.html#myanmar_font_encoding-text-varchar) | No |
| Internationalization Functions | [`myanmar_normalize_unicode`](https://prestodb.io/docs/current/functions/internationalization.html#myanmar_normalize_unicode-text-varchar) | No |
| Set Digest functions | [`hash_counts`](https://prestodb.io/docs/current/functions/setdigest.html#hash_counts) | No |
| Set Digest functions | [`make_set_digest`](https://prestodb.io/docs/current/functions/setdigest.html#make_set_digest-x-setdigest) | No |
| Set Digest functions | [`merge_set_digest`](https://prestodb.io/docs/current/functions/setdigest.html#merge_set_digest-setdigest-setdigest) | No |
| Sketch Functions | [`sketch_kll`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll-T-x-T-kllsketch-T) | No |
| Sketch Functions | [`sketch_kll_quantile`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll_quantile-T-sketch-kllsketch-T-rank-double-inclusivity-boolean-T) | No |
| Sketch Functions | [`sketch_kll_rank`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll_rank-T-sketch-kllsketch-T-quantile-T-inclusivity-boolean-double) | No |
| Sketch Functions | [`sketch_kll_with_k`](https://prestodb.io/docs/current/functions/sketch.html#sketch_kll_with_k-T-x-T-k-int-kllsketch-T) | No |
| Sketch Functions | [`sketch_theta`](https://prestodb.io/docs/current/functions/sketch.html#sketch_theta-x-varbinary) | No |
| Sketch Functions | [`sketch_theta_estimate`](https://prestodb.io/docs/current/functions/sketch.html#sketch_theta_estimate-sketch-double) | No |
| Sketch Functions | [`sketch_theta_summary`](https://prestodb.io/docs/current/functions/sketch.html#sketch_theta_summary) | No |
| Pinot Functions | [`pinot_binary_decimal_to_double`](https://prestodb.io/docs/current/functions/pinot.html#pinot_binary_decimal_to_double-binary-bigIntegerRadix-scale-returnZeroOnNull-double) | No |

**Summary:** 510 Presto functions total, 162 supported, 14 stubbed, 334 not exposed.
