//! This crate is meant to be used to crate custom rendering engines for rusti.

pub trait Renderer {
}


#[cfg(test)]
mod tests {
    use crate::Renderer;

    fn makes_sure_renderer_is_trait_obj() {
        let _ : Box<dyn Renderer>;
    }
}
