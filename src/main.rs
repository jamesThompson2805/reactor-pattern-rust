use std::time::Duration;

enum EventType {
    Connect,
    Message,
    Close,
}

struct InitialisationDispatcher;
impl InitialisationDispatcher {
    pub fn register_handler(&self, eh: &dyn EventHandler, et: EventType) -> Result<(),String> {
        todo!();
    }

    pub fn remove_handler(eh: &dyn EventHandler, et: EventType) -> Result<(),String> {
        todo!();
    }

    pub fn handle_events(timeout: &Duration) -> Result<(),String> {
        todo!();
    }
}

pub trait EventHandler {
    fn handle_event(&mut self, et: EventType) -> Result<(),String>;
    fn get_handle(&self) -> &u32;
}
struct ClientConnector<'a> {
    is: &'a InitialisationDispatcher,
    handlers: Vec<ClientHandler<'a>>,
    handle: u32,
}
impl ClientConnector<'_> {
    pub fn new(is: &InitialisationDispatcher, handle: u32) -> Self {
        let self_ = ClientConnector { is, handlers: Vec::new(), handle};
        is.register_handler(&self_, EventType::Connect);
        return self_;
    }
}
impl EventHandler for ClientConnector<'_> {
    fn handle_event(&mut self, et: EventType) -> Result<(),String> {
        if let et = EventType::Connect {
            self.handlers.push(ClientHandler::new(self.is, self.handle));
            return Ok(());    
        }
        Err("Not a connection request".to_string())
    }
    fn get_handle(&self) -> &u32 {
        &self.handle
    }
}

struct ClientHandler<'a> {
    is: &'a InitialisationDispatcher,
    handle: u32
}
impl ClientHandler<'_> {
    pub fn new(is: & InitialisationDispatcher, handle: u32) -> Self {
        let self_ = ClientHandler { is, handle};
        is.register_handler(&self_, EventType::Message);
        return self_;
    }
}
impl EventHandler for ClientHandler<'_> {
    fn handle_event(&mut self, et: EventType) -> Result<(), String> {
        match et {
            EventType::Message => println!("Receiving message on handle {}", self.handle),
            _ => return Err(format!("Received incorrect event type on handle: {}", self.handle)),
        }
        Ok(())
    }

    fn get_handle(&self) -> &u32 {
        &self.handle
    }
}



fn main() {
    println!("Hello, world!");
}
