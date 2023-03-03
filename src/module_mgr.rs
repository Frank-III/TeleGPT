use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::types::BotCommand;

use crate::HandlerResult;

pub trait Module {
    fn register_dependency(&self, dep_map: &mut DependencyMap);

    fn handler_chain(&self)
        -> Handler<'static, DependencyMap, HandlerResult, DpHandlerDescription>;

    fn commands(&self) -> Vec<BotCommand> {
        vec![]
    }
}

pub struct ModuleManager {
    modules: Vec<Box<dyn Module + 'static>>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self { modules: vec![] }
    }

    pub fn register_module<C>(&mut self, module: C)
    where
        C: Module + 'static,
    {
        self.modules.push(Box::new(module));
    }

    pub fn with_all_modules<F>(&self, mut f: F)
    where
        F: FnMut(&dyn Module),
    {
        for module in self.modules.iter() {
            f(module.as_ref());
        }
    }
}