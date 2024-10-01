use leptos::*;
use leptos_router::*;

use crate::{
    account::*,
    activities::*,
    nav::*
};

const THIS_IS_SALVE_MUNDI: &str = include_str!("../public/text/home-this-is-salve-mundi.txt");

#[component]
pub fn Home() -> impl IntoView {
    let user = expect_context::<User>();

    let name = move || match user.logged_in() {
        true => view! {
            <p class="font-bold">{user.username}!</p>
        },
        false => view! {
            <p>"Mundi!"</p>
        }
    };

    view! {
        <SalveMundiVideo/>
        <Navigation/>
        <p class="text-4xl text-lighter flex justify-center py-4 sm:py-8">
            <p>"Salve\u{A0}"</p>
            {name}
        </p>
        <article>
            <div class="mx-4 sm:mx-12 md:mx-24 lg:mx-36 xl:mx-48">
                <p>
                    {THIS_IS_SALVE_MUNDI}
                </p>
                <A href="/activiteiten">
                    <h2 class="text-3xl text-lighter py-4 sm:py-8">
                        "Aankomende Activiteiten"
                    </h2>
                </A>
                
                <ActivityCards/>
                <h2 class="text-3xl text-lighter py-4 sm:py-8">
                    "Onze partners"
                </h2>
            </div>
            <Sponsors/>
        </article>
        <Footer/>
    }
}

#[component]
pub fn SalveMundiVideo() -> impl IntoView {
    let (vid_expanded, set_vid_expanded) = create_signal(false);
    let max_h = move || match vid_expanded() {
        true => "150vh",
        false => "12rem",
    };

    view! {
        <div
            on:click=move |_| set_vid_expanded.update(|v| *v = !*v)
            style:max-height=max_h
            class="transition-all duration-1000 ease-in-out overflow-y-hidden"
        >
            <video
                poster="/public/images/video-poster-gradient.svg"
                width="100%"
                preload="none"
                autoplay="true"
                disablepictureinpicture="true"
                disableremoteplayback="true"
                muted="true"
                loop="true"
            >
                <source src="/public/videos/intro2023.mp4" type="video/mp4"/>
            </video>
        </div>
    }
}

#[component]
fn Sponsors() -> impl IntoView {
    view! {
        <div class="w-screen py-4 sm:py-8 bg-[#333] flex justify-center overflow-x-scroll">
            <Sponsor src="/public/images/ssc-eindhoven-logo-white.svg"/>
            <Sponsor src="/public/images/knaek-logo-white.svg"/>
            <Sponsor src="/public/images/borrelbar-logo-white.png"/>
            <Sponsor src="/public/images/fontys-logo-white.svg"/>
            <Sponsor src="/public/images/hubble-logo-white.png"/>
        </div>
    }
}

#[component]
fn Sponsor(src: &'static str) -> impl IntoView {
    view! {
        // <div class="w-1/5">
            <img
                src=src
                class="h-20 mx-4 sm:mx-8"
            />
        // </div>
    }
}