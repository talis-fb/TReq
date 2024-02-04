use std::str::FromStr;

use anyhow::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlDatas {
    pub protocol: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub paths: Vec<String>,
    pub query_params: Vec<(String, String)>,
    pub anchor: Option<String>,
}

impl ToString for UrlDatas {
    fn to_string(&self) -> String {
        let protocol = self
            .protocol
            .as_ref()
            .map(|p| format!("{}://", p))
            .unwrap_or_default();

        let port = self
            .port
            .as_ref()
            .map(|p| format!(":{}", p))
            .unwrap_or_default();

        let paths = self
            .paths
            .iter()
            .map(|p| format!("/{p}"))
            .collect::<Vec<String>>()
            .join("");

        let query_params = self
            .query_params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<String>>()
            .join("&");

        let query_params = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{query_params}")
        };

        let anchor = self
            .anchor
            .as_ref()
            .map(|a| format!("#{a}"))
            .unwrap_or_default();

        format!(
            "{}{}{}{}{}{}",
            protocol, self.host, port, paths, query_params, anchor
        )
    }
}

impl UrlDatas {
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

    pub fn with_query_params<Str>(mut self, params: impl IntoIterator<Item = (Str, Str)>) -> Self
    where
        Str: Into<String>,
    {
        self.query_params = params
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    pub fn with_anchor(mut self, value: impl Into<String>) -> Self {
        self.anchor = Some(value.into());
        self
    }
}

impl FromStr for UrlDatas {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_overall_url = Regex::new(
            &[
                r"^",
                r"((?<protocol>https?):\/\/)?",
                r"(?<host>[a-zA-Z0-9._@+=-]+)",
                r"(\:(?<port>[0-9]{1,6}))?",
                r"(\/(?<routes>[a-zA-Z0-9._@=+\/-]+))?",
                r"(\/)?", // For accepting optional '/' at end of url
                r"(\?)?", // For accepting optional '?' at end of url
                r"(\?(?<query_params>[a-zA-Z0-9._@=+\&=-]+))?",
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

                UrlDatas {
                    protocol,
                    host: host.unwrap().to_string(),
                    port,
                    paths,
                    query_params,
                    anchor,
                }
            })
            .next()
            .ok_or(Error::msg("No valid url"))?;

        Ok(url)
    }
}

#[cfg(test)]
mod tests_url {
    use super::*;

    #[test]
    fn test_basic_url_from_str_to_struct() -> anyhow::Result<()> {
        let valid_urls = [
            ("google.com", UrlDatas::default().with_host("google.com")),
            ("google.com/", UrlDatas::default().with_host("google.com")),
            ("google.com?", UrlDatas::default().with_host("google.com")),
            (
                "google.com:81",
                UrlDatas::default().with_host("google.com").with_port(81),
            ),
            (
                "google.com:81/",
                UrlDatas::default().with_host("google.com").with_port(81),
            ),
            (
                "google.com/search/advanced",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"]),
            ),
            (
                "google.com/search/advanced/",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"]),
            ),
            (
                "google.com?search=Rust",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_query_params([("search", "Rust")]),
            ),
            (
                "google.com?search=Rust&country=br",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_query_params([("search", "Rust"), ("country", "br")]),
            ),
            (
                "google.com/search/advanced?name=john",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("name", "john")]),
            ),
            (
                "google.com/search/advanced/?name=john",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("name", "john")]),
            ),
            (
                "google.com/search/advanced?name=john&sort=true",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("name", "john"), ("sort", "true")]),
            ),
            (
                "google.com/search/advanced?name=john&sort=true#landing-page",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("name", "john"), ("sort", "true")])
                    .with_anchor("landing-page"),
            ),
            (
                "google.com/search/advanced#landing-page",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_anchor("landing-page"),
            ),
            (
                "google.com/search/advanced/#landing-page",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_anchor("landing-page"),
            ),
            (
                "google.com#landing-page",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_anchor("landing-page"),
            ),
            (
                "google.com/#landing-page",
                UrlDatas::default()
                    .with_host("google.com")
                    .with_anchor("landing-page"),
            ),
        ];

        let variants_with_http = valid_urls.clone().map(|(url, data)| {
            (format!("http://{}", url), data.with_protocol("http"))
        });

        let variants_with_https = valid_urls.clone().map(|(url, data)| {
            (format!("https://{}", url), data.with_protocol("https"))
        });

        let variants_with_www = valid_urls.clone().map(|(url, data)| {
            let original_host = data.host.clone();
            (format!("www.{}", url), data.with_host(format!("www.{original_host}")))
        });

        let valid_urls = valid_urls.into_iter()
            .map(|(url, data)| (url.to_string(), data))
            .chain(variants_with_http)
            .chain(variants_with_https)
            .chain(variants_with_www);

        for (url_str, expected) in valid_urls {
            let url_data = UrlDatas::from_str(url_str.as_str());

            match url_data {
                Ok(url) => assert_eq!(url, expected),
                Err(_) => panic!("Invalid url: {}", url_str),
            }
        }

        Ok(())
    }

    #[test]
    fn test_variants_url_from_str_to_struct() -> anyhow::Result<()> {
        let valid_urls = [
            (
                "example.com/page#section1",
                UrlDatas::default()
                    .with_host("example.com")
                    .with_anchor("section1")
                    .with_paths(["page"]),
            ),
            (
                "example.com:8080",
                UrlDatas::default().with_host("example.com").with_port(8080),
            ),
            (
                "localhost:3000",
                UrlDatas::default().with_host("localhost").with_port(3000),
            ),
            (
                "xn--bcher-kva.ch",
                UrlDatas::default().with_host("xn--bcher-kva.ch"),
            ),
            (
                "subdomain.example.com",
                UrlDatas::default().with_host("subdomain.example.com"),
            ),
            (
                "higher-subdomain.nested-subdomain.lastdomain.example.com",
                UrlDatas::default()
                    .with_host("higher-subdomain.nested-subdomain.lastdomain.example.com"),
            ),
            (
                "example.com//path//to//page",
                UrlDatas::default()
                    .with_host("example.com")
                    .with_paths(["path", "to", "page"]),
            ),
        ];

        for (url_str, expected) in valid_urls {
            let url_data = UrlDatas::from_str(url_str);

            match url_data {
                Ok(url) => assert_eq!(url, expected),
                Err(_) => panic!("Invalid url: {}", url_str),
            }
        }

        Ok(())
    }

    #[test]
    fn test_url_data_to_string() {
        let valid_urls = [
            (UrlDatas::default().with_host("google.com"), "google.com"),
            (UrlDatas::default().with_host("google.com").with_protocol("http"), "http://google.com"),
            (
                UrlDatas::default().with_host("google.com").with_port(81),
                "google.com:81",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"]),
                "google.com/search/advanced",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_query_params([("search", "Rust")]),
                "google.com?search=Rust",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_query_params([("search", "Rust"), ("country", "br")]),
                "google.com?search=Rust&country=br",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_anchor("landing-page"),
                "google.com#landing-page",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_query_params([("search", "Rust"), ("country", "br")]),
                "google.com/search/advanced?search=Rust&country=br",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_port(8080)
                    .with_paths(["search", "advanced"])
                    .with_query_params([("search", "Rust"), ("country", "br")]),
                "google.com:8080/search/advanced?search=Rust&country=br",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_paths(["search", "advanced"])
                    .with_anchor("landing-page"),
                "google.com/search/advanced#landing-page",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_port(8080)
                    .with_paths(["search", "advanced"])
                    .with_query_params([("search", "Rust"), ("country", "br")])
                    .with_anchor("landing-page"),
                "google.com:8080/search/advanced?search=Rust&country=br#landing-page",
            ),
            (
                UrlDatas::default()
                    .with_host("google.com")
                    .with_protocol("https")
                    .with_port(8080)
                    .with_paths(["search", "advanced"])
                    .with_query_params([("search", "Rust"), ("country", "br")])
                    .with_anchor("landing-page"),
                "https://google.com:8080/search/advanced?search=Rust&country=br#landing-page",
            ),
        ];


        for (url_data, url_str_expected) in valid_urls {
            assert_eq!(url_str_expected, url_data.to_string())
        }

    }
}
