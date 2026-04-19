use crate::config::Config;
use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;

#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub error_modals: Channel<ErrorModal>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            error_modals: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.config.clone());
    }
}
