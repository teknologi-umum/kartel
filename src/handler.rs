pub struct CommandHandler {
    model: Model,
}

impl CommandHandler {
    pub fn new(model: Model) -> Self {
        CommandHandler { model }
    }
}
