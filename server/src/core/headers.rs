/// The dart pub client will always include an Accept header specifying the API version requested, as follows:
///
/// Accept: application/vnd.pub.v2+json
/// To ensure forward compatibility all API requests should include an Accept header which specifies the version of the API being used. This allows future versions of the API to change responses.
///
/// Clients are strongly encouraged to specify an Accept header. But for compatibility we will probably want to assume API version 2, if no Accept header is specified.
const HEADER_ACCEPT_VALUE_API_V2: &'static str = "application/vnd.pub.v2+json";
