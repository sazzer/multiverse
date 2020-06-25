use crate::model::SortFields;
use rocket::{http::RawStr, request::FromFormValue};
use std::{ops::Deref, str::FromStr};

/// Request details to describe the sort fields of a request
#[derive(Debug)]
pub struct SortFieldsRequest<T>(SortFields<T>);

impl<T> Deref for SortFieldsRequest<T> {
    type Target = SortFields<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'v, T> FromFormValue<'v> for SortFieldsRequest<T>
where
    T: FromStr,
    T: strum::VariantNames,
    T: std::fmt::Debug,
{
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        form_value
            .percent_decode()
            .map_err(|e| {
                tracing::warn!(e = ?e, value = ?form_value, "Failed to decode parameter");
                form_value
            })
            .and_then(|sorts| {
                SortFields::<T>::from_str(&sorts).map_err(|e| {
                    tracing::warn!(e = ?e, value = ?form_value, "Failed to parse sorts");
                    form_value
                })
            })
            .map(|sorts| SortFieldsRequest(sorts))
    }

    fn default() -> Option<Self> {
        Some(SortFieldsRequest(SortFields(vec![])))
    }
}
