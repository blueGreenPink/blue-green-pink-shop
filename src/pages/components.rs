use leptos::*;

#[component]
pub fn TitleCard() -> impl IntoView {
    view! {
        <div class="container">
            <p class="textTitle--blue">Blue</p>
            <p class="textTitle--green">Green</p>
            <p class="textTitle--pink">Pink</p>
        </div>
    }
}
#[component]
pub fn LandingText() -> impl IntoView {
    view! {
        <div class="container">
            <p class="textSubtitle">lifestyle/apparel brand</p>
        </div>
    }
}
