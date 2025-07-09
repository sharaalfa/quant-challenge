pub(crate) mod orderbook_analyzer;
pub mod reporting;

pub trait Command {}

pub trait Query {}

pub trait Handler<R> {
    fn handle(&self, request: R);
}

pub struct EmptyCommand;

pub struct EmptyQuery;

pub struct EmptyHandler;

impl Command for EmptyCommand {}

impl Query for EmptyQuery {}

impl Handler<EmptyCommand> for EmptyHandler {
    fn handle(&self, _request: EmptyCommand) {}
}
