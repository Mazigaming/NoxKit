use noxkit::prelude::*;

fn main() {
    let app_view = view! {
        Column {
            Rect([1.0, 0.0, 0.0, 1.0]), // Red
            Rect([0.0, 1.0, 0.0, 1.0]), // Green
            Rect([0.0, 0.0, 1.0, 1.0])  // Blue
        }
    };

    let app = App::new(Box::new(app_view));
    app.run();
}
