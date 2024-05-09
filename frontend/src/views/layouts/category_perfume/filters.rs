use gloo::console::log;
use web_sys::HtmlElement;
use yew::{function_component, html, Callback, Html, MouseEvent, TargetCast};

#[function_component(Filters)]
pub fn filters() -> Html {
    
    
    // checkbox onclick
    let onclick = {
        Callback::from(move |e: MouseEvent| {

            // write to storage, the storage will be sended to backend
            
            let target = e.target_unchecked_into::<HtmlElement>();
            let class_name = target.class_name();
            if class_name.contains("checkbox-filter__link--checked") {
                target.set_class_name("checkbox-filter__link");
            } else {
                target.set_class_name("checkbox-filter__link checkbox-filter__link--checked");
            }

        })
    };

    html! {

        <aside class="sidebar">
            // sidebar-block with search-bar | when disabled has the role sidebar-block_state_collapsed
            <div class="sidebar-block">
                <button class="sidebar-block__toggle">
                    <span class="sidebar-block_toggle-title">
                        {"Бренд"}
                        // <span class="sidebar-block__toggle-quantity">{"1"}</span>
                    </span>

                    <svg class="sidebar-block__toggle-icon">
                        <use href="#icon-angle-left">
                            <symbol viewBox="0 0 24 24" id="icon-angle-left">
                                <path clip-rule="evenodd" d="m16.726 21.6877c-.3799.401-1.0128.4181-1.4137.0383l-10.26633-9.726 10.26633-9.72595c.4009-.37984 1.0338-.36273 1.4137.0382.3798.40094.3627 1.03387-.0383 1.4137l-8.73367 8.27405 8.73367 8.274c.401.3799.4181 1.0128.0383 1.4137z" fill-rule="evenodd"></path>
                            </symbol>
                        </use>
                    </svg>
                </button>

                <div class="sidebar-block__inner" style="overflow-x: hidden;">
                    // <div class="sidebar-search">
                    //     <input class="sidebar-search__input" placeholder="Пошук" />
                    // </div>

                    <div class="scrollbar__layout">
                        <div class="scrollbar__inner" style="width: 251px">
                            <div style="width: 233px;">
                                <ul class="checkbox-filter">
                                    
                                    <li class="checkbox-filter__item" onclick={onclick}>
                                        <a class="checkbox-filter__link">{"Zero 0"}</a>
                                    </li>

                                    {
                                        for (0..75).into_iter().map(|_| {
                                            html! { 
                                                <li class="checkbox-filter__item">
                                                    <a class="checkbox-filter__link">{"First 1"}</a>
                                                </li>
                                            }
                                        })
                                    }

                                    <li class="checkbox-filter__item">
                                        <a class="checkbox-filter__link checkbox-filter__link--checked">{"Second 2"}</a>
                                    </li>

                                </ul>
                            </div>
                        </div>

                        // <div class="scrollbar__holder" style="right: 0px">
                        //     <div class="scrollbar__path">
                        //         <div class="scrollbar__slider" style="height: 69.7785px; transform: translateY(0px);"></div>
                        //     </div>
                        // </div>
                    </div>
                </div>

            </div>
        </aside>

    }
}