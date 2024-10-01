use chrono::NaiveDate;
use gray_matter::{Matter, engine::YAML};
use leptos::*;
use leptos_router::*;
use pulldown_cmark::{html, Options, Parser};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};

use crate::util::*;

#[derive(Embed)]
#[folder = "./public/activities"]
#[include = "*.md"]
pub struct ActivitiesFolder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMetadata {
    title: String,
    date: String,
}

#[derive(Debug, Clone)]
pub struct ActivityData {
    title: String,
    date: NaiveDate,
    body: String
}

pub fn parse_gray_matter(content: &str) -> (ActivityMetadata, String) {
    let matter = Matter::<YAML>::new();

    let gray_matter = matter
        .parse_with_struct::<ActivityMetadata>(content)
        .expect("Unable to parse md frontmatter");

    (gray_matter.data, gray_matter.content)
}

pub fn get_activities() -> Vec<ActivityData> {
    let mut activities = embedded_names::<ActivitiesFolder>()
        .into_iter()
        .zip(ActivitiesFolder::iter())
        .map(|(name, path)| {
            let content =
                String::from_utf8_lossy(&ActivitiesFolder::get(&path).unwrap().data).into_owned();
            let (metadata, content) = parse_gray_matter(&content);
            let date = NaiveDate::parse_from_str(&metadata.date, "%Y-%m-%d").unwrap_or_else(|e| {
                panic!(
                    "Couldn't parse date {} for post '{}': {e}",
                    metadata.date, name
                )
            });

            let mut options = Options::empty();
            options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
            
            let parser = Parser::new_ext(&content, options);

            let mut body = String::new();
            html::push_html(&mut body, parser);

            ActivityData {
                title: metadata.title,
                date,
                body
            }
        })
        .collect::<Vec<_>>();

    activities.sort_by(|a1, a2| a2.date.cmp(&a1.date));

    activities
}

#[component]
pub fn Activities() -> impl IntoView {
    view! { "Activiteiten" }
}

#[component]
pub fn Activity() -> impl IntoView {
    let params = use_params_map();
    let query = move || params.with(|p| p.get("activity").cloned().unwrap_or_default());

    view! {
        {query}
        {move || {
            get_activities()
                .into_iter()
                .find(|a| a.title.replace(" ", "_") == query())
                .map(|a| view! {
                    <ActivityCard activity=a/>
                }).unwrap_or_else(|| view! {
                    <div>"No thingy with that name!"</div>
                }.into_view())
        }}
    }
}

#[component]
pub fn ActivityCard(activity: ActivityData) -> impl IntoView {
    view! {
        <A href=format!("activiteiten/{}", activity.title.replace(" ", "_"))>
            <div class="bg-grey-highlight p-4 rounded-b-xl hover:bg-black/50 border-t-4 border-link">
                <p class="text-link text-lg">{activity.date.to_string()}</p>
                <p class="text-xl py-2">{activity.title}</p>
                <p
                    inner_html=activity.body
                    class="prose prose-posts prose-h1:text-2xl prose-h1:line-clamp-1"
                ></p>
            </div>
        </A>
    }
}

#[component]
pub fn ActivityCards() -> impl IntoView {
    let activities = get_activities();

    view! {
        <div class="grid lg:grid-cols-3 lg:gap-4">
            {activities
                .into_iter()
                .map(|activity| view! { <ActivityCard activity/> })
                .collect_view()}

        </div>
    }
}