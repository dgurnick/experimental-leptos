use leptos::{*, ev::{MouseEvent}};

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(default=100)]
    max: i32,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        cx,
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn ComboBox(
    cx: Scope,
    initial_size: i32,
    selected_option: i32,
    on_change: WriteSignal<i32>,
) -> impl IntoView 
{

    let options = (0..initial_size)
        .map(|id| (id, create_signal(cx, id + 1 )))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(cx, options);

    view! {
        cx,
        <select
            on:change=move |_| on_change.update(|n| *n += 1)    
        >
            <For
                each=counters
                
                key=|counter| counter.0
                
                view = move |cx, (id, (count, _))| {
                    view! {
                        cx,
                        <option 
                            value=id

                            selected= move || {
                                if id == selected_option {
                                    true
                                } else {
                                    false
                                }
                            }
                            >
                            {count}
                            ":"
                            {id}
                        </option>
                    }
                }

            />  
        </select>
    }
}


#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let double_count = move || count() * 2;

    view! {
        cx,
        <button
            class:red=move || count() % 2 == 1

            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {count}
        </button>

        <ProgressBar progress=count/>

        <ComboBox selected_option=5 initial_size=100 on_change=set_count/>

        <p>
            "Count: " {count}
        </p>
        <p>
            "Double count: " {double_count}
        </p>
    }
}