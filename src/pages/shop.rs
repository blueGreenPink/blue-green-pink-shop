use leptos::IntoView;
use leptos::*;

#[derive(Debug, Clone)]
struct Item {
    name: String,
    image_url: String,
    shop_url: String,
    price: u16,
}

#[component]
pub fn PageShop() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        Item {
            name: "bpg logo tee".to_string(),
            image_url: "".to_string(),
            shop_url: "https://www.bonfire.com/bgp-logo-tee/".to_string(),
            price: 35,
        },
        Item {
            name: "happy guys tee".to_string(),
            image_url: "".to_string(),
            shop_url: "https://www.bonfire.com/happy-guys-tee/".to_string(),
            price: 35,
        },
    ]);
    view! {
        <div class="container">
            <div class="containerShopGrid">
                    <For
                        each=data
                        key=|state| state.name.clone()
                        let:child
                    >
                        <div class="containerShopItem">
                            <a class="textLink" href={child.shop_url} target="_blank">{child.name}</a>
                        </div>
                    </For>
            </div>
        </div>
    }
}
