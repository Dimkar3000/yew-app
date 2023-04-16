use yew::prelude::*;

#[function_component]
pub fn Index() -> Html {
    let counter = use_state(|| 0);
    let on_click_increase = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let on_click_decrease = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };
    return html! {
        <div class="prose lg:prose-xl">
            <h1>{"H1"}</h1>
            <h2>{"H2"}</h2>
            <h3>{"H3"}</h3>
            <h4>{"H4"}</h4>
            <h5>{"H5"}</h5>
            <h6>{"H6"}</h6>
            <button onclick={on_click_increase}>{"+1"}</button>
            <button onclick={on_click_decrease}>{"-1"}</button>
            <p>{ *counter }</p>
        </div>
    };
}
