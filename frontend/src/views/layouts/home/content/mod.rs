use std::ops::Deref;
use std::rc::Rc;

use gloo::console::log;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use shared::models::categories::perfume::PerfumeGoods;
use yew::platform::spawn_local;
use yew::{function_component, html, use_context, use_effect_with, use_state, Html, Properties};

use crate::components::utils::client_context::ClientContext;
use crate::components::props::IsAuth;

#[derive(Properties, PartialEq)]
pub struct SectionItemProps {
    tile_picture_src: String,
    product_page_src: i32,
    product_desc: String,
    old_price: Option<i32>,
    price: i32,
    title: String,
    // Debug mode
    is_debug: bool,
    brand: String,
    volume: String,
    seassonality: String,
    class: String,
}

#[function_component(SectionItem)]
pub fn section_item(props: &SectionItemProps) -> Html {
    
    // HERE GOES THE SINGLE DB ITEM FROM ITER, AND WE JUST NEED TO SHOW IT INSIDE TEMPLATE
    let client_contex = use_context::<Rc<ClientContext>>().unwrap();


    html! {
        <li class="main-goods__cell">
            <div class="tile">
                <div class="tile__inner">
                    <a class="product-link tile__picture" href={props.product_page_src.to_string()} title="">
                        <img loading="lazy" rzimgui="" alt={props.product_desc.clone()} src={props.tile_picture_src.clone()} />
                    </a>
                    <a class="product-link tile__title" apprzroute="" href={props.product_page_src.to_string()} title={props.product_desc.clone()}>
                        {props.title.clone()}
                    </a>
                    
                    if { *client_contex.is_auth == IsAuth::Yes } {
                        <div class="tile__actions">
                            <div class="wish-wrapper">
                                <button class="wish-button" aria-label="Add to the wish-list">
                                    <svg width="24" height="24" aria-hidden="true">
                                        <use href="#icon-heart-empty">
                                            <symbol viewBox="0 0 24 24" id="icon-heart-empty">
                                                <path clip-rule="evenodd" d="m3.4181 5.31884c.9661-1.14226 2.37454-1.81884 4.0819-1.81884 1.14319 0 2.23774.62595 3.0785 1.26152.5191.39237 1.0029.83608 1.4215 1.26141.4186-.42533.9024-.86904 1.4215-1.26141.8408-.63557 1.9353-1.26152 3.0785-1.26152 1.7379 0 3.1462.75107 4.0986 1.93888.9358 1.16719 1.4014 2.71241 1.4014 4.31112 0 1.4435-.7114 2.8288-1.6063 4.0219-.9086 1.2116-2.0982 2.3461-3.2535 3.3088-1.1605.9671-2.3162 1.7854-3.1793 2.3607-.4324.2883-.7935.517-1.0478.6745-.1272.0787-.2279.1397-.2975.1815-.0349.0209-.0619.037-.0807.0481l-.022.013-.0061.0036-.0019.0011c-.0002.0001-.001.0006-.5049-.8632-.5039.8638-.5041.8637-.5043.8635l-.0025-.0015-.0063-.0036-.022-.013c-.0189-.0112-.046-.0273-.0809-.0483-.0698-.0419-.1707-.1031-.298-.1822-.2547-.1581-.6162-.3879-1.0491-.678-.86379-.5791-2.02057-1.4045-3.18207-2.3853-1.15656-.9766-2.34684-2.1315-3.2556-3.3734-.89758-1.2266-1.59923-2.645-1.59923-4.12779 0-1.60112.46738-3.10749 1.4181-4.23157zm8.5819 14.18116-.5043.8635.5043.2942.5039-.2939zm.0005-1.1719c.2246-.1408.5147-.3265.8511-.5508.8244-.5496 1.9187-1.3251 3.0082-2.233 1.0947-.9123 2.1551-1.934 2.934-2.9724.7926-1.057 1.2062-2.0154 1.2062-2.8219 0-1.22952-.3599-2.3093-.9618-3.06005-.5854-.73014-1.4271-1.18995-2.5382-1.18995-.4663 0-1.1176.28637-1.8724.85695-.7209.54494-1.3915 1.23793-1.8686 1.79414l-.759.88481-.759-.88481c-.4771-.55621-1.1477-1.2492-1.86856-1.79414-.7548-.57058-1.40613-.85695-1.87244-.85695-1.14164 0-1.9832.4345-2.55485 1.11039-.58703.69408-.94515 1.71291-.94515 2.94002 0 .86699.42335 1.86719 1.21327 2.94669.77874 1.0643 1.83846 2.1031 2.9319 3.0264 1.0885.9192 2.18173 1.7 3.00543 2.2521.336.2253.6256.4115.8499.5525z" fill-rule="evenodd">
                                                </path>
                                            </symbol>
                                        </use>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    }
                    
                    <div class="tile__prices">
                        
                        // If the item is on sale                        
                        if { props.old_price.is_some() } {
                            <div class="tile__prices-old">
                                <span>
                                    {props.old_price}
                                    <span class="currency">{"₴"}</span>
                                </span>
                            </div>

                            <div class="tile__price tile__price_color_red">
                                {props.price}
                                <span class="tile__price-currency currency">{"₴"}</span>
                            </div>
                        } else {
                            <div class="tile__price">
                                {props.price}
                                <span class="tile__price-currency currency">{"₴"}</span>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    title: String,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {

    // HERE GOES THE DB QUERY RESULTS, YOU ITER THEM AND SHOW RESULTS
    let goods_vec = use_state(|| Vec::<PerfumeGoods>::new());
    let is_debug = use_state(|| true);

    use_effect_with(
        (),
        {
            let goods_vec = goods_vec.clone();
            move |()| {
                let goods_vec = goods_vec.clone();

                spawn_local(async move {               
                    match LocalStorage::get::<String>("UniqueID") {
                        Ok(unique_id) => {
                            let fetched_request = Request::get("https://localhost:5000/get_suggestions")
                                .header("UniqueID", &unique_id)
                                .send()
                                .await;

                            match fetched_request {
                                Ok(response) => {
                                    let json: Result<Vec<PerfumeGoods>, gloo::net::Error> = response.json().await;

                                    match json {
                                        Ok(v) => goods_vec.set(v),
                                        Err(e) => { log!(e.to_string()); }
                                    }
                                },
                                Err(e) => { log!(e.to_string()); }
                            }
                        },
                        Err(e) => { log!(e.to_string()); }
                    }
                })
            }
        }
    );


    html! {
        <section class="main-goods">
            <h2 class="main-goods__heading">{props.title.clone()}</h2>
            <ul class="main-goods__grid">

                {
                    for goods_vec.iter().map(|item| {
                        html! {
                            <SectionItem
                                tile_picture_src={item.tile_picture_src.clone()}
                                product_page_src={item.product_page_src}
                                product_desc={item.product_big_desc.clone()}
                                old_price={item.old_price}
                                price={item.price}
                                title={item.title.clone()}
                                is_debug={is_debug.deref()}
                                brand={item.brand.clone()}
                                volume={item.volume.clone()}
                                seassonality={item.seasonality.clone()}
                                class={item.class.clone()}
                            
                            />
                        }
                    })
                }
            </ul>
        </section>
    }
}

#[function_component(Content)]
pub fn content() -> Html {
    
    
    html! {
        <main class="content content_type_main">          
            <div class="main-page-content">

                <Section title={String::from("Особисті рекомендації")} />

                // <Section title={String::from("Specials for you")} />

                // <Section title={String::from("Популярні товари")} />

            </div>
        </main>
    }
}