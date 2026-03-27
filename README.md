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

## Presto function support matrix

| Presto function | Supported in SQLite extension | Comments |
|---|---|---|
| `abs` | Yes | — |
| `acos` | Yes | — |
| `asin` | Yes | — |
| `atan` | Yes | — |
| `atan2` | Yes | — |
| `beta_cdf` | Yes | — |
| `binomial_cdf` | Yes | — |
| `bit_count` | Yes | — |
| `bit_length` | Yes | — |
| `bitwise_and` | Yes | — |
| `bitwise_arithmetic_shift_right` | Yes | — |
| `bitwise_left_shift` | Yes | — |
| `bitwise_logical_shift_right` | Yes | — |
| `bitwise_not` | Yes | — |
| `bitwise_or` | Yes | — |
| `bitwise_right_shift` | Yes | — |
| `bitwise_right_shift_arithmetic` | Yes | — |
| `bitwise_shift_left` | Yes | — |
| `bitwise_xor` | Yes | — |
| `cauchy_cdf` | Yes | — |
| `cbrt` | Yes | — |
| `ceil` | Yes | — |
| `ceiling` | Yes | — |
| `chi_squared_cdf` | Yes | — |
| `chr` | Yes | — |
| `codepoint` | Yes | — |
| `concat` | Yes | Single p_concat bridge handles both text and blob forms. |
| `cos` | Yes | — |
| `cosh` | Yes | — |
| `cosine_similarity` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `crc32` | Yes | — |
| `cume_dist` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `degrees` | Yes | — |
| `dense_rank` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `dot_product` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `e` | Yes | — |
| `ends_with` | Yes | — |
| `exp` | Yes | — |
| `f_cdf` | Yes | — |
| `factorial` | Yes | — |
| `first_value` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `floor` | Yes | — |
| `from_base` | Yes | — |
| `from_base32` | Yes | — |
| `from_base64` | Yes | — |
| `from_base64url` | Yes | — |
| `from_big_endian_32` | Yes | — |
| `from_big_endian_64` | Yes | — |
| `from_hex` | Yes | — |
| `from_ieee754_32` | Yes | — |
| `from_ieee754_64` | Yes | — |
| `from_utf8` | Yes | — |
| `gamma_cdf` | Yes | — |
| `hamming_distance` | Yes | — |
| `hmac_md5` | Yes | — |
| `hmac_sha1` | Yes | — |
| `hmac_sha256` | Yes | — |
| `hmac_sha512` | Yes | — |
| `infinity` | Yes | — |
| `inverse_beta_cdf` | Yes | — |
| `inverse_binomial_cdf` | Yes | — |
| `inverse_cauchy_cdf` | Yes | — |
| `inverse_chi_squared_cdf` | Yes | — |
| `inverse_f_cdf` | Yes | — |
| `inverse_gamma_cdf` | Yes | — |
| `inverse_laplace_cdf` | Yes | — |
| `inverse_normal_cdf` | Yes | — |
| `inverse_poisson_cdf` | Yes | — |
| `inverse_t_cdf` | Yes | — |
| `inverse_weibull_cdf` | Yes | — |
| `ip_prefix` | Yes | — |
| `ip_prefix_collapse` | Yes | Input/output arrays are represented as comma-joined text. |
| `ip_prefix_subnets` | Yes | Returns comma-joined text instead of array type. |
| `ip_subnet_max` | Yes | — |
| `ip_subnet_min` | Yes | — |
| `ip_subnet_range` | Yes | Returns comma-joined text instead of array type. |
| `is_finite` | Yes | — |
| `is_infinite` | Yes | — |
| `is_nan` | Yes | — |
| `is_private_ip` | Yes | — |
| `is_subnet_of` | Yes | — |
| `jarowinkler_similarity` | Yes | — |
| `key_sampling_percent` | Yes | — |
| `l2_squared` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `lag` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `laplace_cdf` | Yes | — |
| `last_value` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `lead` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `length` | Yes | Single p_length bridge handles both text and blob forms. |
| `levenshtein_distance` | Yes | — |
| `ln` | Yes | — |
| `log10` | Yes | — |
| `log2` | Yes | — |
| `longest_common_prefix` | Yes | — |
| `lower` | Yes | — |
| `lpad` | Yes | Single p_lpad bridge handles both text and blob forms. |
| `ltrim` | Yes | — |
| `md5` | Yes | — |
| `mod` | Yes | — |
| `murmur3_x64_128` | Yes | — |
| `nan` | Yes | — |
| `normal_cdf` | Yes | — |
| `normalize` | Yes | — |
| `nth_value` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `ntile` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `percent_rank` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `pi` | Yes | — |
| `poisson_cdf` | Yes | — |
| `pow` | Yes | — |
| `power` | Yes | — |
| `radians` | Yes | — |
| `rand` | Yes | Non-deterministic random output. |
| `random` | Yes | Non-deterministic random output. |
| `rank` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `regexp_extract` | Yes | Uses Rust regex engine (not full Java/Presto regex semantics). |
| `regexp_extract_all` | Yes | Returns comma-joined text instead of array in SQLite bridge. |
| `regexp_like` | Yes | Uses Rust regex engine (not full Java/Presto regex semantics). |
| `regexp_replace` | Yes | Lambda/function replacement overload is not implemented. |
| `regexp_split` | Yes | Returns comma-joined text instead of array in SQLite bridge. |
| `replace` | Yes | — |
| `replace_first` | Yes | — |
| `reverse` | Yes | Single p_reverse bridge handles both text and blob forms. |
| `round` | Yes | — |
| `row_number` | No | Registered name exists, but SQLite argument/result bridge is not implemented yet (stub returns error). |
| `rpad` | Yes | Single p_rpad bridge handles both text and blob forms. |
| `rtrim` | Yes | — |
| `secure_rand` | Yes | Non-deterministic random output (OS RNG). |
| `secure_random` | Yes | Non-deterministic random output (OS RNG). |
| `sha1` | Yes | — |
| `sha256` | Yes | — |
| `sha512` | Yes | — |
| `sign` | Yes | — |
| `sin` | Yes | — |
| `split` | Yes | Returns comma-joined text instead of array in SQLite bridge. |
| `split_part` | Yes | — |
| `split_to_multimap` | Yes | Returns encoded text (key=v1|v2;...) instead of map type. |
| `spooky_hash_v2_32` | Yes | — |
| `spooky_hash_v2_64` | Yes | — |
| `sqrt` | Yes | — |
| `starts_with` | Yes | — |
| `strpos` | Yes | — |
| `strrpos` | Yes | — |
| `substr` | Yes | Single p_substr bridge handles both text and blob forms. |
| `t_cdf` | Yes | — |
| `tan` | Yes | — |
| `tanh` | Yes | — |
| `to_base` | Yes | — |
| `to_base32` | Yes | — |
| `to_base64` | Yes | — |
| `to_base64url` | Yes | — |
| `to_big_endian_32` | Yes | — |
| `to_big_endian_64` | Yes | — |
| `to_hex` | Yes | — |
| `to_ieee754_32` | Yes | — |
| `to_ieee754_64` | Yes | — |
| `to_utf8` | Yes | — |
| `trail` | Yes | — |
| `trim` | Yes | — |
| `truncate` | Yes | — |
| `upper` | Yes | — |
| `url_decode` | Yes | — |
| `url_encode` | Yes | — |
| `url_extract_fragment` | Yes | — |
| `url_extract_host` | Yes | — |
| `url_extract_parameter` | Yes | — |
| `url_extract_path` | Yes | — |
| `url_extract_port` | Yes | — |
| `url_extract_protocol` | Yes | — |
| `url_extract_query` | Yes | — |
| `uuid` | Yes | Non-deterministic (generates a new UUID each call). |
| `weibull_cdf` | Yes | — |
| `width_bucket` | Yes | — |
| `wilson_interval_lower` | Yes | — |
| `wilson_interval_upper` | Yes | — |
| `word_stem` | Yes | — |
| `xxhash64` | Yes | — |

**Summary:** 162 supported, 14 currently stubbed/not bridged.
