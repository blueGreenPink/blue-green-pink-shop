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
            name: "black heart tee".to_string(),
            image_url: "".to_string(),
            shop_url: "https://www.bonfire.com/black-heart-tee/".to_string(),
            price: 45,
        },
        Item {
            name: "bob original sweater".to_string(),
            image_url: "".to_string(),
            shop_url: "https://www.bonfire.com/bob-original-tee/".to_string(),
            price: 35,
        },
        Item {
            name: "bpg logo sweater".to_string(),
            image_url: "https://imagedelivery.net/mKjOxek3QZwwBWUu82qeLg/1c4c6334-8782-4c23-e784-eb400fd33900/public".to_string(),
            shop_url: "https://www.bonfire.com/bgp-logo-sweater/".to_string(),
            price: 45,
        },
        Item {
            name: "happy guys tee ii".to_string(),
            image_url: "https://imagedelivery.net/mKjOxek3QZwwBWUu82qeLg/531e237b-34a2-403c-36c0-0f1a5d920000/public".to_string(),
            shop_url: "https://www.bonfire.com/happy-guys-tee-ii-1/".to_string(),
            price: 35,
        },
        Item {
            name: "bob cluster tee".to_string(),
            image_url: "https://imagedelivery.net/mKjOxek3QZwwBWUu82qeLg/22a4c5de-515c-4b39-6cba-9549fea80c00/public".to_string(),
            shop_url: "https://www.bonfire.com/bob-cluster-tee/".to_string(),
            price: 35,
        },
        Item {
            name: "happy guys tee".to_string(),
            image_url: "https://imagedelivery.net/mKjOxek3QZwwBWUu82qeLg/a66cfc43-3cf7-4543-5d4e-175f2a960e00/public".to_string(),
            shop_url: "https://www.bonfire.com/happy-guys-tee/".to_string(),
            price: 35,
        },
        Item {
            name: "bpg logo tee".to_string(),
            image_url: "https://imagedelivery.net/mKjOxek3QZwwBWUu82qeLg/b22a2e0d-c549-40a4-b0d5-52dc997c9d00/public".to_string(),
            shop_url: "https://www.bonfire.com/bgp-logo-tee/".to_string(),
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
                            <a href={child.shop_url.clone()} target="_blank">
                                <img class="imageGridItem" src={child.image_url} alt={child.name.clone()} />
                            </a>
                            <a class="textLink" href={child.shop_url.clone()} target="_blank">{child.name.clone()}</a>
                        </div>
                    </For>
            </div>
            <div class="spacer">
            </div>
        </div>
    }
}
