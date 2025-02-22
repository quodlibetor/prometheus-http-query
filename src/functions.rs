//! A set of PromQL function equivalents e.g. `abs` and `rate`
use crate::error::{Error, InvalidFunctionArgument};
use crate::vector::*;

/// Apply the PromQL `abs` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::abs;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = abs(vector);
///
///     assert_eq!(&result.to_string(), "abs(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn abs(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("abs({})", query);
    InstantVector(new)
}

/// Apply the PromQL `absent` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::absent;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("nonexistent")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = absent(vector);
///
///     assert_eq!(&result.to_string(), "absent(nonexistent{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn absent(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("absent({})", query);
    InstantVector(new)
}

/// Apply the PromQL `absent_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::absent_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("nonexistent")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = absent_over_time(vector);
///
///     assert_eq!(&result.to_string(), "absent_over_time(nonexistent{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn absent_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("absent_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `ceil` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::ceil;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = ceil(vector);
///
///     assert_eq!(&result.to_string(), "ceil(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn ceil(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("ceil({})", query);
    InstantVector(new)
}

/// Apply the PromQL `changes` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::changes;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = changes(vector);
///
///     assert_eq!(&result.to_string(), "changes(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn changes(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("changes({})", query);
    InstantVector(new)
}

/// Apply the PromQL `clamp` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::clamp;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = clamp(vector, 0.5, 0.75);
///
///     assert_eq!(&result.to_string(), "clamp(some_metric{some_label=\"some_value\"}, 0.5, 0.75)");
///
///     Ok(())
/// }
/// ```
pub fn clamp(vector: InstantVector, min: f64, max: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp({}, {}, {})", query, min, max);
    InstantVector(new)
}

/// Apply the PromQL `clamp_max` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::clamp_max;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = clamp_max(vector, 0.75);
///
///     assert_eq!(&result.to_string(), "clamp_max(some_metric{some_label=\"some_value\"}, 0.75)");
///
///     Ok(())
/// }
/// ```
pub fn clamp_max(vector: InstantVector, max: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp_max({}, {})", query, max);
    InstantVector(new)
}

/// Apply the PromQL `clamp_min` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::clamp_min;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = clamp_min(vector, 0.5);
///
///     assert_eq!(&result.to_string(), "clamp_min(some_metric{some_label=\"some_value\"}, 0.5)");
///
///     Ok(())
/// }
/// ```
pub fn clamp_min(vector: InstantVector, min: f64) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("clamp_min({}, {})", query, min);
    InstantVector(new)
}

/// Apply the PromQL `day_of_month` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::day_of_month;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = day_of_month(vector);
///
///     assert_eq!(&result.to_string(), "day_of_month(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn day_of_month(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("day_of_month({})", query);
    InstantVector(new)
}

/// Apply the PromQL `day_of_week` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::day_of_week;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = day_of_week(vector);
///
///     assert_eq!(&result.to_string(), "day_of_week(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn day_of_week(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("day_of_week({})", query);
    InstantVector(new)
}

/// Apply the PromQL `days_in_month` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::days_in_month;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = days_in_month(vector);
///
///     assert_eq!(&result.to_string(), "days_in_month(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn days_in_month(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("days_in_month({})", query);
    InstantVector(new)
}

/// Apply the PromQL `delta` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::delta;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = delta(vector);
///
///     assert_eq!(&result.to_string(), "delta(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn delta(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("delta({})", query);
    InstantVector(new)
}

/// Apply the PromQL `deriv` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::deriv;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = deriv(vector);
///
///     assert_eq!(&result.to_string(), "deriv(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn deriv(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("deriv({})", query);
    InstantVector(new)
}

/// Apply the PromQL `exp` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::exp;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = exp(vector);
///
///     assert_eq!(&result.to_string(), "exp(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn exp(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("exp({})", query);
    InstantVector(new)
}

/// Apply the PromQL `floor` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::floor;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = floor(vector);
///
///     assert_eq!(&result.to_string(), "floor(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn floor(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("floor({})", query);
    InstantVector(new)
}

/// Apply the PromQL `histogram_quantile` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::histogram_quantile;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = histogram_quantile(0.95, vector);
///
///     assert_eq!(&result.to_string(), "histogram_quantile(0.95, some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn histogram_quantile(quantile: f64, vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("histogram_quantile({}, {})", quantile, query);
    InstantVector(new)
}

/// Apply the PromQL `holt_winters` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::holt_winters;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = holt_winters(vector, 0.5, 0.9)?;
///
///     assert_eq!(&result.to_string(), "holt_winters(some_metric{some_label=\"some_value\"}[5m], 0.5, 0.9)");
///
///     Ok(())
/// }
/// ```
pub fn holt_winters(vector: RangeVector, sf: f64, tf: f64) -> Result<InstantVector, Error> {
    if sf <= 0.0 || tf <= 0.0 || sf >= 1.0 || tf >= 1.0 {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from(
                "smoothing factors in holt_winters() must be between 0.0 (excl.) and 1.0 (excl.)",
            ),
        }));
    }

    let RangeVector(query) = vector;
    let new = format!("holt_winters({}, {}, {})", query, sf, tf);
    Ok(InstantVector(new))
}

/// Apply the PromQL `hour` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::hour;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = hour(vector);
///
///     assert_eq!(&result.to_string(), "hour(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn hour(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("hour({})", query);
    InstantVector(new)
}

/// Apply the PromQL `idelta` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::idelta;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = idelta(vector);
///
///     assert_eq!(&result.to_string(), "idelta(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn idelta(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("idelta({})", query);
    InstantVector(new)
}

/// Apply the PromQL `increase` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::increase;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = increase(vector);
///
///     assert_eq!(&result.to_string(), "increase(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn increase(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("increase({})", query);
    InstantVector(new)
}

/// Apply the PromQL `irate` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::irate;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = irate(vector);
///
///     assert_eq!(&result.to_string(), "irate(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn irate(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("irate({})", query);
    InstantVector(new)
}

/// Apply the PromQL `label_join` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::label_join;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("label1", "value1")
///         .with("label2", "value2")
///         .try_into()?;
///
///     let result = label_join(vector, "new_label", ":", &["label1", "label2"])?;
///
///     let promql = r#"label_join(some_metric{label1="value1",label2="value2"}, "new_label", ":", "label1", "label2")"#;
///
///     assert_eq!(&result.to_string(), promql);
///
///     Ok(())
/// }
/// ```
pub fn label_join(
    vector: InstantVector,
    dst_label: &str,
    separator: &str,
    src_labels: &[&str],
) -> Result<InstantVector, Error> {
    if dst_label.is_empty() {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from("destination label name in label_join() cannot be empty"),
        }));
    }

    if src_labels.is_empty() {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from("list of source label names in label_join() cannot be empty"),
        }));
    }

    let InstantVector(query) = vector;

    let src_labels = src_labels
        .iter()
        .map(|l| format!("\"{}\"", l))
        .collect::<Vec<String>>()
        .join(", ");

    let new = format!(
        "label_join({}, \"{}\", \"{}\", {})",
        query, dst_label, separator, src_labels
    );

    Ok(InstantVector(new))
}

/// Apply the PromQL `label_replace` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::label_replace;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = label_replace(vector, "new_label", "$1", "some_label", "(.*):.*")?;
///
///     let promql = r#"label_replace(some_metric{some_label="some_value"}, "new_label", "$1", "some_label", "(.*):.*")"#;
///
///     assert_eq!(&result.to_string(), promql);
///
///     Ok(())
/// }
/// ```
pub fn label_replace(
    vector: InstantVector,
    dst_label: &str,
    replacement: &str,
    src_label: &str,
    regex: &str,
) -> Result<InstantVector, Error> {
    if dst_label.is_empty() {
        return Err(Error::InvalidFunctionArgument(InvalidFunctionArgument {
            message: String::from("destination label name in label_replace() cannot be empty"),
        }));
    }

    let InstantVector(query) = vector;
    let new = format!(
        "label_replace({}, \"{}\", \"{}\", \"{}\", \"{}\")",
        query, dst_label, replacement, src_label, regex
    );
    Ok(InstantVector(new))
}

/// Apply the PromQL `ln` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::ln;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = ln(vector);
///
///     assert_eq!(&result.to_string(), "ln(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn ln(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("ln({})", query);
    InstantVector(new)
}

/// Apply the PromQL `log2` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::log2;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = log2(vector);
///
///     assert_eq!(&result.to_string(), "log2(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn log2(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("log2({})", query);
    InstantVector(new)
}

/// Apply the PromQL `log10` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::log10;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = log10(vector);
///
///     assert_eq!(&result.to_string(), "log10(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn log10(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("log10({})", query);
    InstantVector(new)
}

/// Apply the PromQL `minute` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::minute;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = minute(vector);
///
///     assert_eq!(&result.to_string(), "minute(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn minute(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("minute({})", query);
    InstantVector(new)
}

/// Apply the PromQL `month` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::month;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = month(vector);
///
///     assert_eq!(&result.to_string(), "month(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn month(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("month({})", query);
    InstantVector(new)
}

/// Apply the PromQL `predict_linear` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::predict_linear;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = predict_linear(vector, 300.0);
///
///     assert_eq!(&result.to_string(), "predict_linear(some_metric{some_label=\"some_value\"}[5m], 300)");
///
///     Ok(())
/// }
/// ```
pub fn predict_linear(vector: RangeVector, seconds: f64) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("predict_linear({}, {})", query, seconds);
    InstantVector(new)
}

/// Apply the PromQL `rate` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::rate;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = rate(vector);
///
///     assert_eq!(&result.to_string(), "rate(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn rate(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("rate({})", query);
    InstantVector(new)
}

/// Apply the PromQL `resets` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::resets;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = resets(vector);
///
///     assert_eq!(&result.to_string(), "resets(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn resets(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("resets({})", query);
    InstantVector(new)
}

/// Apply the PromQL `round` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::round;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = round(vector, None);
///
///     assert_eq!(&result.to_string(), "round(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn round(vector: InstantVector, to_nearest: Option<f64>) -> InstantVector {
    let InstantVector(query) = vector;
    let new = if let Some(nearest) = to_nearest {
        format!("round({}, {})", query, nearest)
    } else {
        format!("round({})", query)
    };
    InstantVector(new)
}

/// Apply the PromQL `scalar` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::scalar;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = scalar(vector);
///
///     assert_eq!(&result.to_string(), "scalar(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn scalar(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("scalar({})", query);
    InstantVector(new)
}

/// Apply the PromQL `sgn` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::sgn;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = sgn(vector);
///
///     assert_eq!(&result.to_string(), "sgn(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn sgn(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("sgn({})", query);
    InstantVector(new)
}

/// Apply the PromQL `sort` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::sort;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = sort(vector);
///
///     assert_eq!(&result.to_string(), "sort(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn sort(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("sort({})", query);
    InstantVector(new)
}

/// Apply the PromQL `sort_desc` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::sort_desc;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = sort_desc(vector);
///
///     assert_eq!(&result.to_string(), "sort_desc(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn sort_desc(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("sort_desc({})", query);
    InstantVector(new)
}

/// Apply the PromQL `timestamp` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::timestamp;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = timestamp(vector);
///
///     assert_eq!(&result.to_string(), "timestamp(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn timestamp(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("timestamp({})", query);
    InstantVector(new)
}

/// Apply the PromQL `year` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, InstantVector};
/// use prometheus_http_query::functions::year;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: InstantVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .try_into()?;
///
///     let result = year(vector);
///
///     assert_eq!(&result.to_string(), "year(some_metric{some_label=\"some_value\"})");
///
///     Ok(())
/// }
/// ```
pub fn year(vector: InstantVector) -> InstantVector {
    let InstantVector(query) = vector;
    let new = format!("year({})", query);
    InstantVector(new)
}

/// Apply the PromQL `avg_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::avg_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = avg_over_time(vector);
///
///     assert_eq!(&result.to_string(), "avg_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn avg_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("avg_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `min_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::min_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = min_over_time(vector);
///
///     assert_eq!(&result.to_string(), "min_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn min_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("min_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `max_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::max_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = max_over_time(vector);
///
///     assert_eq!(&result.to_string(), "max_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn max_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("max_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `sum_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::sum_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = sum_over_time(vector);
///
///     assert_eq!(&result.to_string(), "sum_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn sum_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("sum_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `count_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::count_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = count_over_time(vector);
///
///     assert_eq!(&result.to_string(), "count_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn count_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("count_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `quantile_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::quantile_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = quantile_over_time(0.95, vector);
///
///     assert_eq!(&result.to_string(), "quantile_over_time(0.95, some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn quantile_over_time(quantile: f64, vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("quantile_over_time({}, {})", quantile, query);
    InstantVector(new)
}

/// Apply the PromQL `stddev_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::stddev_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = stddev_over_time(vector);
///
///     assert_eq!(&result.to_string(), "stddev_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn stddev_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("stddev_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `stdvar_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::stdvar_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = stdvar_over_time(vector);
///
///     assert_eq!(&result.to_string(), "stdvar_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn stdvar_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("stdvar_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `last_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::last_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = last_over_time(vector);
///
///     assert_eq!(&result.to_string(), "last_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn last_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("last_over_time({})", query);
    InstantVector(new)
}

/// Apply the PromQL `present_over_time` function.
///
/// ```rust
/// use prometheus_http_query::{Selector, RangeVector};
/// use prometheus_http_query::functions::present_over_time;
/// use std::convert::TryInto;
///
/// fn main() -> Result<(), prometheus_http_query::Error> {
///     let vector: RangeVector = Selector::new()
///         .metric("some_metric")?
///         .with("some_label", "some_value")
///         .range("5m")?
///         .try_into()?;
///
///     let result = present_over_time(vector);
///
///     assert_eq!(&result.to_string(), "present_over_time(some_metric{some_label=\"some_value\"}[5m])");
///
///     Ok(())
/// }
/// ```
pub fn present_over_time(vector: RangeVector) -> InstantVector {
    let RangeVector(query) = vector;
    let new = format!("present_over_time({})", query);
    InstantVector(new)
}
