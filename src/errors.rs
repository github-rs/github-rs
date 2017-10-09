error_chain!{
    foreign_links {
        CellBorrowMut(::std::cell::BorrowMutError);
        Hyper(::hyper::Error);
        HyperMimeFromStr(::hyper::mime::FromStrError);
        HyperUri(::hyper::error::UriError);
        Io(::std::io::Error);
        SerdeJson(::serde_json::Error);
    }
}
