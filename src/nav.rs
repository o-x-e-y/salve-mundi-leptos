use leptos::*;
use leptos_router::*;

use crate::{
    account::*,
    login::*
};

#[component]
pub fn Navigation() -> impl IntoView {
    let user = expect_context::<User>();

    view! {
        <header class="w-full bg-sm-primary">
            <nav class="flex p-4 sm:pl-10 lg:pl-14 border-b-2 border-white/15">
                <A href="/">
                    <img
                        class="h-8 sm:h-10 w-auto"
                        src="/public/images/logo_old.svg"
                        alt="Salve Mundi"
                    />
                </A>
                <NavElements>
                    <NavElem text="Commissies" href="/commissies"/>
                    <NavElem text="Activiteiten" href="/activiteiten"/>
                    <NavElem text="Merch" href="/merch"/>
                    <NavElem text="Stickers" href="/stickers"/>
                    <NavElem text="Contact" href="/contact"/>
                    {move || if !user.logged_in() {
                        view! {
                            <Login/>
                        }.into_view()
                    } else {
                        view! {
                            <Logout/>
                        }.into_view()
                    }}
                </NavElements>
            </nav>
        </header>
    }
}

#[component]
fn NavElements(children: Children) -> impl IntoView {
    view! {
        <div class="w-full flex justify-end text-[#eee] visited:text-[#eee]">
            <ul 
                // style:text-shadow="-0.5px 1px rgba(0,0,0,0.5)"
                class="hidden md:flex list-none text-lg lg:text-xl hover:text-white lg:gap-3"
            >
                {children()}
            </ul>
            <div class="flex md:hidden">
                <img
                    class="h-8 sm:h-10 w-auto mr-1"
                    src="/public/images/hamburger.svg"
                    alt="Menu"
                />
            </div>
        </div>
    }
}

#[component]
fn NavElem(text: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A href class="my-auto px-2 py-1 rounded-lg hover:bg-white/5">
            <li>{text}</li>
        </A>
    }
}
