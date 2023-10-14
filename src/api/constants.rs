/// The maximum number of results to retrieve per listing request.
///
/// This limit only applies to single requests. It may be exceeded across
/// multiple requests using pagination.
pub const LISTING_LIMIT: i64 = 1024;

/// The default number of results to retrieve per page in a listing request.
///
/// This can be overriden on certain endpoints, but the page size cannot exceed
/// `LISTING_LIMIT`.
pub const DEFAULT_PAGE_SIZE: u64 = 100;
