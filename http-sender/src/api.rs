use http::{Request, Response};
use http_body::Body;
use std::future::Future;
use std::pin::Pin;

/// HttpSend is used to send HTTP requests and receive responses.
pub trait HttpSend<RequestBody>
where
    RequestBody: Body + 'static,
{
    type ResponseBody: Body + 'static;
    type Error: std::error::Error;

    fn send(
        &self,
        req: Request<RequestBody>,
    ) -> impl Future<Output = Result<Response<Self::ResponseBody>, Self::Error>> + Send;
}

/// HttpSendDyn is the dynamic version of HttpSend that can generate dynamic Futures.
pub trait HttpSendDyn<RequestBody>
where
    RequestBody: Body + 'static,
{
    type ResponseBody: Body + 'static;
    type Error: std::error::Error;

    #[allow(clippy::type_complexity)]
    fn send_dyn(
        &self,
        req: Request<RequestBody>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<Self::ResponseBody>, Self::Error>> + Send + '_>>;
}

impl<T, RequestBody> HttpSendDyn<RequestBody> for T
where
    RequestBody: Body + 'static,
    T: HttpSend<RequestBody>,
{
    type ResponseBody = T::ResponseBody;
    type Error = T::Error;

    fn send_dyn(
        &self,
        req: Request<RequestBody>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<Self::ResponseBody>, Self::Error>> + Send + '_>>
    {
        Box::pin(HttpSend::send(self, req))
    }
}
