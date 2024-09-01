use http::uri::Uri;

/// Trait for converting a path to its params.
pub(crate) trait ToParams {
    /// Extracts the available params of a provided path.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path used for extracting the params.
    ///
    /// ### Returns
    ///
    /// * `Option<Vec<(String, String)>>` - The extracted params, if the path matches.
    ///
    /// ### Examples
    ///
    /// ```rust ignore
    /// use hyper::http::uri::Uri;
    ///
    /// let uri = Uri::from_static("/users/123");
    /// let route_path = "/users/<id>";
    ///
    /// let params = uri.to_params(route_path);
    ///
    /// assert_eq!(params.is_some(), true);
    /// assert_eq!(params.unwrap(), vec![("id".to_string(), "123".to_string())]);
    /// ```
    fn to_params(&self, path: &str) -> Option<Vec<(String, String)>>;
}

/// Converts a path string to a regular expression.
fn path_to_regex(path: &str) -> regex::Regex {
    let path_b = format!("^{}$", path);
    let path_b = path_b.replace('/', "\\/");
    let path_b = path_b.replace('*', "(.*)");
    let path_b = path_b.replace('<', "(?P<").replace('>', ">[^/]+)");
    regex::Regex::new(&path_b).unwrap()
}

impl ToParams for Uri {
    fn to_params(&self, route_path: &str) -> Option<Vec<(String, String)>> {
        let uri_path = self.path().trim_start_matches('/').trim_end_matches('/');
        let params_path = route_path.trim_start_matches('/').trim_end_matches('/');

        let path_r = path_to_regex(params_path);

        if !path_r.is_match(uri_path) {
            return None;
        }

        let path_b = params_path.split('/').collect::<Vec<&str>>();
        let path_a = uri_path.split('/').collect::<Vec<&str>>();

        let mut named_matches_with_values = Vec::new();

        path_b.iter().for_each(|part| {
            if part.contains('<') {
                let path_r = path_to_regex(part);
                let name = path_r.capture_names().nth(1).unwrap().unwrap();
                let path_b_pos = path_b.iter().position(|&x| &x == part).unwrap();
                let value = {
                    if path_a.len() > path_b.len() {
                        path_a[path_b_pos..].join("/")
                    } else {
                        path_a[path_b_pos].to_string()
                    }
                };
                named_matches_with_values.push((name.to_string(), value));
            }
        });

        Some(named_matches_with_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_params() {
        let uri = Uri::from_static("/users/123");
        let route_path = "/users/<id>";

        let params = uri.to_params(route_path);

        assert_eq!(params.is_some(), true);
        assert_eq!(params.unwrap(), vec![("id".to_string(), "123".to_string())]);
    }

    #[test]
    fn test_to_params_no_match() {
        let uri = Uri::from_static("/posts/456");
        let route_path = "/users/<id>";

        let params = uri.to_params(route_path);

        assert_eq!(params.is_none(), true);
    }

    #[test]
    fn test_to_params_multiple_params() {
        let uri = Uri::from_static("/users/123/posts/456");
        let route_path = "/users/<user_id>/posts/<post_id>";

        let params = uri.to_params(route_path);

        assert_eq!(params.is_some(), true);
        assert_eq!(
            params.unwrap(),
            vec![
                ("user_id".to_string(), "123".to_string()),
                ("post_id".to_string(), "456".to_string())
            ]
        );
    }

    #[test]
    fn test_to_params_wildcard() {
        let uri = Uri::from_static("/users/123/posts");
        let route_path = "/users/<user_id>/*";

        let params = uri.to_params(route_path);

        assert!(params.is_some());
        assert_eq!(
            params.unwrap(),
            vec![("user_id".to_string(), "123".to_string())]
        );
    }

    #[test]
    fn test_to_params_trailing_slash() {
        let uri = Uri::from_static("/users/123/");
        let route_path = "/users/<id>";

        let params = uri.to_params(route_path);

        assert_eq!(params.is_some(), true);
        assert_eq!(params.unwrap(), vec![("id".to_string(), "123".to_string())]);
    }
}
