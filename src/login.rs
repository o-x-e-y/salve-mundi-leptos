use leptos::*;
use crate::account::*;

const LOGIN_CLASS: &'static str = "bg-light/80 my-auto px-4 py-1 ml-4 mr-3 lg:mr-6 rounded-lg
    border border-transparent hover:bg-white/15 hover:border-white/10";

#[component]
pub fn Login() -> impl IntoView {
    let (show_login, set_show_login) = create_signal(false);

    view! {
        <button on:click=move |_| set_show_login(true) class=LOGIN_CLASS>
            "Log in"
        </button>
        {move || {
            if show_login() {
                view! {
                    <button
                        on:click=move |_| set_show_login(false)
                        class="absolute inset-0 w-screen h-screen backdrop-blur-sm"
                    >
                        <div on:click=|ev| ev.stop_propagation() class="absolute top-4 right-4">
                            <LoginBox set_show_login/>
                        </div>
                    </button>
                }
                    .into_view()
            } else {
                ().into_view()
            }
        }}
    }
}

#[component]
fn LoginBox(set_show_login: WriteSignal<bool>) -> impl IntoView {
    let button_class = "bg-bg-darkmode px-2 py-1 border border-white/40 rounded-md";
    let user = expect_context::<User>();

    let username_ref = create_node_ref::<html::Input>();
    let password_ref = create_node_ref::<html::Input>();

    create_effect(move |_| {
        if let Some(u) = username_ref() {
            let _ = u.focus();
        }
    });

    let log_in = move || {
        let username = match username_ref() {
            Some(username) => username.value(),
            None => return,
        };

        let password = match password_ref() {
            Some(password) => password.value(),
            None => return,
        };

        user.update(username, password);
        user.log_in();

        set_show_login(false);
    };

    let focus_ref = move |node_ref: NodeRef<html::Input>| if let Some(node_ref) = node_ref() {
        let _ = node_ref.focus();
    };

    // Navigate to a search page when pressing enter or blur the search box when pressing esc
    let on_keydown_login = move |ev: ev::KeyboardEvent| match ev.key().as_str() {
        "Escape" => set_show_login(false),
        "Enter" => log_in(),
        _ => {}
    };

    view! {
        <div
            on:keydown=on_keydown_login
            class="grid gridflow-row gap-4 p-4 bg-bg-darkmode rounded-lg border border-white/20 hover:cursor-default"
        >
            <label name="username">
                <input
                    on:click=move |_| focus_ref(username_ref)
                    placeholder="username"
                    type="text"
                    node_ref=username_ref
                    class=button_class
                />
            </label>
            <label name="password">
                <input
                    on:click=move |_| focus_ref(password_ref)
                    placeholder="password"
                    type="text"
                    node_ref=password_ref
                    class=button_class
                />
            </label>
            <div class="flex justify-center">
                <label name="submit">
                    <button on:click=move |_| log_in() class=button_class>
                        "Submit"
                    </button>
                </label>
            </div>
        </div>
    }
}

#[component]
pub fn Account() -> impl IntoView {
    let user = expect_context::<User>();

    view! {
        <button on:click=move |_| user.log_out() class=LOGIN_CLASS>
            "Account"
        </button>
    }
}