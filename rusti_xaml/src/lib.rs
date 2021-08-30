
pub struct XamlEntry {
    ty : String,
    childern : Content,
    attributes : Vec<XamlAttribute>
}

enum Content {
    Xaml(Vec<XamlEntry>),
    String(String),
}

pub struct XamlAttribute {
    name : String,
    is_binding : bool,
    value : Option<String>,
    binding : Option<Binding>
}

///Represents the binding mode of a binding.
enum BindingMode {
    /// Represents the mode described by the property.
    /// ex `"{bind foo,mode=Default}"` or `"{bind foo}"`
    Default,
    /// Represents the control will read from the data context exactly once at the start.
    /// ex `"{bind foo,mode=OneTime}"`
    OneTime,
    /// Represents the control will only read but keep updated from the data context.
    /// ex `"{bind foo,mode=ReadOnly}"`
    ReadOnly,
    /// Probably the most common mode. Represents the control will only write to the data context and keep the context updated
    /// ex `"{bind foo,mode=WriteOnly}"`
    WriteOnly,

    /// Both the control and the data context will keep each other updated.
    /// ex `"{bind foo,mode=TwoWay}"`
    TwoWay,
}

struct Binding {
    target : String,
    binding_mode : BindingMode
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
    }
}
