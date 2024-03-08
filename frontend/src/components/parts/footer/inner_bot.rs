use yew::{function_component, html, Html};

#[function_component(InnerBot)]
pub fn inner_bot() -> Html {
    html! {
        
        <div class="footer-top__links" place="footer">
            
            // Block 1 "About the company"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 

                </ul>

                // 
            </div>

            // Block 2 "Help"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                </ul>

                // 
            </div>

            // Block 3 "Services"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                </ul>

                // 
            </div>

            // Block 4 "Partnership"
            <div class="footer-links">
                // Box Title
                <div class="footer-links__item footer-links__item--heading">
                    <h3 class="footer-links__label">{"Інформацію про компанію"}</h3>
                </div>
                
                // Box table
                <ul class="footer-links__list">
                    
                    // About us
                    <li class="footer-links__item">
                        <a href="#about_company">{"Про нас"}</a>
                    </li>

                    // Terms and Conditions of Use
                    <li class="footer-links__item">
                        <a href="#terms_and_conditions">{"Умови користування сайту"}</a>
                    </li> 
                    
                </ul>

                // 
            </div>
        
        </div>
    }
}