use crate::model::{ParseSortFieldError, SortFields};
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
    type Error = ParseSortFieldError;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let sort_fields: SortFields<T> = form_value.as_str().parse()?;

        Ok(SortFieldsRequest(sort_fields))
    }

    fn default() -> Option<Self> {
        Some(SortFieldsRequest(SortFields(vec![])))
    }
}
