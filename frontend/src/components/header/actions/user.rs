use std::rc::Rc;

use yew::{function_component, html, use_context, Callback, Html, MouseEvent, Properties};

use crate::components::utils::client_context::ClientContext;

#[function_component(User)]
pub fn user() -> Html {
    
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    let user_btn_onclick = {
        let client_context = client_context.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            client_context.modal_auth_display.set(true);
        })
    };
    
    html! {
        <li class="header-actions__item header-actions__item--user">
            <div class="header-actions__component">
                <button class="header__button" onclick={user_btn_onclick}>
                    <svg aria-hidden="true">
                        <use href="#icon-user-simple">
                            <symbol id="icon-user-simple" viewBox="0 0 24 24">
                                <g>
                                <path clip-rule="evenodd" d="m18 7.5c0 3.3137-2.6863 6-6 6-3.31375 0-6.00005-2.6863-6.00005-6 0-3.31371 2.6863-6 6.00005-6 3.3137 0 6 2.68629 6 6zm-2 0c0 2.20914-1.7909 4-4 4-2.20918 0-4.00005-1.79086-4.00005-4s1.79087-4 4.00005-4c2.2091 0 4 1.79086 4 4z" fill-rule="evenodd"></path>
                                <path d="m1.49996 22.5c.91192.4104.91152.4113.91152.4113l.00115-.0024c.00283-.006.00826-.0174.01636-.0339.01621-.0328.04305-.0857.08107-.1557.07611-.1402.19659-.3484.3657-.6021.33888-.5083.86856-1.1924 1.62187-1.8773 1.49422-1.3583 3.88685-2.7399 7.50237-2.7399 3.6154 0 6.0081 1.3816 7.5023 2.7399.7533.6849 1.283 1.369 1.6219 1.8773.1691.2537.2895.4619.3657.6021.038.07.0648.1229.081.1557.0081.0165.0136.0279.0164.0339l.0011.0024s-.0004-.0009.9116-.4113c.9119-.4104.9114-.4114.9114-.4114l-.0005-.0012-.0013-.0027-.0032-.0072-.0097-.0208c-.0079-.0167-.0186-.039-.0322-.0665-.0271-.055-.0659-.1311-.1169-.2251-.1021-.1879-.2534-.4485-.4593-.7573-.4112-.6167-1.044-1.4326-1.9407-2.2477-1.8057-1.6417-4.6631-3.2601-8.8476-3.2601-4.18457 0-7.04194 1.6184-8.84772 3.2601-.89669.8151-1.52951 1.631-1.94062 2.2477-.2059.3088-.357294.5694-.459306.7573-.05104.094-.089823.1701-.116976.2251-.01358.0275-.024262.0498-.032124.0665l-.009691.0208-.00328.0072-.001252.0027-.00053.0012s-.000465.001.911459.4114z"></path>
                                </g>
                            </symbol>
                        </use>
                    </svg>
                </button>
            </div>
        </li>
    }
}