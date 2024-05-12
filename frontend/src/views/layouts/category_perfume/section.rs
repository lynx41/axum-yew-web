use std::ops::Deref;

use gloo::{console::log, net::http::Request, storage::{LocalStorage, Storage}, utils::window};
use shared::models::categories::perfume::{PerfumeQuery, PerfumeTile};

use yew::{function_component, html, platform::spawn_local, use_effect_with, use_state, Callback, Html, Properties};


#[derive(Properties, PartialEq)]
pub struct CatalogTileProps {
    pub tile_picture_src: String,
    pub product_page_src: i32,
    pub old_price: Option<i32>,
    pub price: i32,
    pub title: String,
}

#[function_component(CatalogTile)]
pub fn catalog_tile(props: &CatalogTileProps) -> Html {

    let price_change: Option<f32> = match props.old_price {
        Some(old_price) => { Some(((props.price - old_price) as f32 / old_price as f32) * 100.0) },
        None => None
    };

    html! {
        <li class="catalog-grid__cell catalog-grid__cell_type_slim">
            <div class="goods-tile">
                <div class="goods-tile-inner">
                    <div class="goods-tile__content">
                        
                        // On condition, if the goods has old price
                        if let Some(price_percentage) = price_change {
                            <span class="goods-tile__label promo-label promo-label__type_action">{format!("{:.0}%", price_percentage)}</span>
                        }
                        
                        // Icons to add to WishList (Only for users, no guests) and Compare the goods (in future)
                        <div class="goods-tile__actions">
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

                        // Product page
                        <a class="product-link goods-tile__picture" href={"#LINK_TO_THIS_ONE_GOODS"} title={props.title.clone()}>
                            <img loading="lazy" alt={props.title.clone()} title={props.title.clone()} src={props.tile_picture_src.clone()} />
                        </a>

                        <div class="goods-tile__colors"></div>

                        <a class="product-link goods-tile__heading" href={"#LINK_TO_THIS_ONE_GOODS"} title={props.title.clone()}>
                            <span class="goods-tile__title">{props.title.clone()}</span>
                        </a>

                        <div class="goods-tile__prices">
                            
                            if let Some(old_price) = props.old_price {
                                <div class="goods-tile__price--old price--gray">
                                    {old_price}
                                    <span class="currency">{"₴"}</span>
                                </div>
                            }

                            <div class={ if props.old_price.is_some() { "goods-tile__price price--red" } else { "goods-tile__price" } }>
                                <p class="">
                                    <span class="goods-tile__price-value">
                                        {props.price}
                                        <span class="goods-tile__price-currency currency">{"₴"}</span>
                                    </span>
                                </p>

                                <button class="buy-button goods-tile__buy-button" aria-label={"Buy"}>
                                    <svg width="24" height="24" aria-hidden="true">
                                        <use href="#icon-busket">
                                            <symbol viewBox="0 0 24 24" id="icon-busket">
                                                <g>
                                                    <path fill-rule="evenodd" clip-rule="evenodd" d="M1 2C0.447715 2 0 2.44772 0 3C0 3.55228 0.447715 4 1 4H2.68121C3.08124 4 3.44277 4.2384 3.60035 4.60608L8.44161 15.9023C9.00044 17.2063 10.3963 17.9405 11.7874 17.6623L19.058 16.2082C20.1137 15.9971 20.9753 15.2365 21.3157 14.2151L23.0712 8.94868C23.7187 7.00609 22.2728 5 20.2251 5H5.94511L5.43864 3.81824C4.96591 2.71519 3.88129 2 2.68121 2H1ZM10.2799 15.1145L6.80225 7H20.2251C20.9077 7 21.3897 7.6687 21.1738 8.31623L19.4183 13.5827C19.3049 13.9231 19.0177 14.1767 18.6658 14.247L11.3952 15.7012C10.9315 15.7939 10.4662 15.5492 10.2799 15.1145Z"></path>
                                                    <path d="M11 22C11 23.1046 10.1046 24 9 24C7.89543 24 7 23.1046 7 22C7 20.8954 7.89543 20 9 20C10.1046 20 11 20.8954 11 22Z"></path>
                                                    <path d="M21 22C21 23.1046 20.1046 24 19 24C17.8954 24 17 23.1046 17 22C17 20.8954 17.8954 20 19 20C20.1046 20 21 20.8954 21 22Z"></path>
                                                </g>
                                            </symbol>
                                        </use>
                                    </svg>
                                </button>
                            </div>
                        </div>

                    </div>
                </div>
            </div>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub href: Option<String>,
    pub updated: bool,
    pub on_update: Callback<bool>,
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    
    let goods_vec = use_state(|| Vec::<PerfumeTile>::new());
    // collect all applyed filters

    if props.updated {
        log!("TRUE FOR UPDATE");
        props.on_update.emit(true);
        {
            let goods_vec = goods_vec.clone();
            let href = props.href.clone();

                spawn_local(async move {
                    
                    let fetched_request = match href {
                        Some(link) => { Request::get(&format!("https://localhost:5000/perfume?{}", link)) },
                        None => { Request::get("https://localhost:5000/perfume") }
                    }
                        .send()
                        .await;
                    
                    match fetched_request {
                        Ok(response) => {
                            let json: Result<Vec<PerfumeTile>, gloo::net::Error>= response.json().await;

                            match json {
                                Ok(v) => goods_vec.set(v),
                                Err(e) => log!(e.to_string())
                            }
                        },
                        Err(_) => {}
                    }
                })
        }
    };

    // get the query results
    use_effect_with(
        (),
        {
            let goods_vec = goods_vec.clone();
            let href = props.href.clone();
            move |()| {
                let goods_vec = goods_vec.clone();
                let href = href.clone();
                spawn_local(async move {
                    
                    let fetched_request = Request::get("https://localhost:5000/perfume")
                        .send()
                        .await;

                    match fetched_request {
                        Ok(response) => {
                            let json: Result<Vec<PerfumeTile>, gloo::net::Error>= response.json().await;

                            match json {
                                Ok(v) => goods_vec.set(v),
                                Err(e) => log!(e.to_string())
                            }
                        },
                        Err(_) => {}
                    }
                })
            }
        }
    );
    
    html! {
        <section class="content content_type_catalog">

            <ul class="catalog-grid">
                
                {
                    for goods_vec.iter().map(|item| {
                        html! { <CatalogTile
                            tile_picture_src={item.tile_picture_src.clone()}
                            product_page_src={item.product_page_src}
                            old_price={item.old_price}
                            price={item.price}
                            title={item.title.clone()}
                        /> }
                    })
                }

            </ul>

        </section>
    }
}