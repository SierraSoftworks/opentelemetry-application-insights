use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};
#[cfg(any(feature = "reqwest-blocking-client", feature = "reqwest-client"))]
use std::convert::TryInto;
use std::fmt::Debug;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// HTTP client used by the exporter to send telemetry to Application Insights
///
/// This trait can be implemented for different async runtimes, which makes the exporter agnostic
/// to any runtime the user may choose.
#[async_trait]
pub trait HttpClient: Debug + Send + Sync {
    /// Send telemetry to Application Insights
    ///
    /// This may fail if it can't connect to the server or if the request cannot be completed due
    /// to redirects. In those cases the exporter will retry the request.
    async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Bytes>, BoxError>;
}

/// `HttpClient` implementation for `reqwest::Client`
#[cfg(feature = "reqwest-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest-client")))]
#[async_trait]
impl HttpClient for reqwest::Client {
    async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Bytes>, BoxError> {
        let res = self.execute(request.try_into()?).await?;
        Ok(Response::builder()
            .status(res.status())
            .body(res.bytes().await?)?)
    }
}

/// `HttpClient` implementation for `reqwest::blocking::Client`
#[cfg(feature = "reqwest-blocking-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest-blocking-client")))]
#[async_trait]
impl HttpClient for reqwest::blocking::Client {
    async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Bytes>, BoxError> {
        let res = self.execute(request.try_into()?)?;
        Ok(Response::builder()
            .status(res.status())
            .body(res.bytes()?)?)
    }
}

/// `HttpClient` implementation for `surf::Client`
#[cfg(feature = "surf-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "surf-client")))]
#[async_trait]
impl HttpClient for surf::Client {
    async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Bytes>, BoxError> {
        let (parts, body) = request.into_parts();
        let req = surf::post(parts.uri.to_string())
            .content_type("application/json")
            .body(body);
        let mut res = self.send(req).await?;
        Ok(Response::builder()
            .status(res.status() as u16)
            .body(res.body_bytes().await?.into())?)
    }
}
