use presto_core::{FunctionRegistry, PrestoResult};

pub mod bitwise {
    use presto_core::{PrestoError, PrestoResult};

    fn normalize_shift(shift: i64) -> PrestoResult<u32> {
        if shift < 0 {
            return Err(PrestoError::InvalidArgument("shift must be >= 0"));
        }
        Ok((shift as u32).min(63))
    }

    fn normalize_bits(bits: i64) -> PrestoResult<u32> {
        if !(1..=64).contains(&bits) {
            return Err(PrestoError::InvalidArgument("bits must be in [1, 64]"));
        }
        Ok(bits as u32)
    }

    fn mask_for_bits(bits: u32) -> u64 {
        if bits == 64 {
            u64::MAX
        } else {
            (1u64 << bits) - 1
        }
    }

    pub fn bit_count(x: i64, bits: i64) -> PrestoResult<i64> {
        let bits = normalize_bits(bits)?;
        let mask = mask_for_bits(bits);
        Ok(((x as u64) & mask).count_ones() as i64)
    }

    pub fn bitwise_and(x: i64, y: i64) -> i64 {
        x & y
    }

    pub fn bitwise_or(x: i64, y: i64) -> i64 {
        x | y
    }

    pub fn bitwise_xor(x: i64, y: i64) -> i64 {
        x ^ y
    }

    pub fn bitwise_not(x: i64) -> i64 {
        !x
    }

    pub fn bitwise_arithmetic_shift_right(x: i64, shift: i64) -> PrestoResult<i64> {
        let shift = normalize_shift(shift)?;
        Ok(x >> shift)
    }

    pub fn bitwise_right_shift(value: i64, shift: i64) -> PrestoResult<i64> {
        bitwise_arithmetic_shift_right(value, shift)
    }

    pub fn bitwise_right_shift_arithmetic(value: i64, shift: i64) -> PrestoResult<i64> {
        bitwise_arithmetic_shift_right(value, shift)
    }

    pub fn bitwise_left_shift(value: i64, shift: i64) -> PrestoResult<i64> {
        let shift = normalize_shift(shift)?;
        Ok(value.wrapping_shl(shift))
    }

    pub fn bitwise_logical_shift_right(x: i64, shift: i64, bits: i64) -> PrestoResult<i64> {
        let shift = normalize_shift(shift)?;
        let bits = normalize_bits(bits)?;
        let mask = mask_for_bits(bits);
        Ok((((x as u64) & mask) >> shift) as i64)
    }

    pub fn bitwise_shift_left(x: i64, shift: i64, bits: i64) -> PrestoResult<i64> {
        let shift = normalize_shift(shift)?;
        let bits = normalize_bits(bits)?;
        let mask = mask_for_bits(bits);
        Ok((((x as u64) << shift) & mask) as i64)
    }
}

pub mod math {
    use rand::rngs::OsRng;
    use rand::{Rng, RngCore};
    use statrs::distribution::{
        Beta, Binomial, Cauchy, ChiSquared, ContinuousCDF, DiscreteCDF, FisherSnedecor, Gamma,
        Laplace, Normal, Poisson, StudentsT, Weibull,
    };

    use presto_core::{PrestoError, PrestoResult};

    fn checked_prob(p: f64) -> PrestoResult<f64> {
        if !(0.0..=1.0).contains(&p) || p.is_nan() {
            return Err(PrestoError::Domain("probability must be in [0,1]"));
        }
        Ok(p)
    }

    fn checked_positive(name: &'static str, value: f64) -> PrestoResult<f64> {
        if value <= 0.0 || value.is_nan() {
            return Err(PrestoError::Domain(name));
        }
        Ok(value)
    }

    pub fn abs_i64(v: i64) -> PrestoResult<i64> {
        v.checked_abs()
            .ok_or(PrestoError::Overflow("abs(i64::MIN) overflows"))
    }

    pub fn abs_f64(v: f64) -> f64 {
        v.abs()
    }

    pub fn acos(x: f64) -> f64 {
        x.acos()
    }

    pub fn asin(x: f64) -> f64 {
        x.asin()
    }

    pub fn atan(x: f64) -> f64 {
        x.atan()
    }

    pub fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    pub fn cbrt(x: f64) -> f64 {
        x.cbrt()
    }

    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }

    pub fn ceiling(x: f64) -> f64 {
        ceil(x)
    }

    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    pub fn cosh(x: f64) -> f64 {
        x.cosh()
    }

    pub fn degrees(x: f64) -> f64 {
        x.to_degrees()
    }

    pub fn dot_product(x: &[f64], y: &[f64]) -> PrestoResult<f64> {
        if x.len() != y.len() {
            return Err(PrestoError::InvalidArgument(
                "vectors must have same length",
            ));
        }
        Ok(x.iter().zip(y).map(|(a, b)| a * b).sum())
    }

    pub fn cosine_similarity(x: &[f64], y: &[f64]) -> PrestoResult<f64> {
        let dot = dot_product(x, y)?;
        let norm_x: f64 = x.iter().map(|v| v * v).sum::<f64>().sqrt();
        let norm_y: f64 = y.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm_x == 0.0 || norm_y == 0.0 {
            return Err(PrestoError::Domain(
                "cosine similarity undefined for zero vectors",
            ));
        }
        Ok(dot / (norm_x * norm_y))
    }

    pub fn l2_squared(x: &[f64], y: &[f64]) -> PrestoResult<f64> {
        if x.len() != y.len() {
            return Err(PrestoError::InvalidArgument(
                "vectors must have same length",
            ));
        }
        Ok(x.iter().zip(y).map(|(a, b)| (a - b) * (a - b)).sum())
    }

    pub fn e() -> f64 {
        std::f64::consts::E
    }

    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    pub fn factorial(x: i64) -> PrestoResult<i64> {
        if x < 0 {
            return Err(PrestoError::Domain("factorial input must be >= 0"));
        }
        let x = x as u128;
        let mut out: u128 = 1;
        for i in 2..=x {
            out = out
                .checked_mul(i)
                .ok_or(PrestoError::Overflow("factorial overflow"))?;
        }
        i64::try_from(out).map_err(|_| PrestoError::Overflow("factorial overflow"))
    }

    pub fn floor(x: f64) -> f64 {
        x.floor()
    }

    pub fn from_base(value: &str, radix: i64) -> PrestoResult<i64> {
        if !(2..=36).contains(&radix) {
            return Err(PrestoError::Domain("radix must be in [2,36]"));
        }
        i64::from_str_radix(value.trim(), radix as u32)
            .map_err(|_| PrestoError::InvalidArgument("invalid value for radix"))
    }

    pub fn to_base(x: i64, radix: i64) -> PrestoResult<String> {
        if !(2..=36).contains(&radix) {
            return Err(PrestoError::Domain("radix must be in [2,36]"));
        }
        let negative = x.is_negative();
        let mut n = x.unsigned_abs() as u128;
        let r = radix as u128;
        let mut chars = Vec::new();
        if n == 0 {
            chars.push('0');
        }
        while n > 0 {
            let digit = (n % r) as u8;
            let c = match digit {
                0..=9 => (b'0' + digit) as char,
                _ => (b'a' + (digit - 10)) as char,
            };
            chars.push(c);
            n /= r;
        }
        if negative {
            chars.push('-');
        }
        chars.reverse();
        Ok(chars.into_iter().collect())
    }

    pub fn infinity() -> f64 {
        f64::INFINITY
    }

    pub fn nan() -> f64 {
        f64::NAN
    }

    pub fn is_finite(x: f64) -> bool {
        x.is_finite()
    }

    pub fn is_infinite(x: f64) -> bool {
        x.is_infinite()
    }

    pub fn is_nan(x: f64) -> bool {
        x.is_nan()
    }

    pub fn laplace_cdf(mean: f64, scale: f64, value: f64) -> PrestoResult<f64> {
        let d = Laplace::new(mean, checked_positive("scale must be > 0", scale)?)
            .map_err(|_| PrestoError::Domain("invalid Laplace params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_laplace_cdf(mean: f64, scale: f64, p: f64) -> PrestoResult<f64> {
        let d = Laplace::new(mean, checked_positive("scale must be > 0", scale)?)
            .map_err(|_| PrestoError::Domain("invalid Laplace params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn ln(x: f64) -> f64 {
        x.ln()
    }

    pub fn log10(x: f64) -> f64 {
        x.log10()
    }

    pub fn log2(x: f64) -> f64 {
        x.log2()
    }

    pub fn mod_i64(n: i64, m: i64) -> PrestoResult<i64> {
        if m == 0 {
            return Err(PrestoError::Domain("mod by zero"));
        }
        Ok(n % m)
    }

    pub fn mod_f64(n: f64, m: f64) -> PrestoResult<f64> {
        if m == 0.0 {
            return Err(PrestoError::Domain("mod by zero"));
        }
        Ok(n % m)
    }

    pub fn pi() -> f64 {
        std::f64::consts::PI
    }

    pub fn pow(x: f64, p: f64) -> f64 {
        x.powf(p)
    }

    pub fn power(x: f64, p: f64) -> f64 {
        pow(x, p)
    }

    pub fn radians(x: f64) -> f64 {
        x.to_radians()
    }

    pub fn rand() -> f64 {
        rand::random::<f64>()
    }

    pub fn random() -> f64 {
        rand()
    }

    pub fn random_bounded(n: i64) -> PrestoResult<i64> {
        if n <= 0 {
            return Err(PrestoError::Domain("n must be > 0"));
        }
        Ok(rand::thread_rng().gen_range(0..n))
    }

    pub fn secure_rand() -> f64 {
        OsRng.r#gen()
    }

    pub fn secure_random() -> f64 {
        secure_rand()
    }

    pub fn secure_random_range(lower: i64, upper: i64) -> PrestoResult<i64> {
        if lower >= upper {
            return Err(PrestoError::Domain("lower must be < upper"));
        }
        let span = (upper - lower) as u64;
        let value = (OsRng.next_u64() % span) as i64;
        Ok(lower + value)
    }

    pub fn round(x: f64) -> f64 {
        x.round()
    }

    pub fn round_n(x: f64, digits: i32) -> f64 {
        let factor = 10f64.powi(digits);
        (x * factor).round() / factor
    }

    pub fn sign(x: f64) -> f64 {
        if x.is_nan() {
            f64::NAN
        } else if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    pub fn tanh(x: f64) -> f64 {
        x.tanh()
    }

    pub fn truncate(x: f64) -> f64 {
        x.trunc()
    }

    pub fn truncate_n(x: f64, n: i32) -> f64 {
        let factor = 10f64.powi(n);
        (x * factor).trunc() / factor
    }

    pub fn width_bucket_range(x: f64, bound1: f64, bound2: f64, n: i64) -> PrestoResult<i64> {
        if n <= 0 {
            return Err(PrestoError::Domain("n must be > 0"));
        }
        if bound1 == bound2 {
            return Err(PrestoError::Domain("bounds must be different"));
        }

        let (low, high, ascending) = if bound1 < bound2 {
            (bound1, bound2, true)
        } else {
            (bound2, bound1, false)
        };

        let out = if x < low {
            0
        } else if x >= high {
            n + 1
        } else {
            (((x - low) / (high - low) * n as f64).floor() as i64) + 1
        };

        Ok(if ascending { out } else { n + 1 - out })
    }

    pub fn width_bucket_bins(x: f64, bins: &[f64]) -> PrestoResult<i64> {
        if bins.is_empty() {
            return Err(PrestoError::InvalidArgument("bins must not be empty"));
        }
        for pair in bins.windows(2) {
            if pair[0] >= pair[1] {
                return Err(PrestoError::InvalidArgument(
                    "bins must be strictly ascending",
                ));
            }
        }
        let idx = bins.partition_point(|v| *v <= x);
        Ok(idx as i64)
    }

    pub fn wilson_interval_lower(successes: f64, trials: f64, z: f64) -> PrestoResult<f64> {
        wilson_interval(successes, trials, z).map(|(lower, _)| lower)
    }

    pub fn wilson_interval_upper(successes: f64, trials: f64, z: f64) -> PrestoResult<f64> {
        wilson_interval(successes, trials, z).map(|(_, upper)| upper)
    }

    fn wilson_interval(successes: f64, trials: f64, z: f64) -> PrestoResult<(f64, f64)> {
        if trials <= 0.0 {
            return Err(PrestoError::Domain("trials must be > 0"));
        }
        if successes < 0.0 || successes > trials {
            return Err(PrestoError::Domain("successes must be in [0,trials]"));
        }
        let p = successes / trials;
        let z2 = z * z;
        let denom = 1.0 + z2 / trials;
        let center = (p + z2 / (2.0 * trials)) / denom;
        let margin = (z / denom) * ((p * (1.0 - p) / trials + z2 / (4.0 * trials * trials)).sqrt());
        Ok((center - margin, center + margin))
    }

    pub fn normal_cdf(mean: f64, sd: f64, value: f64) -> PrestoResult<f64> {
        let d = Normal::new(mean, checked_positive("sd must be > 0", sd)?)
            .map_err(|_| PrestoError::Domain("invalid normal params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_normal_cdf(mean: f64, sd: f64, p: f64) -> PrestoResult<f64> {
        let d = Normal::new(mean, checked_positive("sd must be > 0", sd)?)
            .map_err(|_| PrestoError::Domain("invalid normal params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn beta_cdf(a: f64, b: f64, value: f64) -> PrestoResult<f64> {
        let d = Beta::new(
            checked_positive("a must be > 0", a)?,
            checked_positive("b must be > 0", b)?,
        )
        .map_err(|_| PrestoError::Domain("invalid beta params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_beta_cdf(a: f64, b: f64, p: f64) -> PrestoResult<f64> {
        let d = Beta::new(
            checked_positive("a must be > 0", a)?,
            checked_positive("b must be > 0", b)?,
        )
        .map_err(|_| PrestoError::Domain("invalid beta params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn binomial_cdf(
        number_of_trials: i64,
        success_probability: f64,
        value: i64,
    ) -> PrestoResult<f64> {
        if number_of_trials < 0 || value < 0 {
            return Err(PrestoError::Domain("trials/value must be >= 0"));
        }
        let d = Binomial::new(checked_prob(success_probability)?, number_of_trials as u64)
            .map_err(|_| PrestoError::Domain("invalid binomial params"))?;
        Ok(d.cdf(value as u64))
    }

    pub fn inverse_binomial_cdf(
        number_of_trials: i64,
        success_probability: f64,
        p: f64,
    ) -> PrestoResult<i64> {
        if number_of_trials < 0 {
            return Err(PrestoError::Domain("trials must be >= 0"));
        }
        let d = Binomial::new(checked_prob(success_probability)?, number_of_trials as u64)
            .map_err(|_| PrestoError::Domain("invalid binomial params"))?;
        let p = checked_prob(p)?;
        let mut lo = 0u64;
        let mut hi = number_of_trials as u64;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if d.cdf(mid) >= p {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        Ok(lo as i64)
    }

    pub fn cauchy_cdf(median: f64, scale: f64, value: f64) -> PrestoResult<f64> {
        let d = Cauchy::new(median, checked_positive("scale must be > 0", scale)?)
            .map_err(|_| PrestoError::Domain("invalid cauchy params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_cauchy_cdf(median: f64, scale: f64, p: f64) -> PrestoResult<f64> {
        let d = Cauchy::new(median, checked_positive("scale must be > 0", scale)?)
            .map_err(|_| PrestoError::Domain("invalid cauchy params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn chi_squared_cdf(df: f64, value: f64) -> PrestoResult<f64> {
        let d = ChiSquared::new(checked_positive("df must be > 0", df)?)
            .map_err(|_| PrestoError::Domain("invalid chi-square params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_chi_squared_cdf(df: f64, p: f64) -> PrestoResult<f64> {
        let d = ChiSquared::new(checked_positive("df must be > 0", df)?)
            .map_err(|_| PrestoError::Domain("invalid chi-square params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn f_cdf(df1: f64, df2: f64, value: f64) -> PrestoResult<f64> {
        let d = FisherSnedecor::new(
            checked_positive("df1 must be > 0", df1)?,
            checked_positive("df2 must be > 0", df2)?,
        )
        .map_err(|_| PrestoError::Domain("invalid F params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_f_cdf(df1: f64, df2: f64, p: f64) -> PrestoResult<f64> {
        let d = FisherSnedecor::new(
            checked_positive("df1 must be > 0", df1)?,
            checked_positive("df2 must be > 0", df2)?,
        )
        .map_err(|_| PrestoError::Domain("invalid F params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn gamma_cdf(shape: f64, scale: f64, value: f64) -> PrestoResult<f64> {
        let d = Gamma::new(
            checked_positive("shape must be > 0", shape)?,
            checked_positive("scale must be > 0", scale)?,
        )
        .map_err(|_| PrestoError::Domain("invalid gamma params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_gamma_cdf(shape: f64, scale: f64, p: f64) -> PrestoResult<f64> {
        let d = Gamma::new(
            checked_positive("shape must be > 0", shape)?,
            checked_positive("scale must be > 0", scale)?,
        )
        .map_err(|_| PrestoError::Domain("invalid gamma params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn poisson_cdf(lambda: f64, value: i64) -> PrestoResult<f64> {
        if value < 0 {
            return Err(PrestoError::Domain("value must be >= 0"));
        }
        let d = Poisson::new(checked_positive("lambda must be > 0", lambda)?)
            .map_err(|_| PrestoError::Domain("invalid poisson params"))?;
        Ok(d.cdf(value as u64))
    }

    pub fn inverse_poisson_cdf(lambda: f64, p: f64) -> PrestoResult<i64> {
        let d = Poisson::new(checked_positive("lambda must be > 0", lambda)?)
            .map_err(|_| PrestoError::Domain("invalid poisson params"))?;
        let p = checked_prob(p)?;
        let mut lo = 0u64;
        let mut hi = (lambda.max(1.0) * 10.0).ceil() as u64;
        while d.cdf(hi) < p {
            hi = hi.saturating_mul(2).max(1);
            if hi > 10_000_000 {
                return Err(PrestoError::Overflow("inverse_poisson_cdf search overflow"));
            }
        }
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if d.cdf(mid) >= p {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        Ok(lo as i64)
    }

    pub fn t_cdf(df: f64, value: f64) -> PrestoResult<f64> {
        let d = StudentsT::new(0.0, 1.0, checked_positive("df must be > 0", df)?)
            .map_err(|_| PrestoError::Domain("invalid t params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_t_cdf(df: f64, p: f64) -> PrestoResult<f64> {
        let d = StudentsT::new(0.0, 1.0, checked_positive("df must be > 0", df)?)
            .map_err(|_| PrestoError::Domain("invalid t params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }

    pub fn weibull_cdf(a: f64, b: f64, value: f64) -> PrestoResult<f64> {
        let d = Weibull::new(
            checked_positive("a must be > 0", a)?,
            checked_positive("b must be > 0", b)?,
        )
        .map_err(|_| PrestoError::Domain("invalid weibull params"))?;
        Ok(d.cdf(value))
    }

    pub fn inverse_weibull_cdf(a: f64, b: f64, p: f64) -> PrestoResult<f64> {
        let d = Weibull::new(
            checked_positive("a must be > 0", a)?,
            checked_positive("b must be > 0", b)?,
        )
        .map_err(|_| PrestoError::Domain("invalid weibull params"))?;
        Ok(d.inverse_cdf(checked_prob(p)?))
    }
}

pub fn register(registry: &mut FunctionRegistry) -> PrestoResult<()> {
    let scalar_functions = [
        "abs",
        "acos",
        "asin",
        "atan",
        "atan2",
        "beta_cdf",
        "binomial_cdf",
        "cauchy_cdf",
        "cbrt",
        "ceil",
        "ceiling",
        "chi_squared_cdf",
        "cos",
        "cosh",
        "cosine_similarity",
        "degrees",
        "dot_product",
        "e",
        "exp",
        "f_cdf",
        "factorial",
        "floor",
        "from_base",
        "gamma_cdf",
        "infinity",
        "inverse_beta_cdf",
        "inverse_binomial_cdf",
        "inverse_cauchy_cdf",
        "inverse_chi_squared_cdf",
        "inverse_f_cdf",
        "inverse_gamma_cdf",
        "inverse_laplace_cdf",
        "inverse_normal_cdf",
        "inverse_poisson_cdf",
        "inverse_t_cdf",
        "inverse_weibull_cdf",
        "is_finite",
        "is_infinite",
        "is_nan",
        "l2_squared",
        "laplace_cdf",
        "ln",
        "log10",
        "log2",
        "mod",
        "nan",
        "normal_cdf",
        "pi",
        "poisson_cdf",
        "pow",
        "power",
        "radians",
        "rand",
        "random",
        "round",
        "secure_rand",
        "secure_random",
        "sign",
        "sin",
        "sqrt",
        "t_cdf",
        "tan",
        "tanh",
        "to_base",
        "truncate",
        "weibull_cdf",
        "width_bucket",
        "wilson_interval_lower",
        "wilson_interval_upper",
        "bit_count",
        "bitwise_and",
        "bitwise_arithmetic_shift_right",
        "bitwise_left_shift",
        "bitwise_logical_shift_right",
        "bitwise_not",
        "bitwise_or",
        "bitwise_right_shift",
        "bitwise_right_shift_arithmetic",
        "bitwise_shift_left",
        "bitwise_xor",
    ];

    for name in scalar_functions {
        registry.register_scalar(name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{bitwise, math};

    #[test]
    fn abs_and_factorial_work() {
        assert_eq!(math::abs_f64(-3.5), 3.5);
        assert_eq!(math::abs_i64(-12).unwrap(), 12);
        assert_eq!(math::factorial(5).unwrap(), 120);
    }

    #[test]
    fn bitwise_ops_work() {
        assert_eq!(bitwise::bitwise_and(0b1100, 0b1010), 0b1000);
        assert_eq!(bitwise::bitwise_or(0b1100, 0b1010), 0b1110);
        assert_eq!(bitwise::bitwise_xor(0b1100, 0b1010), 0b0110);
        assert_eq!(bitwise::bit_count(0b1111_0000, 8).unwrap(), 4);
    }

    #[test]
    fn cdf_functions_work_for_basic_inputs() {
        let p = math::normal_cdf(0.0, 1.0, 0.0).unwrap();
        assert!((p - 0.5).abs() < 1e-12);

        let q = math::inverse_normal_cdf(0.0, 1.0, 0.5).unwrap();
        assert!(q.abs() < 1e-12);
    }

    #[test]
    fn width_bucket_basic_behavior() {
        assert_eq!(math::width_bucket_range(0.1, 0.0, 1.0, 10).unwrap(), 2);
        assert_eq!(
            math::width_bucket_bins(12.0, &[10.0, 20.0, 30.0]).unwrap(),
            1
        );
    }
}
