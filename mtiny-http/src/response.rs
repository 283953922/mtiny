use http::{HeaderMap, HeaderName, HeaderValue, Result, StatusCode, Version};

use crate::extensions::Extensions;

#[derive(Default)]
pub struct Head {
    pub headers: HeaderMap<HeaderValue>,
    pub version: Version,
    pub status: StatusCode,
    pub extensions: Extensions,
    _private: (),
}

impl Head {
    fn new() -> Self {
        Head::default()
    }
}

impl std::fmt::Debug for Head {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Head")
            .field("headers", &self.headers)
            .field("status", &self.status)
            .field("version", &self.version)
            .finish()
    }
}

pub struct Response<T> {
    head: Head,
    body: T,
}

impl Response<()> {
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl<B> Response<B> {
    #[inline]
    pub fn new(body: B) -> Response<B> {
        Self {
            head: Head::new(),
            body,
        }
    }
    #[inline]
    pub fn from_head(head: Head, body: B) -> Response<B> {
        Self { head, body }
    }

    #[inline]
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.head.headers
    }

    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.head.headers
    }
    #[inline]
    pub fn status(&self) -> &StatusCode {
        &self.head.status
    }

    #[inline]
    pub fn status_mut(&mut self) -> &mut StatusCode {
        &mut self.head.status
    }

    #[inline]
    pub fn version(&self) -> &Version {
        &self.head.version
    }

    #[inline]
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.head.version
    }

    #[inline]
    pub fn extensions(&self) -> &Extensions {
        &self.head.extensions
    }

    #[inline]
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.head.extensions
    }

    #[inline]
    pub fn body(&self) -> &B {
        &self.body
    }

    #[inline]
    pub fn body_mut(&mut self) -> &mut B {
        &mut self.body
    }

    #[inline]
    pub fn into_body(self) -> B {
        self.body
    }

    #[inline]
    pub fn into_head(self) -> (Head, B) {
        (self.head, self.body)
    }

    #[inline]
    pub fn map<F, U>(self, f: F) -> Response<U>
    where
        F: FnOnce(B) -> U,
    {
        Response {
            head: self.head,
            body: f(self.body),
        }
    }
}

impl<B: Default> Default for Response<B> {
    fn default() -> Response<B> {
        Response::new(B::default())
    }
}

impl<B: std::fmt::Debug> std::fmt::Debug for Response<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("headers", self.headers())
            .field("status", self.status())
            .field("version", self.version())
            .field("body", self.body())
            .finish()
    }
}
pub struct Builder {
    inner: Result<Head>,
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Self {
            inner: Ok(Head::new()),
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }

    pub fn header<K, V>(self, key: K, value: V) -> Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<crate::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let name = <HeaderName as TryFrom<K>>::try_from(key).map_err(Into::into)?;
            let value = <HeaderValue as TryFrom<V>>::try_from(value).map_err(Into::into)?;
            head.headers.append(name, value);
            Ok(head)
        })
    }

    pub fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {
        self.inner.as_ref().ok().map(|head| &head.headers)
    }
    pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
        self.inner.as_mut().ok().map(|head| &mut head.headers)
    }

    pub fn status<T>(self, status: T) -> Builder
    where
        StatusCode: TryFrom<T>,
        <StatusCode as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let status = TryFrom::try_from(status).map_err(Into::into)?;
            head.status = status;
            Ok(head)
        })
    }

    pub fn status_ref(&self) -> Option<&StatusCode> {
        self.inner.as_ref().ok().map(|head| &head.status)
    }

    pub fn version_ref(&self) -> Option<&Version> {
        self.inner.as_ref().ok().map(|head| &head.version)
    }

    pub fn extensions<T: 'static>(self, extension: T) -> Builder {
        self.and_then(move |mut head| {
            head.extensions.insert(extension);
            Ok(head)
        })
    }
    pub fn extensions_ref(&self) -> Option<&Extensions> {
        self.inner.as_ref().ok().map(|head| &head.extensions)
    }
    pub fn extensions_mut(&mut self) -> Option<&mut Extensions> {
        self.inner.as_mut().ok().map(|head| &mut head.extensions)
    }

    pub fn body<T>(self, body: T) -> Result<Response<T>> {
        self.inner.map(move |head| Response { head, body })
    }

    fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Head) -> Result<Head>,
    {
        Builder {
            inner: self.inner.and_then(f),
        }
    }
}
