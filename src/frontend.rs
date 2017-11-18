use errors::ServerError;
use protocol::Service;
use futures::{future, Future};
use serde_json::{from_value, Value};
use structs::{ScrollTo, Style, Update};
use client::Client;

pub type ServerResult<T> = Box<Future<Item = T, Error = ServerError>>;

/// The `Frontend` trait must be implemented by clients. It defines how the
/// client handles notifications and requests coming from `xi-core`.
pub trait Frontend {
    /// handle `"updates"` notifications from `xi-core`
    fn update(&mut self, update: Update) -> ServerResult<()>;
    /// handle `"scroll_to"` notifications from `xi-core`
    fn scroll_to(&mut self, scroll_to: ScrollTo) -> ServerResult<()>;
    /// handle `"def_style"` notifications from `xi-core`
    fn def_style(&mut self, style: Style) -> ServerResult<()>;
}

/// A builder for the type `F` that implement the `Frontend` trait.
pub trait FrontendBuilder<F>
where
    F: Frontend,
{
    fn build(self, client: Client) -> F;
}

impl<F: Frontend> Service for F {
    type T = Value;
    type E = Value;
    type Error = ServerError;

    fn handle_request(
        &mut self,
        method: &str,
        params: Value,
    ) -> Box<Future<Item = Result<Self::T, Self::E>, Error = Self::Error>> {
        // AFAIK the core does not send any request to frontends yet
        // We should return an ServerError here
        info!("<<< request: method={}, params={}", method, &params);
        unimplemented!();
    }

    fn handle_notification(
        &mut self,
        method: &str,
        params: Value,
    ) -> Box<Future<Item = (), Error = Self::Error>> {
        info!("<<< notification: method={}, params={}", method, &params);
        match method {
            "update" => {
                match from_value::<Update>(params) {
                    Ok(update) => self.update(update),
                    Err(e) => Box::new(
                        future::err(ServerError::DeserializeFailed(e)),
                    ),
                }
            }

            "scroll_to" => {
                match from_value::<ScrollTo>(params) {
                    Ok(scroll_to) => self.scroll_to(scroll_to),
                    Err(e) => Box::new(
                        future::err(ServerError::DeserializeFailed(e)),
                    ),
                }
            }

            "def_style" => {
                match from_value::<Style>(params) {
                    Ok(style) => self.def_style(style),
                    Err(e) => Box::new(
                        future::err(ServerError::DeserializeFailed(e)),
                    ),
                }
            }
            _ => Box::new(
                future::err(ServerError::UnknownMethod(method.into())),
            ),
        }
    }
}
