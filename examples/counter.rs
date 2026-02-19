use noxkit::prelude::*;

fn main() {
    let state = create_signal(0);

    let my_view = view! {
        Column {
            Text("Counter:"),
            Text(state.get().to_string()),
            Button("Increment", move || {
                state.update(|n| *n += 1);
            }),
            RoundedRect([0.0, 0.5, 1.0, 1.0], 20.0),
            Circle([1.0, 0.8, 0.0, 1.0])
        }
    };

    let app = App::new(Box::new(my_view));
    app.run();
}
