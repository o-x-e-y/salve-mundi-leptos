use leptos::*;
use leptos_router::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <header class="w-full bg-sm-primary">
            <nav class="flex p-4 sm:px-10 lg:px-14 border-b-2 border-white/15">
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
                    <Login/>
                </NavElements>
            </nav>
        </header>
    }
}

#[component]
fn NavElements(children: Children) -> impl IntoView {
    view! {
        <ul // style:text-shadow="-0.5px 1px rgba(0,0,0,0.5)"
        class="list-none hidden w-full justify-end text-lg lg:text-xl text-[#eee] visited:text-[#eee]
        hover:text-white sm:flex sm:gap-2 md:gap-3">{children()}</ul>
    }
}

#[component]
fn Login() -> impl IntoView {
    view! {
        <li class="bg-sm-light/80 my-auto px-4 py-1 rounded-lg hover:bg-white/5 sm:ml-4 lg:ml-8">
            "Log in"
        </li>
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
