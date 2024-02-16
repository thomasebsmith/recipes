/// Converts a result containing an error type that is convertible to String
/// into a result with a String error type.
pub fn stringify_err<T, U: ToString>(
    result: Result<T, U>,
) -> Result<T, String> {
    result.map_err(|err| err.to_string())
}
