use leptos::*;

#[component]
pub fn TitleCard() -> impl IntoView {
    view! {
        <div class="container">
            <p class="titleStyle">test_landing_page</p>
        </div>
    }
}
#[component]
pub fn LandingText() -> impl IntoView {
    view! {
        <div class="container">
            <p class="textStyle">small text</p>
        </div>
    }
}
