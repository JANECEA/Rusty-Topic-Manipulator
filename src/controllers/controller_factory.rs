use crate::{
    controllers::{
        arg_controller::ArgController, runtime_controller::RuntimeController, Controller,
    },
    models::model::Model,
    views::View,
};

pub trait ControllerFactory {
    fn get_controller(&self, model: Model, view: Box<dyn View>) -> Box<dyn Controller>;
}

pub struct ArgControllerFactory;

impl ControllerFactory for ArgControllerFactory {
    fn get_controller(&self, model: Model, view: Box<dyn View>) -> Box<dyn Controller> {
        Box::new(ArgController::new(model, view))
    }
}

impl ArgControllerFactory {
    pub fn new() -> ArgControllerFactory {
        Self {}
    }
}

pub struct RuntimeControllerFactory;

impl ControllerFactory for RuntimeControllerFactory {
    fn get_controller(&self, model: Model, view: Box<dyn View>) -> Box<dyn Controller> {
        Box::new(RuntimeController::new(model, view))
    }
}

impl RuntimeControllerFactory {
    pub fn new() -> RuntimeControllerFactory {
        Self {}
    }
}
