#[cfg(test)]
mod tests {
    use crate::*;
    use bytes::Bytes;
    use http::{Method, Request};

    #[test]
    fn test_fluent_api() {
        // Create a conditional rewriter using fluent API
        let rewriter = PathRewriter::new("^/api/(.*)", "/v2/$1").unwrap()
            .when(MethodCondition::new(Method::GET)
                .expect("Method::GET is always valid"));

        // Test with matching request
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/users")
            .body(Bytes::new())
            .unwrap();

        let result = rewriter.rewrite(request).unwrap();
        assert_eq!(result.uri().path(), "/v2/users");

        // Test with non-matching request
        let request = Request::builder()
            .method(Method::POST)
            .uri("/api/users")
            .body(Bytes::new())
            .unwrap();

        let result = rewriter.rewrite(request).unwrap();
        assert_eq!(result.uri().path(), "/api/users");
    }

    #[test]
    fn test_chained_rewriters() {
        // Create a chain of rewriters
        let rewriter = PathRewriter::new("^/old/(.*)", "/new/$1").unwrap()
            .then(MethodRewriter::new(Method::POST))
            .then(|mut request: Request<()>| {
                request.headers_mut().insert("X-Rewritten", "true".parse().unwrap());
                Ok(request)
            });

        let request = Request::builder()
            .method(Method::GET)
            .uri("/old/path")
            .body(Bytes::new())
            .unwrap();

        let result = rewriter.rewrite(request).unwrap();
        assert_eq!(result.uri().path(), "/new/path");
        assert_eq!(result.method(), Method::POST);
        assert_eq!(result.headers().get("x-rewritten").unwrap(), "true");
    }

    #[test]
    fn test_complex_conditions() {
        // Create complex condition: (GET or POST) AND /api/*
        let method_condition = MethodCondition::new(Method::GET)
                .expect("Method::GET is always valid")
            .or(MethodCondition::new(Method::POST)
                .expect("Method::POST is always valid"));
        let path_condition = PathCondition::new("^/api/.*").unwrap();
        let combined = method_condition.and(path_condition);

        // Test matching
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/users")
            .body(Bytes::new())
            .unwrap();
        assert!(combined.matches(&request));

        let request = Request::builder()
            .method(Method::POST)
            .uri("/api/users")
            .body(Bytes::new())
            .unwrap();
        assert!(combined.matches(&request));

        // Test non-matching
        let request = Request::builder()
            .method(Method::DELETE)
            .uri("/api/users")
            .body(Bytes::new())
            .unwrap();
        assert!(!combined.matches(&request));

        let request = Request::builder()
            .method(Method::GET)
            .uri("/home")
            .body(Bytes::new())
            .unwrap();
        assert!(!combined.matches(&request));
    }

    #[test]
    fn test_closure_condition_preserves_body() {
        // Create a closure condition that checks the path
        let is_api_path = |request: &Request<()>| -> bool {
            request.uri().path().starts_with("/api/")
        };

        // Create a rewriter with the closure condition
        let rewriter = PathRewriter::new("^/api/v1/(.*)$", "/api/v2/$1").unwrap()
            .when(is_api_path);

        // Test with a request that has a body
        let original_body = Bytes::from("request body content");
        let request = Request::builder()
            .method(Method::POST)
            .uri("/api/v1/users")
            .body(original_body.clone())
            .unwrap();

        // Verify the condition matches
        assert!(is_api_path.matches(&request));

        // Apply the rewriter
        let result = rewriter.rewrite(request).unwrap();

        // Verify the path was rewritten
        assert_eq!(result.uri().path(), "/api/v2/users");

        // Verify the body was preserved
        assert_eq!(result.body(), &original_body);
        assert_eq!(result.body().as_ref(), b"request body content");
    }

    #[test]
    fn test_existence_conditions_with_document_root() {
        use std::fs;
        use std::env;

        // Create a temporary directory for testing
        let temp_dir = env::temp_dir().join("lang_handler_test");
        let _ = fs::create_dir(&temp_dir);

        // Create a test file
        let test_file = temp_dir.join("exists.txt");
        fs::write(&test_file, "test content").expect("Failed to create test file");

        // Test ExistenceCondition
        let exists_cond = ExistenceCondition::new();

        let mut request = Request::builder()
            .uri("/exists.txt")
            .body(Bytes::new())
            .unwrap();

        // Without document root, should not match
        assert!(!exists_cond.matches(&request));

        // With document root, should match existing file
        request.set_document_root(DocumentRoot::from(temp_dir.as_path()));
        assert!(exists_cond.matches(&request));

        // Test NonExistenceCondition
        let not_exists_cond = NonExistenceCondition::new();

        // Should not match for existing file
        assert!(!not_exists_cond.matches(&request));

        // Should match for non-existing file
        let mut request2 = Request::builder()
            .uri("/does_not_exist.txt")
            .body(Bytes::new())
            .unwrap();
        request2.set_document_root(DocumentRoot::from(temp_dir.as_path()));
        assert!(not_exists_cond.matches(&request2));

        // Cleanup
        let _ = fs::remove_file(test_file);
        let _ = fs::remove_dir(temp_dir);
    }
}
