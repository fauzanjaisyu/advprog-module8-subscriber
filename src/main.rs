use borsh_derive::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};
use std::time;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let ten_millis = time::Duration::from_millis(1000);
        let _now = time::Instant::now();

        std::thread::sleep(ten_millis); 

        println!("In Muhammad Fauzan Jaisyurrahman's Computer [2206814040]. Message received: {:?}", message);
        Ok(())
    }

}

fn main() {
    let listener = CrosstownBus::new_queue_listener("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    _ = listener.listen(
        "user_created".to_owned(),
        UserCreatedHandler{},
        crosstown_bus::QueueProperties {
            auto_delete: false, 
            durable: false, 
            use_dead_letter: true
        }
    );

    loop {
        // This loop will keep the listener active.
    }
}