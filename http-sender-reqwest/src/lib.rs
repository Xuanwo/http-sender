use http_sender::HttpSend;

pub struct Sender(reqwest::Client);

impl<RequestBody> HttpSend<RequestBody> for Sender
where
    RequestBody: http_body::Body + 'static,
{
    type ResponseBody = reqwest::Body;
    type Error = reqwest::Error;

    async fn send(
        &self,
        req: http::Request<RequestBody>,
    ) -> Result<http::Response<Self::ResponseBody>, Self::Error> {
        Ok(self.0.execute(req.try_into()?).await?.into())
    }
}
