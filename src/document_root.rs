//! Document root type for filesystem-based conditions

use std::path::{Path, PathBuf};

/// Document root for filesystem-based conditions
///
/// This type stores the document root path that should be used
/// for checking file existence in conditions like `ExistenceCondition`
/// and `NonExistenceCondition`.
///
/// # Examples
///
/// ```
/// use http_rewriter::DocumentRoot;
/// use std::path::Path;
///
/// // Create from various types
/// let root = DocumentRoot::new("/var/www/html");
/// assert_eq!(root.path(), Path::new("/var/www/html"));
///
/// let root = DocumentRoot::from("/srv/static");
/// assert_eq!(root.path(), Path::new("/srv/static"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentRoot(PathBuf);

impl DocumentRoot {
    /// Create a new document root
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::DocumentRoot;
    ///
    /// let root = DocumentRoot::new("/var/www/html");
    /// ```
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(path.as_ref().to_path_buf())
    }

    /// Get the path
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::DocumentRoot;
    /// use std::path::Path;
    ///
    /// let root = DocumentRoot::new("/var/www/html");
    /// assert_eq!(root.path(), Path::new("/var/www/html"));
    /// ```
    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl From<PathBuf> for DocumentRoot {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl From<&Path> for DocumentRoot {
    fn from(path: &Path) -> Self {
        Self(path.to_path_buf())
    }
}

impl From<&str> for DocumentRoot {
    fn from(s: &str) -> Self {
        Self(PathBuf::from(s))
    }
}

/// Extension trait for storing and retrieving document root from requests
pub trait DocumentRootExt {
    /// Get document root from request extensions
    fn document_root(&self) -> Option<&DocumentRoot>;

    /// Set document root in request extensions
    fn set_document_root(&mut self, root: DocumentRoot);
}

impl<T> DocumentRootExt for http::Request<T> {
    fn document_root(&self) -> Option<&DocumentRoot> {
        self.extensions().get::<DocumentRoot>()
    }

    fn set_document_root(&mut self, root: DocumentRoot) {
        self.extensions_mut().insert(root);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_root() {
        // Test creation from different types
        let root1 = DocumentRoot::new("/var/www/html");
        assert_eq!(root1.path(), Path::new("/var/www/html"));

        let root2 = DocumentRoot::from(PathBuf::from("/srv/static"));
        assert_eq!(root2.path(), Path::new("/srv/static"));

        let root3 = DocumentRoot::from(Path::new("/home/user/public"));
        assert_eq!(root3.path(), Path::new("/home/user/public"));

        let root4 = DocumentRoot::from("/opt/app/static");
        assert_eq!(root4.path(), Path::new("/opt/app/static"));
    }

    #[test]
    fn test_request_extension() {
        let mut request = http::Request::builder()
            .uri("/test")
            .body(())
            .unwrap();

        assert!(request.document_root().is_none());
        request.set_document_root(DocumentRoot::new("/var/www"));
        assert_eq!(request.document_root().unwrap().path(), Path::new("/var/www"));
    }
}
