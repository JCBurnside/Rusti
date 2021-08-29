
use std::marker::PhantomData;

use super::Application;

pub trait BuilderState{}

struct NeedsMainWindow;
impl BuilderState for NeedsMainWindow {}

struct NeedsRenderer;
impl BuilderState for NeedsRenderer {}

pub struct ApplicationBuilder<T:Application, State : BuilderState> {
    app : T,
    _state : std::marker::PhantomData<State>
}

impl <T:Application + Default> ApplicationBuilder<T,NeedsMainWindow> {
    pub fn configure() -> Self {
        Self {
            app : Default::default(),
            _state : PhantomData
        }
    }
}

impl <T:Application> ApplicationBuilder<T,NeedsMainWindow> {
    pub fn configure_with(app:T) -> Self {
        Self {
            app,
            _state : PhantomData
        }
    }
}
