use crate::controllers::Controller;

pub struct RuntimeController {}

impl Controller for RuntimeController {}

impl RuntimeController {
    pub fn new() -> Self {
        Self {}
    }
}
