// This is free and unencumbered software released into the public domain.

pub fn normalize_url(url: &str) -> String {
    // test whether it's a normal, valid, URL
    if let Ok(url) = <url::Url>::parse(url) {
        return url.to_string();
    };

    // all the below cases treat the url as a file path.

    // replace a `~/` prefix with the path to the user's home dir.
    let url = url
        .strip_prefix("~/")
        .map(|path| {
            std::env::home_dir()
                .expect("unable to determine home directory")
                .join(path)
        })
        .unwrap_or_else(|| std::path::PathBuf::from(url));

    // `std::path::Path::canonicalize`:
    // > Returns the canonical, absolute form of the path with all
    // > intermediate components normalized and symbolic links resolved.
    //
    // This will only work if the file actually exists.
    if let Ok(path) = std::path::Path::new(&url)
        .canonicalize()
        .map_err(|_| ())
        .and_then(url::Url::from_file_path)
    {
        return path.to_string();
    };

    // `std::path::absolute`:
    // > Makes the path absolute without accessing the filesystem.
    if let Ok(path) = std::path::absolute(&url)
        .map_err(|_| ())
        .and_then(url::Url::from_file_path)
    {
        return path.to_string();
    }

    // TODO: add `std::path::Path::normalize_lexically` once it stabilizes.
    // https://github.com/rust-lang/rust/issues/134694
    //
    // if let Ok(path) = std::path::Path::new(url).normalize_lexically() {
    //     return url::Url::from_file_path(path).unwrap().to_string();
    // }

    // one last try, test whether the `url` crate accepts it as path without changes.
    if let Ok(path) = url::Url::from_file_path(std::path::Path::new(&url)) {
        return path.to_string();
    }

    // otherwise just convert to a file URL without changes and hope for the best :)
    // (we should not really get here but just in case.)
    format!("file://{}", url.display())
}
