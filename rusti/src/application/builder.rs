
use std::sync::{Arc, Mutex};
pub use rusti_rendering as rendering;
use super::Application;

pub trait BuilderState{}
struct Ready {
    rendering_engine : Arc<Mutex<dyn rusti_rendering::Renderer>>,
}
impl BuilderState for Ready {}

struct NeedsMainWindow { 
    rendering_engine : Arc<Mutex<dyn rusti_rendering::Renderer>>,
}
impl BuilderState  for NeedsMainWindow {}

struct NeedsRenderer;
impl BuilderState for NeedsRenderer {}

pub struct ApplicationBuilder<T:Application, State : BuilderState> {
    app : T,
    state : State
}

impl <T:Application + Default> ApplicationBuilder<T,NeedsRenderer> {
    pub fn configure() -> Self {
        Self {
            app : Default::default(),
            state : NeedsRenderer{},
        }
    }
}

impl <T:Application> ApplicationBuilder<T,NeedsRenderer> {
    pub fn configure_with(app:T) -> Self {
        Self {
            app,
            state : NeedsRenderer{}
        }
    }

    pub fn with_renderer<TRenderer : 'static + rusti_rendering::Renderer>(self,renderer : TRenderer) -> ApplicationBuilder<T,NeedsMainWindow> {
        ApplicationBuilder::<T,NeedsMainWindow> {
            app : self.app,
            state : NeedsMainWindow {
                rendering_engine : Arc::new(Mutex::new(renderer))
            }
        }
    }

    pub fn with_renderer_defaulted<TRenderer:'static + rusti_rendering::Renderer + Default>(self) -> ApplicationBuilder<T,NeedsMainWindow> {
        ApplicationBuilder::<T,NeedsMainWindow> {
            app : self.app,
            state : NeedsMainWindow {
                rendering_engine : Arc::new(Mutex::new(TRenderer::default()))
            }
        }
    }

    //TODO platform dependent with_default_renderer.  depends on [https://github.com/JCBurnside/Rusti/issues/11](#11)
}

impl <T:Application> ApplicationBuilder<T,NeedsMainWindow> {
    //TODO pub fn with_main_window<TContext:DataContext,TWindow : rusti::xaml::Window<DataContext=TContext>>(self,window : TWindow) -> ApplicationBuilder<T,Ready>;
    //TODO pub async fn start_with_main_window<TContext:DataContext,TWindow : rusti::xaml::Window<DataContext=TContext>>(self,window: TWindow) -> Result<(),Box<dyn Error>>
    //TODO pub async fn start_with_main_window_from_context<TContext : DataConext,TWindow : rusti::xaml::Window<DataConext=TContext>>(self,ctx_provider:impl Fn()->TContext)
}