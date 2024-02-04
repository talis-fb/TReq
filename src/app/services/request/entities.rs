use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::utils::validators;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum METHODS {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
}

impl FromStr for METHODS {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => METHODS::GET,
            "POST" => METHODS::POST,
            "PUT" => METHODS::PUT,
            "DELETE" => METHODS::DELETE,
            "HEAD" => METHODS::HEAD,
            "PATCH" => METHODS::PATCH,
            _ => return Err(anyhow::Error::msg("No valid METHOD")),
        })
    }
}

impl ToString for METHODS {
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}
impl METHODS {
    pub fn as_str(&self) -> &'static str {
        match self {
            METHODS::GET => "GET",
            METHODS::POST => "POST",
            METHODS::PUT => "PUT",
            METHODS::DELETE => "DELETE",
            METHODS::HEAD => "HEAD",
            METHODS::PATCH => "PATCH",
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    protocol: Option<String>,
    host: String,
    port: Option<u16>,
    paths: Vec<String>,
    query_params: Vec<(String, String)>,
    anchor: Option<String>,
}

impl ToString for Url {
    fn to_string(&self) -> String {
        let protocol = self.protocol.as_ref().map(|p| format!("{}://", p)).unwrap_or_default();

        let port = self.port.as_ref().map(|p| format!(":{}", p)).unwrap_or_default();

        let paths = self
            .paths
            .iter()
            .map(|p| format!("/{}", p))
            .collect::<Vec<String>>()
            .join("");

        let query_params = self
            .query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        let anchor = self
            .anchor
            .as_ref()
            .map(|a| format!("#{}", a))
            .unwrap_or_default();

        format!("{}{}{}{}{}{}", protocol, self.host, port, paths, query_params, anchor)
    }
}

impl Url {
    pub fn with_protocol(mut self, value: impl Into<String>) -> Self {
        self.protocol = Some(value.into());
        self
    }

    pub fn with_host(mut self, value: impl Into<String>) -> Self {
        self.host = value.into();
        self
    }

    pub fn with_port(mut self, value: u16) -> Self {
        self.port = Some(value);
        self
    }

    pub fn with_paths<Str>(mut self, paths: impl IntoIterator<Item = Str>) -> Self
    where
        Str: Into<String>,
    {
        self.paths = paths.into_iter().map(|p| p.into()).collect();
        self
    }

    pub fn with_query_params<Str>(mut self, params: impl IntoIterator<Item = (Str, Str)>) -> Self where Str : Into<String> {
        self.query_params = params.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    pub fn with_anchor(mut self, value: impl Into<String>) -> Self {
        self.anchor = Some(value.into());
        self
    }
}

impl FromStr for Url {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_overall_url = Regex::new(
            &[
                r"^",
                r"((?<protocol>https?):\/\/)?",
                r"(?<host>[a-zA-Z0-9._-]+)",
                r"(\:(?<port>[0-9]{1,6}))?",
                r"(\/(?<routes>[a-zA-Z0-9._\/-]+))?",
                r"(\/)?",
                r"(\?(?<query_params>[a-zA-Z0-9._\&=-]+))?",
                r"(\#(?<anchor>[a-zA-Z0-9._-]+))?",
                r"$",
            ]
            .join("")
            .to_string(),
        )
        .unwrap();

        let re_routes = Regex::new(r"[^\/]+").unwrap();
        let re_query_params = Regex::new(r"[^\&]+=[^&]+").unwrap();

        let url = re_overall_url
            .captures_iter(s)
            .map(|matcher| {
                let protocol = matcher
                    .name("protocol")
                    .map(|m| m.as_str())
                    .map(String::from);

                let host = matcher.name("host").map(|m| m.as_str()).map(String::from);

                let port = matcher
                    .name("port")
                    .map(|m| m.as_str())
                    .map(|port| port.parse::<u16>().unwrap());

                let paths: Vec<String> = matcher
                    .name("routes")
                    .map(|m| m.as_str())
                    .map(|complete_path| {
                        re_routes
                            .find_iter(complete_path)
                            .map(|m| m.as_str())
                            .map(String::from)
                            .collect()
                    })
                    .unwrap_or_default();

                let query_params: Vec<(String, String)> = matcher
                    .name("query_params")
                    .map(|m| m.as_str())
                    .map(|query_params| {
                        re_query_params
                            .find_iter(query_params)
                            .map(|m| m.as_str().split_once("=").unwrap())
                            .map(|(key, value)| (key.to_string(), value.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                let anchor = matcher.name("anchor").map(|m| m.as_str().to_string());

                Url {
                    protocol,
                    host: host.unwrap().to_string(),
                    port,
                    paths,
                    query_params,
                    anchor,
                }
            })
            .next()
            .ok_or(Error::msg("No url found"))?;

        Ok(url)
    }
}

#[cfg(test)]
mod tests_url {
    use super::*;

    #[test]
    fn test_url() {
        let valid_urls = [
            ("google.com", Url::default().with_host("google.com")),
            (
                "http://google.com",
                Url::default().with_host("google.com").with_protocol("http"),
            ),
            (
                "https://google.com",
                Url::default()
                    .with_host("google.com")
                    .with_protocol("https"),
            ),
            (
                "https://google.com/",
                Url::default()
                    .with_host("google.com")
                    .with_protocol("https"),
            ),
            (
                "https://google.com/search/advanced",
                Url::default()
                    .with_host("google.com")
                    .with_protocol("https")
                    .with_paths(["search", "advanced"]),
            ),
            (
                "https://google.com/search/advanced?name=john",
                Url::default()
                    .with_host("google.com")
                    .with_protocol("https")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("name", "john")])
            ),
            (
                "https://google.com/search/advanced?name=john&sort=true",
                Url::default()
                    .with_host("google.com")
                    .with_protocol("https")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("name", "john"), ("sort", "true")])
            ),
            (
                "https://google.com/search/advanced?name=john&sort=true#landing-page",
                Url::default()
                    .with_host("google.com")
                    .with_protocol("https")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("name", "john"), ("sort", "true")])
                    .with_anchor("landing-page"),
            ),
        ];

        for (url, expected) in valid_urls {
            assert_eq!(expected, Url::from_str(url).unwrap());
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestData {
    pub url: String,
    pub method: METHODS,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl RequestData {
    pub fn with_url(mut self, value: impl Into<String>) -> Self {
        let mut value: String = value.into();

        // TODO: Remove it, this is a side effect hidden that will lead to an unexpected behavior
        if !(value.starts_with("http://") || value.starts_with("https://"))
            && validators::is_url(&value)
        {
            value = format!("http://{value}");
        }

        self.url = value;
        self
    }
    pub fn with_body(mut self, value: impl Into<String>) -> Self {
        self.body = value.into();
        self
    }
    pub fn with_method(mut self, value: METHODS) -> Self {
        self.method = value;
        self
    }
    pub fn with_headers(mut self, values: impl Into<HashMap<String, String>>) -> Self {
        self.headers = values.into();
        self
    }
}

// Used to
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OptionalRequestData {
    pub url: Option<String>,
    pub method: Option<METHODS>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl From<RequestData> for OptionalRequestData {
    fn from(value: RequestData) -> Self {
        Self {
            url: Some(value.url),
            method: Some(value.method),
            headers: Some(value.headers),
            body: Some(value.body),
        }
    }
}

impl OptionalRequestData {
    pub fn to_request_data(self) -> RequestData {
        RequestData::default()
            .with_url(self.url.expect("Url is required to define a Request Data"))
            .with_method(
                self.method
                    .expect("METHOD is required to define a Request Data"),
            )
            .with_headers(self.headers.unwrap_or_default())
            .with_body(self.body.unwrap_or_default())
    }

    pub fn merge_with(self, other: RequestData) -> RequestData {
        RequestData::default()
            .with_url(self.url.unwrap_or(other.url))
            .with_method(self.method.unwrap_or(other.method))
            .with_headers(self.headers.unwrap_or(other.headers))
            .with_body(self.body.unwrap_or(other.body))
    }
}

#[derive(Default)]
pub struct RequestEntity {
    current_request: Box<NodeHistoryRequest>,
}

impl RequestEntity {
    pub fn get_current_request(&self) -> Arc<RequestData> {
        self.current_request.data.clone()
    }

    pub fn update_current_request(&mut self, request_data: RequestData) {
        let new_node = Box::from(NodeHistoryRequest::from(request_data));
        let last_state = std::mem::replace(&mut self.current_request, new_node);
        self.current_request.previous = Some(last_state);
    }

    pub fn undo(&mut self) {
        if let Some(previous_req_node) = self.current_request.previous.take() {
            let last_state = std::mem::replace(&mut self.current_request, previous_req_node);
            self.current_request.next = Some(last_state);
        }
    }

    pub fn redo(&mut self) {
        if let Some(next_req_node) = self.current_request.next.take() {
            let last_state = std::mem::replace(&mut self.current_request, next_req_node);
            self.current_request.previous = Some(last_state);
        }
    }
}

impl From<RequestData> for RequestEntity {
    fn from(value: RequestData) -> Self {
        Self {
            current_request: Box::from(NodeHistoryRequest::from(value)),
        }
    }
}

#[derive(Default)]
struct NodeHistoryRequest {
    pub data: Arc<RequestData>,
    pub previous: Option<Box<NodeHistoryRequest>>,
    pub next: Option<Box<NodeHistoryRequest>>,
}
impl From<RequestData> for NodeHistoryRequest {
    fn from(value: RequestData) -> Self {
        let data = Arc::new(value);
        Self {
            data,
            previous: None,
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_with_protocol_default() {
        let start_with_https = RequestData::default().with_url("https://google.com");
        assert_eq!("https://google.com", start_with_https.url);

        let start_with_http = RequestData::default().with_url("http://duck.com");
        assert_eq!("http://duck.com", start_with_http.url);

        let start_without_protocol = RequestData::default().with_url("duck.com");
        assert_eq!("http://duck.com", start_without_protocol.url);
    }

    #[test]
    fn test_url_struct_to_url() {
        let start_with_https = RequestData::default().with_url("https://google.com");
        assert_eq!("https://google.com", start_with_https.url);

        let start_with_http = RequestData::default().with_url("http://duck.com");
        assert_eq!("http://duck.com", start_with_http.url);

        let start_without_protocol = RequestData::default().with_url("duck.com");
        assert_eq!("http://duck.com", start_without_protocol.url);
    }

    // mod url_struct {
    //     use super::*;
    //
    //     // -----------
    //     // FromStr
    //     // -----------
    //     #[test]
    //     fn test_url_struct_from_url_string() {
    //         let basic_url = "http://google.com";
    //
    //         assert_eq!(
    //             Url {
    //                 protocol: Some("http".into()),
    //                 domain: "google.com".to_string(),
    //                 port: None,
    //                 path: "".to_string(),
    //                 query_params: None,
    //                 anchor: None,
    //             },
    //             Url::from_str(basic_url).unwrap()
    //         )
    //     }
    //
    //     #[test]
    //     fn test_url_struct_from_url_string_with_path_and_query_params() {
    //         let basic_url = "http://google.com/api/v1/?name=John";
    //
    //         assert_eq!(
    //             Url {
    //                 protocol: Some("http".into()),
    //                 domain: "google.com".to_string(),
    //                 port: None,
    //                 path: "api/v1/".to_string(),
    //                 query_params: Some(Vec::from([("name".into(), "John".into())])),
    //                 anchor: None,
    //             },
    //             Url::from_str(basic_url).unwrap()
    //         )
    //     }
    //
    //     #[test]
    //     fn test_url_struct_from_url_string_with_query_params_and_anchor() {
    //         // Are the same
    //         let basic_url = "http://google.com/?name=John#Home";
    //         let basic_url2 = "http://google.com?name=John#Home";
    //
    //         let expected = Url {
    //             protocol: Some("http".into()),
    //             domain: "google.com".to_string(),
    //             port: None,
    //             path: "".into(),
    //             query_params: Some(Vec::from([("name".into(), "John".into())])),
    //             anchor: Some("Home".into()),
    //         };
    //
    //         assert_eq!(expected, Url::from_str(basic_url).unwrap());
    //         assert_eq!(expected, Url::from_str(basic_url2).unwrap());
    //     }
    //
    //     // #[test]
    //     // fn test_url_struct_from_url_string_with_port_query_params_and_anchor() {
    //     //     // Are the same
    //     //     let basic_url = "http://google.com:3030/?name=John#Home";
    //     //     let basic_url2 = "http://google.com:3030?name=John#Home";
    //     //
    //     //     let expected = Url {
    //     //         protocol: Some("http".into()),
    //     //         domain: "google.com".to_string(),
    //     //         port: Some(3030),
    //     //         path: "".into(),
    //     //         query_params: Some(Vec::from([("name".into(), "John".into())])),
    //     //         anchor: Some("Home".into()),
    //     //     };
    //     //
    //     //     assert_eq!(expected, Url::from_str(basic_url).unwrap());
    //     //     assert_eq!(expected, Url::from_str(basic_url2).unwrap());
    //     // }
    //
    //     // -----------
    //     // ToString
    //     // -----------
    //     #[test]
    //     fn test_url_struct_to_string() {
    //         let url = Url {
    //             protocol: Some("https".into()),
    //             domain: "google.com".to_string(),
    //             port: None,
    //             path: "".to_string(),
    //             query_params: None,
    //             anchor: None,
    //         };
    //
    //         assert_eq!("https://google.com", url.to_string())
    //     }
    //
    //     #[test]
    //     fn test_url_struct_to_string_with_query_params() {
    //         let url = Url {
    //             protocol: None,
    //             domain: "google.com".to_string(),
    //             port: None,
    //             path: "".to_string(),
    //             query_params: Some(Vec::from([
    //                 ("name".into(), "John".into()),
    //                 ("sort".into(), "true".into()),
    //             ])),
    //             anchor: None,
    //         };
    //
    //         assert_eq!("http://google.com?name=John&sort=true", url.to_string())
    //     }
    //
    //     #[test]
    //     fn test_url_struct_to_string_with_query_params_and_explicit_port() {
    //         let url = Url {
    //             protocol: None,
    //             domain: "google.com".to_string(),
    //             port: Some(3030),
    //             path: "".to_string(),
    //             query_params: Some(Vec::from([
    //                 ("name".into(), "John".into()),
    //                 ("sort".into(), "true".into()),
    //             ])),
    //             anchor: None,
    //         };
    //
    //         assert_eq!(
    //             "http://google.com:3030?name=John&sort=true",
    //             url.to_string()
    //         )
    //     }
    //
    //     #[test]
    //     fn test_url_struct_to_string_with_query_params_and_explicit_port_and_anchor() {
    //         let url = Url {
    //             protocol: None,
    //             domain: "google.com".to_string(),
    //             port: Some(3030),
    //             path: "".to_string(),
    //             query_params: Some(Vec::from([
    //                 ("name".into(), "John".into()),
    //                 ("sort".into(), "true".into()),
    //             ])),
    //             anchor: Some("LandingPage".into()),
    //         };
    //
    //         assert_eq!(
    //             "http://google.com:3030?name=John&sort=true#LandingPage",
    //             url.to_string()
    //         )
    //     }
    // }
}
