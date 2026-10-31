use anyhow::{Context, Result, anyhow};
use zylotek_domain::woods::value_objects::Measurement;

pub(crate) fn parse_number(value: &str) -> Result<i64> {
    let value = value.trim();

    if value.is_empty() || value == "-" {
        return Err(anyhow!("missing numeric value"));
    }

    value
        .parse::<i64>()
        .with_context(|| format!("invalid numeric value: {value:?}"))
}

pub(crate) fn optional_number(value: Option<&String>) -> Option<i64> {
    let value = value?.trim();

    if value.is_empty() || value == "-" {
        return None;
    }

    parse_number(value).ok()
}

pub(crate) fn parse_decimal(value: &str) -> Result<f64> {
    let value = value.trim();

    if value.is_empty() || value == "-" {
        return Err(anyhow!("missing numeric value"));
    }

    value
        .replace(',', ".")
        .parse::<f64>()
        .with_context(|| format!("invalid numeric value: {value:?}"))
}

pub(crate) fn optional_decimal(value: Option<&String>) -> Option<f64> {
    let value = value?.trim();

    if value.is_empty() || value == "-" {
        return None;
    }

    parse_decimal(value).ok()
}

/// Parses an optional decimal cell into a domain measurement, rejecting out-of-range values.
pub(crate) fn measurement<M: Measurement>(value: Option<&String>) -> Result<Option<M>> {
    Ok(M::from_optional(optional_decimal(value))?)
}

pub(crate) fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
