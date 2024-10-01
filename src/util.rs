use rust_embed::Embed;
use std::path::PathBuf;

pub fn embedded_names<R: Embed>() -> Vec<String> {
    let mut names = R::iter()
        .flat_map(|s| {
            PathBuf::from(s.to_string())
                .file_stem()
                .map(ToOwned::to_owned)
        })
        .flat_map(|os| os.into_string())
        .map(|s| (s.to_lowercase(), s))
        .collect::<Vec<_>>();

    names.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));

    names.into_iter().map(|(_, s)| s).collect()
}