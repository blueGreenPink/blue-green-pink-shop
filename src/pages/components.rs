use leptos::*;

#[component]
pub fn TitleCard() -> impl IntoView {
    view! {
        <div class="container">
            <div class="containerTitle">
                <p class="titleStyle--blue">Blue</p>
                <p class="titleStyle--green">Green</p>
                <p class="titleStyle--pink">Pink</p>
            </div>
        </div>
    }
}
#[component]
pub fn LandingText() -> impl IntoView {
    view! {
        <div class="container">
            <p class="textStyle">lifestyle brand</p>
        </div>
    }
}
