use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_app_entities::TodoRoute;
use yew_router::prelude::*;

#[function_component(CreateTodo)]
pub fn create_todo() -> Html {
    let navigator = use_navigator().unwrap();
    let n = navigator.clone();
    let on_cancel = Callback::from(move |_| n.push(&TodoRoute::Index));

    let title_ref = use_node_ref();
    let description_ref = use_node_ref();

    let n = navigator.clone();
    let on_submit = {
        let title_ref = title_ref.clone();
        let description_ref = description_ref.clone();
        Callback::from(move |_| {
            let title = title_ref.cast::<HtmlInputElement>().unwrap();
            let description = description_ref.cast::<HtmlTextAreaElement>().unwrap();
            yew_app_services::create_todo(title.value(), description.value());
            n.push(&TodoRoute::Index)
        })
    };

    return html! {
        <div class="container w-full prose flex flex-col justify-center align-middle mx-auto mt-10">
            <div class="card w-5/6 bg-neutral text-neutral-content">
                <div class="card-body items-center text-center">
                    <h2 class="card-title">{"Create Todo"}</h2>
                    <form class="w-full">
                        <input ref={title_ref} type="text" placeholder="Add the todo title" class="input w-full" />
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">{"Description"}</span>
                            </label>
                            <textarea ref={description_ref} class="textarea textarea-bordered h-24" placeholder="Add your descrption descrption"></textarea>
                        </div>
                    </form>
                    <div class="card-actions justify-end">
                        <button class="btn btn-primary" onclick={on_submit}>{"Create"}</button>
                        <button class="btn btn-ghost" onclick={on_cancel}>{"Cancel"}</button>
                    </div>
                </div>
            </div>
        </div>
    };
}
