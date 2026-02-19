use noxkit::prelude::*;

fn main() {
    let count = create_signal(0);

    let app_view = view! {
        Column {
            Rect([0.2, 0.2, 0.2, 1.0]), // Background
            Button("Click Me", move || {
                count.update(|n| *n += 1);
                println!("Count is now: {}", count.get());
            })
        }
    };

    let app = App::new(Box::new(app_view));
    app.run();
}
