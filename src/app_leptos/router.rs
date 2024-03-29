use leptos::*;
use leptos_router::*;

use crate::pages::home::PageHome;
use crate::pages::shop::PageShop;

#[component]
pub fn SiteRouter() -> impl IntoView {
    view! {
        <div class="containerNav">
            <Router>
                <nav>
                    <A class="textNav" exact=true href="/">"home"</A>
                    <A class="textNav" href="shop">"shop"</A>
                </nav>
                <main>
                    <Routes>
                        <Route path="" view=|| view! { <PageHome/> }/>
                        <Route path="shop" view=|| view! { <PageShop/> }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
