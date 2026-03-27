use std::collections::HashSet;

use presto_core::{FunctionRegistry, PrestoResult};
use sqlite_loadable::prelude::*;
use sqlite_loadable::{
    api, define_scalar_function, define_scalar_function_with_aux, Error, FunctionFlags, Result,
};

fn register_all(registry: &mut FunctionRegistry) -> PrestoResult<()> {
    presto_numeric::register(registry)?;
    presto_text::register(registry)?;
    presto_binary::register(registry)?;
    presto_net::register(registry)?;
    presto_window::register(registry)?;
    Ok(())
}

fn presto_abs(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let arg = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument"))?;
    if api::value_is_null(arg) {
        api::result_null(context);
        return Ok(());
    }

    match api::value_type(arg) {
        api::ValueType::Integer => {
            let value = api::value_int64(arg);
            let out = presto_numeric::math::abs_i64(value)
                .map_err(|e| Error::new_message(e.to_string().as_str()))?;
            api::result_int64(context, out);
        }
        _ => {
            let value = api::value_double(arg);
            api::result_double(context, presto_numeric::math::abs_f64(value));
        }
    }

    Ok(())
}

fn presto_bitwise_and(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let left = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument 1"))?;
    let right = values
        .get(1)
        .ok_or_else(|| Error::new_message("missing argument 2"))?;

    if api::value_is_null(left) || api::value_is_null(right) {
        api::result_null(context);
        return Ok(());
    }

    let out = presto_numeric::bitwise::bitwise_and(api::value_int64(left), api::value_int64(right));
    api::result_int64(context, out);
    Ok(())
}

fn presto_to_hex(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let arg = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument"))?;
    if api::value_is_null(arg) {
        api::result_null(context);
        return Ok(());
    }

    let out = presto_binary::encoding::to_hex(api::value_blob(arg));
    api::result_text(context, out)?;
    Ok(())
}

fn presto_sha256(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let arg = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument"))?;
    if api::value_is_null(arg) {
        api::result_null(context);
        return Ok(());
    }

    let out = presto_binary::hash::sha256(api::value_blob(arg));
    api::result_blob(context, &out);
    Ok(())
}

fn presto_lower(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let arg = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument"))?;
    if api::value_is_null(arg) {
        api::result_null(context);
        return Ok(());
    }

    let text = api::value_text(arg)?;
    let out = presto_text::string::lower(text);
    api::result_text(context, out)?;
    Ok(())
}

fn presto_regexp_like(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let input = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument 1"))?;
    let pattern = values
        .get(1)
        .ok_or_else(|| Error::new_message("missing argument 2"))?;

    if api::value_is_null(input) || api::value_is_null(pattern) {
        api::result_null(context);
        return Ok(());
    }

    let result =
        presto_text::regex::regexp_like(api::value_text(input)?, api::value_text(pattern)?)
            .map_err(|e| Error::new_message(e.to_string().as_str()))?;
    api::result_bool(context, result);
    Ok(())
}

fn presto_url_extract_host(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let arg = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument"))?;
    if api::value_is_null(arg) {
        api::result_null(context);
        return Ok(());
    }

    match presto_net::url::url_extract_host(api::value_text(arg)?)
        .map_err(|e| Error::new_message(e.to_string().as_str()))?
    {
        Some(host) => api::result_text(context, host)?,
        None => api::result_null(context),
    }
    Ok(())
}

fn presto_is_private_ip(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let arg = values
        .first()
        .ok_or_else(|| Error::new_message("missing argument"))?;
    if api::value_is_null(arg) {
        api::result_null(context);
        return Ok(());
    }

    let result = presto_net::ip::is_private_ip(api::value_text(arg)?)
        .map_err(|e| Error::new_message(e.to_string().as_str()))?;
    api::result_bool(context, result);
    Ok(())
}

fn presto_uuid(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, presto_net::uuid::uuid())?;
    Ok(())
}

fn register_sqlite_functions(db: *mut sqlite3, registry: &FunctionRegistry) -> Result<()> {
    let deterministic = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;
    let mut exposed = HashSet::new();

    macro_rules! expose {
        ($name:expr) => {
            exposed.insert($name.to_string());
        };
    }

    // Hand-bridged representative functions.
    define_scalar_function(db, "p_abs", 1, presto_abs, deterministic)?;
    expose!("p_abs");

    define_scalar_function(db, "p_bitwise_and", 2, presto_bitwise_and, deterministic)?;
    expose!("p_bitwise_and");

    define_scalar_function(db, "p_to_hex", 1, presto_to_hex, deterministic)?;
    expose!("p_to_hex");

    define_scalar_function(db, "p_sha256", 1, presto_sha256, deterministic)?;
    expose!("p_sha256");

    define_scalar_function(db, "p_lower", 1, presto_lower, deterministic)?;
    expose!("p_lower");

    define_scalar_function(db, "p_regexp_like", 2, presto_regexp_like, deterministic)?;
    expose!("p_regexp_like");

    define_scalar_function(
        db,
        "p_url_extract_host",
        1,
        presto_url_extract_host,
        deterministic,
    )?;
    expose!("p_url_extract_host");

    define_scalar_function(
        db,
        "p_is_private_ip",
        1,
        presto_is_private_ip,
        deterministic,
    )?;
    expose!("p_is_private_ip");

    define_scalar_function(db, "p_uuid", 0, presto_uuid, FunctionFlags::UTF8)?;
    expose!("p_uuid");

    // Numeric bridges - unary f64 -> f64.
    let unary_f64: &[(&str, fn(f64) -> f64)] = &[
        ("p_acos", presto_numeric::math::acos),
        ("p_asin", presto_numeric::math::asin),
        ("p_atan", presto_numeric::math::atan),
        ("p_cbrt", presto_numeric::math::cbrt),
        ("p_ceil", presto_numeric::math::ceil),
        ("p_ceiling", presto_numeric::math::ceiling),
        ("p_cos", presto_numeric::math::cos),
        ("p_cosh", presto_numeric::math::cosh),
        ("p_degrees", presto_numeric::math::degrees),
        ("p_exp", presto_numeric::math::exp),
        ("p_floor", presto_numeric::math::floor),
        ("p_ln", presto_numeric::math::ln),
        ("p_log10", presto_numeric::math::log10),
        ("p_log2", presto_numeric::math::log2),
        ("p_radians", presto_numeric::math::radians),
        ("p_round", presto_numeric::math::round),
        ("p_sign", presto_numeric::math::sign),
        ("p_sin", presto_numeric::math::sin),
        ("p_sqrt", presto_numeric::math::sqrt),
        ("p_tan", presto_numeric::math::tan),
        ("p_tanh", presto_numeric::math::tanh),
        ("p_truncate", presto_numeric::math::truncate),
    ];
    for &(name, func) in unary_f64 {
        define_scalar_function_with_aux(
            db,
            name,
            1,
            |context, values, func: &fn(f64) -> f64| {
                let arg = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument"))?;
                if api::value_is_null(arg) {
                    api::result_null(context);
                    return Ok(());
                }
                api::result_double(context, (*func)(api::value_double(arg)));
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    let binary_f64: &[(&str, fn(f64, f64) -> f64)] = &[
        ("p_atan2", presto_numeric::math::atan2),
        ("p_pow", presto_numeric::math::pow),
        ("p_power", presto_numeric::math::power),
    ];
    for &(name, func) in binary_f64 {
        define_scalar_function_with_aux(
            db,
            name,
            2,
            |context, values, func: &fn(f64, f64) -> f64| {
                let a = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument 1"))?;
                let b = values
                    .get(1)
                    .ok_or_else(|| Error::new_message("missing argument 2"))?;
                if api::value_is_null(a) || api::value_is_null(b) {
                    api::result_null(context);
                    return Ok(());
                }
                api::result_double(context, (*func)(api::value_double(a), api::value_double(b)));
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    let unary_bool_f64: &[(&str, fn(f64) -> bool)] = &[
        ("p_is_finite", presto_numeric::math::is_finite),
        ("p_is_infinite", presto_numeric::math::is_infinite),
        ("p_is_nan", presto_numeric::math::is_nan),
    ];
    for &(name, func) in unary_bool_f64 {
        define_scalar_function_with_aux(
            db,
            name,
            1,
            |context, values, func: &fn(f64) -> bool| {
                let arg = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument"))?;
                if api::value_is_null(arg) {
                    api::result_null(context);
                    return Ok(());
                }
                api::result_bool(context, (*func)(api::value_double(arg)));
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    let nullary_f64_det: &[(&str, fn() -> f64)] = &[
        ("p_e", presto_numeric::math::e),
        ("p_infinity", presto_numeric::math::infinity),
        ("p_nan", presto_numeric::math::nan),
        ("p_pi", presto_numeric::math::pi),
    ];
    for &(name, func) in nullary_f64_det {
        define_scalar_function_with_aux(
            db,
            name,
            0,
            |context, _values, func: &fn() -> f64| {
                api::result_double(context, (*func)());
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    let nullary_f64_nondet: &[(&str, fn() -> f64)] = &[
        ("p_rand", presto_numeric::math::rand),
        ("p_random", presto_numeric::math::random),
        ("p_secure_rand", presto_numeric::math::secure_rand),
        ("p_secure_random", presto_numeric::math::secure_random),
    ];
    for &(name, func) in nullary_f64_nondet {
        define_scalar_function_with_aux(
            db,
            name,
            0,
            |context, _values, func: &fn() -> f64| {
                api::result_double(context, (*func)());
                Ok(())
            },
            FunctionFlags::UTF8,
            func,
        )?;
        expose!(name);
    }

    define_scalar_function(
        db,
        "p_random",
        1,
        |context, values| {
            let n = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(n) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::random_bounded(api::value_int64(n))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        FunctionFlags::UTF8,
    )?;
    expose!("p_random");

    define_scalar_function(
        db,
        "p_secure_random",
        2,
        |context, values| {
            let lo = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let hi = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(lo) || api::value_is_null(hi) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::secure_random_range(
                api::value_int64(lo),
                api::value_int64(hi),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        FunctionFlags::UTF8,
    )?;
    expose!("p_secure_random");

    define_scalar_function(
        db,
        "p_round",
        2,
        |context, values| {
            let x = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let d = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(x) || api::value_is_null(d) {
                api::result_null(context);
                return Ok(());
            }
            let digits = i32::try_from(api::value_int64(d))
                .map_err(|_| Error::new_message("digits out of i32 range"))?;
            api::result_double(
                context,
                presto_numeric::math::round_n(api::value_double(x), digits),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_round");

    define_scalar_function(
        db,
        "p_truncate",
        2,
        |context, values| {
            let x = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let n = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(x) || api::value_is_null(n) {
                api::result_null(context);
                return Ok(());
            }
            let digits = i32::try_from(api::value_int64(n))
                .map_err(|_| Error::new_message("digits out of i32 range"))?;
            api::result_double(
                context,
                presto_numeric::math::truncate_n(api::value_double(x), digits),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_truncate");

    define_scalar_function(
        db,
        "p_mod",
        2,
        |context, values| {
            let n = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let m = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(n) || api::value_is_null(m) {
                api::result_null(context);
                return Ok(());
            }

            if api::value_type(n) == api::ValueType::Integer
                && api::value_type(m) == api::ValueType::Integer
            {
                let out = presto_numeric::math::mod_i64(api::value_int64(n), api::value_int64(m))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_int64(context, out);
            } else {
                let out = presto_numeric::math::mod_f64(api::value_double(n), api::value_double(m))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_double(context, out);
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_mod");

    define_scalar_function(
        db,
        "p_factorial",
        1,
        |context, values| {
            let x = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(x) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::factorial(api::value_int64(x))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_factorial");

    define_scalar_function(
        db,
        "p_from_base",
        2,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let radix = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(value) || api::value_is_null(radix) {
                api::result_null(context);
                return Ok(());
            }
            let out =
                presto_numeric::math::from_base(api::value_text(value)?, api::value_int64(radix))
                    .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_from_base");

    define_scalar_function(
        db,
        "p_to_base",
        2,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let radix = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(value) || api::value_is_null(radix) {
                api::result_null(context);
                return Ok(());
            }
            let out =
                presto_numeric::math::to_base(api::value_int64(value), api::value_int64(radix))
                    .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_to_base");

    // Bitwise additional bridges.
    define_scalar_function(
        db,
        "p_bitwise_or",
        2,
        |context, values| {
            let x = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let y = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(x) || api::value_is_null(y) {
                api::result_null(context);
                return Ok(());
            }
            api::result_int64(
                context,
                presto_numeric::bitwise::bitwise_or(api::value_int64(x), api::value_int64(y)),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_bitwise_or");

    define_scalar_function(
        db,
        "p_bitwise_xor",
        2,
        |context, values| {
            let x = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let y = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(x) || api::value_is_null(y) {
                api::result_null(context);
                return Ok(());
            }
            api::result_int64(
                context,
                presto_numeric::bitwise::bitwise_xor(api::value_int64(x), api::value_int64(y)),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_bitwise_xor");

    define_scalar_function(
        db,
        "p_bitwise_not",
        1,
        |context, values| {
            let x = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(x) {
                api::result_null(context);
                return Ok(());
            }
            api::result_int64(
                context,
                presto_numeric::bitwise::bitwise_not(api::value_int64(x)),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_bitwise_not");

    let bitwise_binary_result: &[(&str, fn(i64, i64) -> presto_core::PrestoResult<i64>)] = &[
        ("p_bit_count", presto_numeric::bitwise::bit_count),
        (
            "p_bitwise_arithmetic_shift_right",
            presto_numeric::bitwise::bitwise_arithmetic_shift_right,
        ),
        (
            "p_bitwise_left_shift",
            presto_numeric::bitwise::bitwise_left_shift,
        ),
        (
            "p_bitwise_right_shift",
            presto_numeric::bitwise::bitwise_right_shift,
        ),
        (
            "p_bitwise_right_shift_arithmetic",
            presto_numeric::bitwise::bitwise_right_shift_arithmetic,
        ),
    ];
    for &(name, func) in bitwise_binary_result {
        define_scalar_function_with_aux(
            db,
            name,
            2,
            |context, values, func: &fn(i64, i64) -> presto_core::PrestoResult<i64>| {
                let x = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument 1"))?;
                let y = values
                    .get(1)
                    .ok_or_else(|| Error::new_message("missing argument 2"))?;
                if api::value_is_null(x) || api::value_is_null(y) {
                    api::result_null(context);
                    return Ok(());
                }
                let out = (*func)(api::value_int64(x), api::value_int64(y))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_int64(context, out);
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    let bitwise_ternary_result: &[(&str, fn(i64, i64, i64) -> presto_core::PrestoResult<i64>)] = &[
        (
            "p_bitwise_logical_shift_right",
            presto_numeric::bitwise::bitwise_logical_shift_right,
        ),
        (
            "p_bitwise_shift_left",
            presto_numeric::bitwise::bitwise_shift_left,
        ),
    ];
    for &(name, func) in bitwise_ternary_result {
        define_scalar_function_with_aux(
            db,
            name,
            3,
            |context, values, func: &fn(i64, i64, i64) -> presto_core::PrestoResult<i64>| {
                let x = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument 1"))?;
                let y = values
                    .get(1)
                    .ok_or_else(|| Error::new_message("missing argument 2"))?;
                let z = values
                    .get(2)
                    .ok_or_else(|| Error::new_message("missing argument 3"))?;
                if api::value_is_null(x) || api::value_is_null(y) || api::value_is_null(z) {
                    api::result_null(context);
                    return Ok(());
                }
                let out = (*func)(
                    api::value_int64(x),
                    api::value_int64(y),
                    api::value_int64(z),
                )
                .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_int64(context, out);
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    // Distribution and statistics bridges.
    let binary_f64_result: &[(&str, fn(f64, f64) -> presto_core::PrestoResult<f64>)] = &[
        ("p_chi_squared_cdf", presto_numeric::math::chi_squared_cdf),
        (
            "p_inverse_chi_squared_cdf",
            presto_numeric::math::inverse_chi_squared_cdf,
        ),
        ("p_t_cdf", presto_numeric::math::t_cdf),
        ("p_inverse_t_cdf", presto_numeric::math::inverse_t_cdf),
    ];
    for &(name, func) in binary_f64_result {
        define_scalar_function_with_aux(
            db,
            name,
            2,
            |context, values, func: &fn(f64, f64) -> presto_core::PrestoResult<f64>| {
                let a = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument 1"))?;
                let b = values
                    .get(1)
                    .ok_or_else(|| Error::new_message("missing argument 2"))?;
                if api::value_is_null(a) || api::value_is_null(b) {
                    api::result_null(context);
                    return Ok(());
                }
                let out = (*func)(api::value_double(a), api::value_double(b))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_double(context, out);
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    let ternary_f64_result: &[(&str, fn(f64, f64, f64) -> presto_core::PrestoResult<f64>)] = &[
        ("p_beta_cdf", presto_numeric::math::beta_cdf),
        ("p_cauchy_cdf", presto_numeric::math::cauchy_cdf),
        ("p_f_cdf", presto_numeric::math::f_cdf),
        ("p_gamma_cdf", presto_numeric::math::gamma_cdf),
        ("p_laplace_cdf", presto_numeric::math::laplace_cdf),
        ("p_normal_cdf", presto_numeric::math::normal_cdf),
        ("p_weibull_cdf", presto_numeric::math::weibull_cdf),
        ("p_inverse_beta_cdf", presto_numeric::math::inverse_beta_cdf),
        (
            "p_inverse_cauchy_cdf",
            presto_numeric::math::inverse_cauchy_cdf,
        ),
        ("p_inverse_f_cdf", presto_numeric::math::inverse_f_cdf),
        (
            "p_inverse_gamma_cdf",
            presto_numeric::math::inverse_gamma_cdf,
        ),
        (
            "p_inverse_laplace_cdf",
            presto_numeric::math::inverse_laplace_cdf,
        ),
        (
            "p_inverse_normal_cdf",
            presto_numeric::math::inverse_normal_cdf,
        ),
        (
            "p_inverse_weibull_cdf",
            presto_numeric::math::inverse_weibull_cdf,
        ),
        (
            "p_wilson_interval_lower",
            presto_numeric::math::wilson_interval_lower,
        ),
        (
            "p_wilson_interval_upper",
            presto_numeric::math::wilson_interval_upper,
        ),
    ];
    for &(name, func) in ternary_f64_result {
        define_scalar_function_with_aux(
            db,
            name,
            3,
            |context, values, func: &fn(f64, f64, f64) -> presto_core::PrestoResult<f64>| {
                let a = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument 1"))?;
                let b = values
                    .get(1)
                    .ok_or_else(|| Error::new_message("missing argument 2"))?;
                let c = values
                    .get(2)
                    .ok_or_else(|| Error::new_message("missing argument 3"))?;
                if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                    api::result_null(context);
                    return Ok(());
                }
                let out = (*func)(
                    api::value_double(a),
                    api::value_double(b),
                    api::value_double(c),
                )
                .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_double(context, out);
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    define_scalar_function(
        db,
        "p_binomial_cdf",
        3,
        |context, values| {
            let n = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let p = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let v = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(n) || api::value_is_null(p) || api::value_is_null(v) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::binomial_cdf(
                api::value_int64(n),
                api::value_double(p),
                api::value_int64(v),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_double(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_binomial_cdf");

    define_scalar_function(
        db,
        "p_inverse_binomial_cdf",
        3,
        |context, values| {
            let n = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let p = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let prob = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(n) || api::value_is_null(p) || api::value_is_null(prob) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::inverse_binomial_cdf(
                api::value_int64(n),
                api::value_double(p),
                api::value_double(prob),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_inverse_binomial_cdf");

    define_scalar_function(
        db,
        "p_poisson_cdf",
        2,
        |context, values| {
            let lambda = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let value = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(lambda) || api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::poisson_cdf(
                api::value_double(lambda),
                api::value_int64(value),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_double(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_poisson_cdf");

    define_scalar_function(
        db,
        "p_inverse_poisson_cdf",
        2,
        |context, values| {
            let lambda = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let prob = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(lambda) || api::value_is_null(prob) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::inverse_poisson_cdf(
                api::value_double(lambda),
                api::value_double(prob),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_inverse_poisson_cdf");

    define_scalar_function(
        db,
        "p_width_bucket",
        4,
        |context, values| {
            let x = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b1 = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let b2 = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            let n = values
                .get(3)
                .ok_or_else(|| Error::new_message("missing argument 4"))?;
            if api::value_is_null(x)
                || api::value_is_null(b1)
                || api::value_is_null(b2)
                || api::value_is_null(n)
            {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_numeric::math::width_bucket_range(
                api::value_double(x),
                api::value_double(b1),
                api::value_double(b2),
                api::value_int64(n),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_width_bucket");

    // URL + IP bridges.
    define_scalar_function(
        db,
        "p_url_encode",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_net::url::url_encode(api::value_text(value)?),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_encode");

    define_scalar_function(
        db,
        "p_url_decode",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_net::url::url_decode(api::value_text(value)?)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_decode");

    define_scalar_function(
        db,
        "p_url_extract_fragment",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            match presto_net::url::url_extract_fragment(api::value_text(value)?)
                .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_extract_fragment");

    define_scalar_function(
        db,
        "p_url_extract_parameter",
        2,
        |context, values| {
            let url = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let name = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(url) || api::value_is_null(name) {
                api::result_null(context);
                return Ok(());
            }
            match presto_net::url::url_extract_parameter(
                api::value_text(url)?,
                api::value_text(name)?,
            )
            .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_extract_parameter");

    define_scalar_function(
        db,
        "p_url_extract_path",
        1,
        |context, values| {
            let url = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(url) {
                api::result_null(context);
                return Ok(());
            }
            match presto_net::url::url_extract_path(api::value_text(url)?)
                .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_extract_path");

    define_scalar_function(
        db,
        "p_url_extract_port",
        1,
        |context, values| {
            let url = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(url) {
                api::result_null(context);
                return Ok(());
            }
            match presto_net::url::url_extract_port(api::value_text(url)?)
                .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_int64(context, v),
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_extract_port");

    define_scalar_function(
        db,
        "p_url_extract_protocol",
        1,
        |context, values| {
            let url = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(url) {
                api::result_null(context);
                return Ok(());
            }
            match presto_net::url::url_extract_protocol(api::value_text(url)?)
                .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_extract_protocol");

    define_scalar_function(
        db,
        "p_url_extract_query",
        1,
        |context, values| {
            let url = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(url) {
                api::result_null(context);
                return Ok(());
            }
            match presto_net::url::url_extract_query(api::value_text(url)?)
                .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_url_extract_query");

    define_scalar_function(
        db,
        "p_ip_prefix",
        2,
        |context, values| {
            let ip = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let bits = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(ip) || api::value_is_null(bits) {
                api::result_null(context);
                return Ok(());
            }
            let bits = u8::try_from(api::value_int64(bits))
                .map_err(|_| Error::new_message("prefix_bits out of u8 range"))?;
            let out = presto_net::ip::ip_prefix(api::value_text(ip)?, bits)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ip_prefix");

    define_scalar_function(
        db,
        "p_ip_prefix_collapse",
        1,
        |context, values| {
            let input = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(input) {
                api::result_null(context);
                return Ok(());
            }
            let prefixes = api::value_text(input)?
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>();
            let out = presto_net::ip::ip_prefix_collapse(&prefixes)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ip_prefix_collapse");

    define_scalar_function(
        db,
        "p_ip_prefix_subnets",
        2,
        |context, values| {
            let prefix = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let new_len = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(prefix) || api::value_is_null(new_len) {
                api::result_null(context);
                return Ok(());
            }
            let new_len = u8::try_from(api::value_int64(new_len))
                .map_err(|_| Error::new_message("prefix_length out of u8 range"))?;
            let out = presto_net::ip::ip_prefix_subnets(api::value_text(prefix)?, new_len)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ip_prefix_subnets");

    define_scalar_function(
        db,
        "p_ip_subnet_min",
        1,
        |context, values| {
            let prefix = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(prefix) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_net::ip::ip_subnet_min(api::value_text(prefix)?)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ip_subnet_min");

    define_scalar_function(
        db,
        "p_ip_subnet_max",
        1,
        |context, values| {
            let prefix = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(prefix) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_net::ip::ip_subnet_max(api::value_text(prefix)?)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ip_subnet_max");

    define_scalar_function(
        db,
        "p_ip_subnet_range",
        1,
        |context, values| {
            let prefix = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(prefix) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_net::ip::ip_subnet_range(api::value_text(prefix)?)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ip_subnet_range");

    define_scalar_function(
        db,
        "p_is_subnet_of",
        2,
        |context, values| {
            let p1 = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let p2 = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(p1) || api::value_is_null(p2) {
                api::result_null(context);
                return Ok(());
            }
            let first = api::value_text(p1)?;
            let second = api::value_text(p2)?;
            let out = if second.contains('/') {
                presto_net::ip::is_subnet_of_prefix_prefix(first, second)
            } else {
                presto_net::ip::is_subnet_of_prefix_ip(first, second)
            }
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_bool(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_is_subnet_of");

    // Binary + mixed text/binary bridges.
    define_scalar_function(
        db,
        "p_concat",
        -1,
        |context, values| {
            if values.is_empty() {
                api::result_null(context);
                return Ok(());
            }
            if values.iter().any(api::value_is_null) {
                api::result_null(context);
                return Ok(());
            }

            if api::value_type(&values[0]) == api::ValueType::Blob {
                let parts = values.iter().map(api::value_blob).collect::<Vec<_>>();
                let out = presto_binary::blob::concat(&parts);
                api::result_blob(context, &out);
            } else {
                let mut parts = Vec::with_capacity(values.len());
                for value in values {
                    parts.push(api::value_text(value)?);
                }
                let out = presto_text::string::concat(&parts);
                api::result_text(context, out)?;
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_concat");

    define_scalar_function(
        db,
        "p_length",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }

            if api::value_type(value) == api::ValueType::Blob {
                api::result_int64(context, presto_binary::blob::length(api::value_blob(value)));
            } else {
                api::result_int64(
                    context,
                    presto_text::string::length(api::value_text(value)?),
                );
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_length");

    define_scalar_function(
        db,
        "p_reverse",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            if api::value_type(value) == api::ValueType::Blob {
                let out = presto_binary::blob::reverse(api::value_blob(value));
                api::result_blob(context, &out);
            } else {
                let out = presto_text::string::reverse(api::value_text(value)?);
                api::result_text(context, out)?;
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_reverse");

    define_scalar_function(
        db,
        "p_lpad",
        3,
        |context, values| {
            let input = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let size = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let pad = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(input) || api::value_is_null(size) || api::value_is_null(pad) {
                api::result_null(context);
                return Ok(());
            }
            let size = api::value_int64(size);
            if api::value_type(input) == api::ValueType::Blob {
                let out =
                    presto_binary::blob::lpad(api::value_blob(input), size, api::value_blob(pad))
                        .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_blob(context, &out);
            } else {
                let out =
                    presto_text::string::lpad(api::value_text(input)?, size, api::value_text(pad)?)
                        .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_text(context, out)?;
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_lpad");

    define_scalar_function(
        db,
        "p_rpad",
        3,
        |context, values| {
            let input = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let size = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let pad = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(input) || api::value_is_null(size) || api::value_is_null(pad) {
                api::result_null(context);
                return Ok(());
            }
            let size = api::value_int64(size);
            if api::value_type(input) == api::ValueType::Blob {
                let out =
                    presto_binary::blob::rpad(api::value_blob(input), size, api::value_blob(pad))
                        .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_blob(context, &out);
            } else {
                let out =
                    presto_text::string::rpad(api::value_text(input)?, size, api::value_text(pad)?)
                        .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_text(context, out)?;
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_rpad");

    define_scalar_function(
        db,
        "p_substr",
        2,
        |context, values| {
            let input = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let start = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(input) || api::value_is_null(start) {
                api::result_null(context);
                return Ok(());
            }
            let start = api::value_int64(start);
            if api::value_type(input) == api::ValueType::Blob {
                let out = presto_binary::blob::substr(api::value_blob(input), start, None)
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_blob(context, &out);
            } else {
                let out = presto_text::string::substr(api::value_text(input)?, start, None)
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_text(context, out)?;
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_substr");

    define_scalar_function(
        db,
        "p_substr",
        3,
        |context, values| {
            let input = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let start = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let len = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(input) || api::value_is_null(start) || api::value_is_null(len) {
                api::result_null(context);
                return Ok(());
            }
            let start = api::value_int64(start);
            let len = api::value_int64(len);
            if api::value_type(input) == api::ValueType::Blob {
                let out = presto_binary::blob::substr(api::value_blob(input), start, Some(len))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_blob(context, &out);
            } else {
                let out = presto_text::string::substr(api::value_text(input)?, start, Some(len))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_text(context, out)?;
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_substr");

    let unary_blob_to_text: &[(&str, fn(&[u8]) -> String)] = &[
        ("p_to_base32", presto_binary::encoding::to_base32),
        ("p_to_base64", presto_binary::encoding::to_base64),
        ("p_to_base64url", presto_binary::encoding::to_base64url),
    ];
    for &(name, func) in unary_blob_to_text {
        define_scalar_function_with_aux(
            db,
            name,
            1,
            |context, values, func: &fn(&[u8]) -> String| {
                let value = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument"))?;
                if api::value_is_null(value) {
                    api::result_null(context);
                    return Ok(());
                }
                api::result_text(context, (*func)(api::value_blob(value)))?;
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    let unary_text_to_blob: &[(&str, fn(&str) -> presto_core::PrestoResult<Vec<u8>>)] = &[
        ("p_from_base32", presto_binary::encoding::from_base32),
        ("p_from_base64", presto_binary::encoding::from_base64),
        ("p_from_base64url", presto_binary::encoding::from_base64url),
        ("p_from_hex", presto_binary::encoding::from_hex),
    ];
    for &(name, func) in unary_text_to_blob {
        define_scalar_function_with_aux(
            db,
            name,
            1,
            |context, values, func: &fn(&str) -> presto_core::PrestoResult<Vec<u8>>| {
                let value = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument"))?;
                if api::value_is_null(value) {
                    api::result_null(context);
                    return Ok(());
                }
                let out = (*func)(api::value_text(value)?)
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_blob(context, &out);
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    define_scalar_function(
        db,
        "p_crc32",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            api::result_int64(context, presto_binary::blob::crc32(api::value_blob(value)));
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_crc32");

    define_scalar_function(
        db,
        "p_md5",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::hash::md5(api::value_blob(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_md5");

    define_scalar_function(
        db,
        "p_sha1",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::hash::sha1(api::value_blob(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_sha1");

    define_scalar_function(
        db,
        "p_sha512",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::hash::sha512(api::value_blob(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_sha512");

    let hmac_funcs: &[(&str, fn(&[u8], &[u8]) -> presto_core::PrestoResult<Vec<u8>>)] = &[
        ("p_hmac_md5", presto_binary::hash::hmac_md5),
        ("p_hmac_sha1", presto_binary::hash::hmac_sha1),
        ("p_hmac_sha256", presto_binary::hash::hmac_sha256),
        ("p_hmac_sha512", presto_binary::hash::hmac_sha512),
    ];
    for &(name, func) in hmac_funcs {
        define_scalar_function_with_aux(
            db,
            name,
            2,
            |context, values, func: &fn(&[u8], &[u8]) -> presto_core::PrestoResult<Vec<u8>>| {
                let value = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument 1"))?;
                let key = values
                    .get(1)
                    .ok_or_else(|| Error::new_message("missing argument 2"))?;
                if api::value_is_null(value) || api::value_is_null(key) {
                    api::result_null(context);
                    return Ok(());
                }
                let out = (*func)(api::value_blob(value), api::value_blob(key))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_blob(context, &out);
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    define_scalar_function(
        db,
        "p_murmur3_x64_128",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::hash::murmur3_x64_128(api::value_blob(value))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_murmur3_x64_128");

    define_scalar_function(
        db,
        "p_xxhash64",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::hash::xxhash64(api::value_blob(value), None);
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_xxhash64");

    define_scalar_function(
        db,
        "p_xxhash64",
        2,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let seed = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(value) || api::value_is_null(seed) {
                api::result_null(context);
                return Ok(());
            }
            let out =
                presto_binary::hash::xxhash64(api::value_blob(value), Some(api::value_int64(seed)));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_xxhash64");

    define_scalar_function(
        db,
        "p_spooky_hash_v2_32",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::hash::spooky_hash_v2_32(api::value_blob(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_spooky_hash_v2_32");

    define_scalar_function(
        db,
        "p_spooky_hash_v2_64",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::hash::spooky_hash_v2_64(api::value_blob(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_spooky_hash_v2_64");

    let blob_to_i64_funcs: &[(&str, fn(&[u8]) -> presto_core::PrestoResult<i64>)] =
        &[("p_from_big_endian_64", |b| {
            presto_binary::blob::from_big_endian_64(b)
        })];
    for &(name, func) in blob_to_i64_funcs {
        define_scalar_function_with_aux(
            db,
            name,
            1,
            |context, values, func: &fn(&[u8]) -> presto_core::PrestoResult<i64>| {
                let value = values
                    .first()
                    .ok_or_else(|| Error::new_message("missing argument"))?;
                if api::value_is_null(value) {
                    api::result_null(context);
                    return Ok(());
                }
                let out = (*func)(api::value_blob(value))
                    .map_err(|e| Error::new_message(e.to_string()))?;
                api::result_int64(context, out);
                Ok(())
            },
            deterministic,
            func,
        )?;
        expose!(name);
    }

    define_scalar_function(
        db,
        "p_from_big_endian_32",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::blob::from_big_endian_32(api::value_blob(value))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_from_big_endian_32");

    define_scalar_function(
        db,
        "p_to_big_endian_32",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::blob::to_big_endian_32(api::value_int(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_to_big_endian_32");

    define_scalar_function(
        db,
        "p_to_big_endian_64",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::blob::to_big_endian_64(api::value_int64(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_to_big_endian_64");

    define_scalar_function(
        db,
        "p_from_ieee754_32",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::blob::from_ieee754_32(api::value_blob(value))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_double(context, out as f64);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_from_ieee754_32");

    define_scalar_function(
        db,
        "p_from_ieee754_64",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::blob::from_ieee754_64(api::value_blob(value))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_double(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_from_ieee754_64");

    define_scalar_function(
        db,
        "p_to_ieee754_32",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::blob::to_ieee754_32(api::value_double(value) as f32);
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_to_ieee754_32");

    define_scalar_function(
        db,
        "p_to_ieee754_64",
        1,
        |context, values| {
            let value = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(value) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_binary::blob::to_ieee754_64(api::value_double(value));
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_to_ieee754_64");

    // Text + regex bridges.
    define_scalar_function(
        db,
        "p_bit_length",
        1,
        |context, values| {
            let input = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(input) {
                api::result_null(context);
                return Ok(());
            }
            api::result_int64(
                context,
                presto_text::string::bit_length(api::value_text(input)?),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_bit_length");

    define_scalar_function(
        db,
        "p_chr",
        1,
        |context, values| {
            let n = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(n) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::chr(api::value_int64(n))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_chr");

    define_scalar_function(
        db,
        "p_codepoint",
        1,
        |context, values| {
            let text = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(text) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::codepoint(api::value_text(text)?)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_codepoint");

    define_scalar_function(
        db,
        "p_ends_with",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_bool(
                context,
                presto_text::string::ends_with(api::value_text(a)?, api::value_text(b)?),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ends_with");

    define_scalar_function(
        db,
        "p_starts_with",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_bool(
                context,
                presto_text::string::starts_with(api::value_text(a)?, api::value_text(b)?),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_starts_with");

    define_scalar_function(
        db,
        "p_from_utf8",
        1,
        |context, values| {
            let b = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::from_utf8(api::value_blob(b))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_from_utf8");

    define_scalar_function(
        db,
        "p_from_utf8",
        2,
        |context, values| {
            let b = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let rep = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(b) || api::value_is_null(rep) {
                api::result_null(context);
                return Ok(());
            }
            let out =
                presto_text::string::from_utf8_replace(api::value_blob(b), api::value_text(rep)?);
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_from_utf8");

    define_scalar_function(
        db,
        "p_hamming_distance",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out =
                presto_text::string::hamming_distance(api::value_text(a)?, api::value_text(b)?)
                    .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_hamming_distance");

    define_scalar_function(
        db,
        "p_jarowinkler_similarity",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_double(
                context,
                presto_text::string::jarowinkler_similarity(
                    api::value_text(a)?,
                    api::value_text(b)?,
                ),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_jarowinkler_similarity");

    define_scalar_function(
        db,
        "p_key_sampling_percent",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            api::result_double(
                context,
                presto_text::string::key_sampling_percent(api::value_text(a)?),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_key_sampling_percent");

    define_scalar_function(
        db,
        "p_levenshtein_distance",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_int64(
                context,
                presto_text::string::levenshtein_distance(api::value_text(a)?, api::value_text(b)?),
            );
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_levenshtein_distance");

    define_scalar_function(
        db,
        "p_longest_common_prefix",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::longest_common_prefix(
                    api::value_text(a)?,
                    api::value_text(b)?,
                ),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_longest_common_prefix");

    define_scalar_function(
        db,
        "p_ltrim",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::ltrim(api::value_text(a)?, None),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_ltrim",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::ltrim(api::value_text(a)?, Some(api::value_text(b)?)),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_ltrim");

    define_scalar_function(
        db,
        "p_rtrim",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::rtrim(api::value_text(a)?, None),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_rtrim",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::rtrim(api::value_text(a)?, Some(api::value_text(b)?)),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_rtrim");

    define_scalar_function(
        db,
        "p_trim",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::trim(api::value_text(a)?, None),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_trim",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::trim(api::value_text(a)?, Some(api::value_text(b)?)),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_trim");

    define_scalar_function(
        db,
        "p_normalize",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::normalize(api::value_text(a)?, None)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_normalize",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out =
                presto_text::string::normalize(api::value_text(a)?, Some(api::value_text(b)?))
                    .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_normalize");

    define_scalar_function(
        db,
        "p_replace",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::replace(api::value_text(a)?, api::value_text(b)?, None),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_replace",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::replace(
                    api::value_text(a)?,
                    api::value_text(b)?,
                    Some(api::value_text(c)?),
                ),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_replace");

    define_scalar_function(
        db,
        "p_replace_first",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::replace_first(
                    api::value_text(a)?,
                    api::value_text(b)?,
                    api::value_text(c)?,
                ),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_replace_first");

    define_scalar_function(
        db,
        "p_split",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::split(api::value_text(a)?, api::value_text(b)?, None)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_split",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::split(
                api::value_text(a)?,
                api::value_text(b)?,
                Some(api::value_int64(c)),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_split");

    define_scalar_function(
        db,
        "p_split_part",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            match presto_text::string::split_part(
                api::value_text(a)?,
                api::value_text(b)?,
                api::value_int64(c),
            )
            .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_split_part");

    define_scalar_function(
        db,
        "p_split_to_multimap",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::split_to_multimap(
                api::value_text(a)?,
                api::value_text(b)?,
                api::value_text(c)?,
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            let encoded = out
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v.join("|")))
                .collect::<Vec<_>>()
                .join(";");
            api::result_text(context, encoded)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_split_to_multimap");

    define_scalar_function(
        db,
        "p_strpos",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::strpos(api::value_text(a)?, api::value_text(b)?, None)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_strpos",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::strpos(
                api::value_text(a)?,
                api::value_text(b)?,
                Some(api::value_int64(c)),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_strpos");

    define_scalar_function(
        db,
        "p_strrpos",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::strrpos(api::value_text(a)?, api::value_text(b)?, None)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_strrpos",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::strrpos(
                api::value_text(a)?,
                api::value_text(b)?,
                Some(api::value_int64(c)),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_int64(context, out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_strrpos");

    define_scalar_function(
        db,
        "p_to_utf8",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::to_utf8(api::value_text(a)?);
            api::result_blob(context, &out);
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_to_utf8");

    define_scalar_function(
        db,
        "p_trail",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let n = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(n) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::string::trail(api::value_text(a)?, api::value_int64(n))
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_trail");

    define_scalar_function(
        db,
        "p_upper",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(context, presto_text::string::upper(api::value_text(a)?))?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_upper");

    define_scalar_function(
        db,
        "p_word_stem",
        1,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument"))?;
            if api::value_is_null(a) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::word_stem(api::value_text(a)?, None),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_word_stem",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            api::result_text(
                context,
                presto_text::string::word_stem(api::value_text(a)?, Some(api::value_text(b)?)),
            )?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_word_stem");

    define_scalar_function(
        db,
        "p_regexp_extract",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            match presto_text::regex::regexp_extract(api::value_text(a)?, api::value_text(b)?, None)
                .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_regexp_extract",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let g = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(g) {
                api::result_null(context);
                return Ok(());
            }
            let group = usize::try_from(api::value_int64(g))
                .map_err(|_| Error::new_message("group out of range"))?;
            match presto_text::regex::regexp_extract(
                api::value_text(a)?,
                api::value_text(b)?,
                Some(group),
            )
            .map_err(|e| Error::new_message(e.to_string()))?
            {
                Some(v) => api::result_text(context, v)?,
                None => api::result_null(context),
            }
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_regexp_extract");

    define_scalar_function(
        db,
        "p_regexp_extract_all",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::regex::regexp_extract_all(
                api::value_text(a)?,
                api::value_text(b)?,
                None,
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_regexp_extract_all",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let g = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(g) {
                api::result_null(context);
                return Ok(());
            }
            let group = usize::try_from(api::value_int64(g))
                .map_err(|_| Error::new_message("group out of range"))?;
            let out = presto_text::regex::regexp_extract_all(
                api::value_text(a)?,
                api::value_text(b)?,
                Some(group),
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_regexp_extract_all");

    define_scalar_function(
        db,
        "p_regexp_replace",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::regex::regexp_replace(api::value_text(a)?, api::value_text(b)?)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    define_scalar_function(
        db,
        "p_regexp_replace",
        3,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            let c = values
                .get(2)
                .ok_or_else(|| Error::new_message("missing argument 3"))?;
            if api::value_is_null(a) || api::value_is_null(b) || api::value_is_null(c) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::regex::regexp_replace_with(
                api::value_text(a)?,
                api::value_text(b)?,
                api::value_text(c)?,
            )
            .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out)?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_regexp_replace");

    define_scalar_function(
        db,
        "p_regexp_split",
        2,
        |context, values| {
            let a = values
                .first()
                .ok_or_else(|| Error::new_message("missing argument 1"))?;
            let b = values
                .get(1)
                .ok_or_else(|| Error::new_message("missing argument 2"))?;
            if api::value_is_null(a) || api::value_is_null(b) {
                api::result_null(context);
                return Ok(());
            }
            let out = presto_text::regex::regexp_split(api::value_text(a)?, api::value_text(b)?)
                .map_err(|e| Error::new_message(e.to_string()))?;
            api::result_text(context, out.join(","))?;
            Ok(())
        },
        deterministic,
    )?;
    expose!("p_regexp_split");

    for raw_name in registry
        .scalar_functions()
        .iter()
        .chain(registry.window_functions().iter())
    {
        let sql_name = format!("p_{raw_name}");
        if exposed.contains(&sql_name) {
            continue;
        }

        define_scalar_function_with_aux(
            db,
            &sql_name,
            -1,
            |_context: *mut sqlite3_context, values: &[*mut sqlite3_value], fn_name: &String| {
                Err(Error::new_message(format!(
                    "{fn_name} is exposed in SQL, but its SQLite argument/result bridge is not implemented yet (argc={}).",
                    values.len()
                )))
            },
            FunctionFlags::UTF8,
            sql_name.clone(),
        )?;
        exposed.insert(sql_name);
    }

    Ok(())
}

fn initialize(db: *mut sqlite3) -> Result<()> {
    let mut registry = FunctionRegistry::default();
    register_all(&mut registry).map_err(|e| Error::new_message(e.to_string().as_str()))?;
    register_sqlite_functions(db, &registry)?;
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_extension_init(db: *mut sqlite3) -> Result<()> {
    initialize(db)
}

#[sqlite_entrypoint]
pub fn sqlite3_presto_ext_init(db: *mut sqlite3) -> Result<()> {
    initialize(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registration_pipeline_collects_functions() {
        let mut registry = FunctionRegistry::default();
        register_all(&mut registry).unwrap();
        assert!(!registry.scalar_functions().is_empty());
        assert!(!registry.window_functions().is_empty());
    }
}
