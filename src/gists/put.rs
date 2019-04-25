//! Access the Gists portion of the GitHub API
imports!();
use crate::client::PutQueryBuilder;

new_type!(
    Gists
    GistsId
    GistsIdStar
);

from!(
    @PutQueryBuilder
        -> Gists = "gists"
    @Gists
        => GistsId
    @GistsId
        -> GistsIdStar = "star"
);

impl_macro!(
    @Gists
        |
        |=> id -> GistsId = id_str
    @GistsId
        |=> star -> GistsIdStar
        |
);

exec!(GistsIdStar);
