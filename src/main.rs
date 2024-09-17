mod activities;
mod commissions;
mod contact;
mod home;
mod merch;
mod nav;
mod stickers;

use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
            <main class="bg-darker text-txt-dark selection:bg-sm-light">
                <nav::Navigation></nav::Navigation>
                <Routes>
                    <Route path="/" view=home::Home/>
                    <Route path="/activiteiten" view=activities::Activities/>
                    <Route path="/commissies" view=commissions::Commissions/>
                    <Route path="/merch" view=merch::Merch/>
                    <Route path="/stickers" view=stickers::Stickers/>
                    <Route path="/contact" view=contact::Contact/>
                </Routes>
            </main>
        </Router>
    }
}
