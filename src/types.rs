use serde::Serialize;

/// Trait for converting request types to URL query parameters.
///
/// This trait is automatically implemented for all types that implement [`Serialize`].
/// It uses [`serde_html_form`] to serialize the struct fields into a query string.
/// Arrays are serialized as repeated keys (`key=val1&key=val2`).
pub trait ToQueryParams: Serialize {
    /// Converts the request to a URL query string.
    ///
    /// Returns an empty string if no parameters are set, otherwise returns
    /// a string starting with `?` followed by URL-encoded key-value pairs.
    fn query_params(&self) -> String {
        let params = serde_html_form::to_string(self).unwrap_or_default();

        if params.is_empty() {
            String::new()
        } else {
            format!("?{params}")
        }
    }
}

impl<T: Serialize> ToQueryParams for T {}
