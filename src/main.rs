mod activities;
mod account;
mod commissions;
mod contact;
mod home;
mod login;
mod merch;
mod nav;
mod stickers;
mod util;

use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(App)
}

#[component]
fn GenericPage() -> impl IntoView {
    view! {
        <nav::Navigation/>
        <Outlet/>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(account::User::new("Luc".to_owned(), "Very secret password".to_owned()));

    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="bg-bg-darkmode text-txt-dark selection:bg-light">
                <Routes>
                    <Route path="/" view=home::Home/>
                    <Route path="/activiteiten" view=GenericPage>
                        <Route path=":activity" view=activities::Activity/>
                        <Route path="" view=activities::Activities/>
                    </Route>
                    <Route path="/commissies" view=commissions::Commissions/>
                    <Route path="/merch" view=merch::Merch/>
                    <Route path="/stickers" view=stickers::Stickers/>
                    <Route path="/contact" view=contact::Contact/>
                </Routes>
            </main>
        </Router>
    }
}
