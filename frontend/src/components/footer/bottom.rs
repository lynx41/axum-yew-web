use yew::{function_component, html, Html};
use yew_i18n::use_translation;

use crate::components::props::SelectedLangauge;


#[function_component(Bottom)]
pub fn bottom(prop: &SelectedLangauge) -> Html {
    
    let mut i18n = use_translation();

    i18n.set_translation_language(&prop.selected_language);
    
    html! { 
        
        <>
        
            <div class="footer-payments">
            
                <ul class="payments-buttons">
                
                    <li class="payments-buttons__item">
                        <button class="payments-buttons__button" title="MasterCard Secure">
                            <img loading="lazy" alt="MasterCard Secure" src="https://localhost:5000/public/system/images/locales/footer/mastercard-logo.svg" />
                        </button>
                    </li>

                    <li class="payments-buttons__item">
                        <button class="payments-buttons__button" title="Visa Verified">
                            <img loading="lazy" alt="Visa Verified" src="https://localhost:5000/public/system/images/locales/footer/visa-logo.svg" />
                        </button>
                    </li>

                </ul>
            
            </div>

            <p class="footer-copyright">
                { i18n.t("Copyright") }
                <span class="footer-copyright--gray">
                    { i18n.t("TM") }
                </span>
            </p>
        
        </>
    }
}