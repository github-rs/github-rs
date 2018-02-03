error_chain!{
    foreign_links {
        CellBorrowMut(::std::cell::BorrowMutError);
        Hyper(::hyper::Error);
        HyperMimeFromStr(::hyper::mime::FromStrError);
        HyperUri(::hyper::error::UriError);
        HyperTls(::native_tls::Error) #[cfg(feature = "rust-native-tls")];
        Io(::std::io::Error);
        SerdeJson(::serde_json::Error);
    }
}
