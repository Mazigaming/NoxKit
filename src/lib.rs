pub mod view;
pub mod layout;
pub mod render;
pub mod widgets;
pub mod state;
pub mod app;

pub use noxkit_macros::view;
pub use view::View;
pub use widgets::{Column, Text, Button};
pub use state::{create_signal, Signal};
pub use app::App;

pub mod prelude {
    pub use crate::view::View;
    pub use crate::widgets::{Column, Text, Button};
    pub use crate::state::{create_signal, Signal};
    pub use crate::app::App;
    pub use noxkit_macros::view;
}
