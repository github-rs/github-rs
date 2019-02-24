#![allow(deprecated)] // cause()
error_chain! {
    foreign_links {
        CellBorrowMut(::std::cell::BorrowMutError);
        Hyper(::hyper::Error);
        Http(::hyper::http::Error);
        HeaderValue(::hyper::header::InvalidHeaderValue);
        Uri(::hyper::http::uri::InvalidUriParts);
        HyperTls(::native_tls::Error) #[cfg(feature = "rust-native-tls")];
        Io(::std::io::Error);
        SerdeJson(::serde_json::Error);
    }
}
