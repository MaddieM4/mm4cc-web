use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use std::str;
use std::path::Path;
use std::path::PathBuf;
use safe_path::scoped_join;
use std::ffi::OsStr;

pub async fn load<P: AsRef<Path>>(root: &str, path: P) -> Option<String> where PathBuf: From<P> {
    for pb in propose_paths(root, path) {
        let found = attempt_load(&pb).await;
        if found.is_some() {
            return found;
        }
    }
    return None;
}

async fn attempt_load(path: &PathBuf) -> Option<String> {
    let mut file = File::open(path).await.ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.ok()?;
    return Some(contents)
}

pub fn propose_paths<P: AsRef<Path>>(root: &str, path: P) -> Vec<PathBuf> where PathBuf: From<P> {
    let pb: PathBuf = path.into();
    vec![
        pb.clone(),
        pb.join("Index.md"),
        pb.with_extension("md"),
    ].iter()
     .filter_map(|p| scoped_join(root, p).ok())
     .filter(|p| match p.extension() {
         Some(ext) => ext == OsStr::new("md"),
         None => false,
     })
     .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::tokio;

    #[tokio::test]
    async fn test1() {
        let loaded = load("./fixtures", "test1.md").await;
        let expected = "Test 1...\n".to_string();
        assert_eq!(loaded.unwrap(), expected);
    }

    #[tokio::test]
    async fn test2() {
        let loaded = load("./fixtures", "test2.md").await;
        let expected = "Test 2!\n".to_string();
        assert_eq!(loaded.unwrap(), expected);
    }

    #[tokio::test]
    async fn not_found() {
        let loaded = load("./fixtures", "not_found.md").await;
        assert!(matches!(loaded, None))
    }

    #[tokio::test]
    async fn configuration() {
        let loaded = load("./pages", "Index.md").await;
        assert!(loaded.unwrap().contains("neofeudal"))
    }

    #[test]
    fn propose_paths_blank() {
        let paths = propose_paths("./pages", "");
        assert_eq!(paths, vec![
            scoped_join("./pages", "Index.md").unwrap(),
        ])
    }

    #[test]
    fn propose_paths_slash() {
        let paths = propose_paths("./pages", "/");
        assert_eq!(paths, vec![
            scoped_join("./pages", "Index.md").unwrap(),
        ])
    }

    #[test]
    fn propose_paths_in_subdir() {
        let paths = propose_paths("./pages", "site/how");
        assert_eq!(paths, vec![
            scoped_join("./pages", "site/how/Index.md").unwrap(),
            scoped_join("./pages", "site/how.md").unwrap(),
        ])
    }

    #[test]
    fn propose_paths_bare_subdir() {
        let paths = propose_paths("./pages", "site");
        assert_eq!(paths, vec![
            scoped_join("./pages", "site/Index.md").unwrap(),
            scoped_join("./pages", "site.md").unwrap(),
        ])
    }

    #[test]
    fn propose_paths_slashed_subdir() {
        let paths = propose_paths("./pages", "site/");
        assert_eq!(paths, vec![
            scoped_join("./pages", "site/Index.md").unwrap(),
            scoped_join("./pages", "site.md").unwrap(),
        ])
    }

}
