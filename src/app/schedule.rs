use leptos::*;

#[component]
pub fn InputCellRenderer<F>(
    #[prop(into)] class: MaybeSignal<String>,
    #[prop(into)] value: MaybeSignal<String>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    view! {
        <td class=class>
            <input type="text" value=value on:change=move |evt| { on_change(event_target_value(&evt)); } />
        </td>
    }
}


