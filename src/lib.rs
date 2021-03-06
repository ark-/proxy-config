extern crate url;

use std::fmt;

use url::Url;

#[cfg(windows)]
mod windows;

mod plat {
    #[cfg(windows)]
    pub use windows::get_proxy_strings;

    #[allow(unused_imports)]
    use super::*;

    #[cfg(not(windows))]
    pub fn get_proxy_strings() -> Result<Vec<String>, ProxyConfigError> {
        Err(PlatformNotSupportedError)
    }
}

/// Returns a vector of URLs for the proxies configured by the system
pub fn get_proxies() -> Result<Vec<Url>, ProxyConfigError> {
    match plat::get_proxy_strings() {
        Ok(strings) => {
            let mut result = vec![];
            for string in strings {
                if let Ok(url) = Url::parse(&string) {
                    result.push(url);
                } else {
                    return Err(InvalidConfigError("unable to parse proxy URL"));
                }
            }
            Ok(result)
        },
        Err(e) => Err(e),
    }
}

/// Returns the proxy to use for the given URL
pub fn get_proxy_for_url(url: Url) -> Result<Url, ProxyConfigError> {
    // TODO: cache get_proxies result?
    match get_proxies() {
        Ok(proxies) => {
            for proxy in proxies {
                if proxy.scheme() == url.scheme() {
                    return Ok(proxy);
                }
            }
            return Err(NoProxyForSchemeError(url.scheme().to_string()));
        },
        Err(e) => Err(e),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProxyConfigError {
    InvalidConfigError(&'static str),
    NoProxyConfiguredError,
    NoProxyForSchemeError(String),
    OsError,
    PlatformNotSupportedError,
    ProxyTypeNotSupportedError(&'static str),
}
use ProxyConfigError::*;

impl fmt::Display for ProxyConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidConfigError(s) => write!(f, "Proxy configuration invalid: {}", s),
            NoProxyConfiguredError => write!(f, "No proxy configuration found"),
            NoProxyForSchemeError(ref s) => write!(f, "No proxy found for scheme: {}", s),
            OsError => 
                write!(f, "Error getting proxy configuration from the Operating System"),
            PlatformNotSupportedError => {
                write!(f, "Can not read proxy configuration on this platform")
            },
            ProxyTypeNotSupportedError(s) => {
                write!(f, "Proxy type not supported: {}", s)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test_get_proxies() {
        let _ = get_proxies();
    }

    #[test]
    fn smoke_test_get_proxy_for_url() {
        let _ = get_proxy_for_url(Url::parse("https://google.com").unwrap());
    }
}
