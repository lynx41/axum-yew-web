use yew::{function_component, html, Html};

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    html! {
        <div class="header-search">
            <form class="search-form">
                
                // Input line
                <div class="search-form__inner">
                    <div class="search-form__input-wrapper">
                        // Input itself
                        <input class="search-form__input" type="text" autocomplete="off" name="search" formcontrolname="text" placeholder="Я хочу знайти..." />
                    
                        // input clear button - it shows if input.is_not_empty() 
                        <button class="search-form__clear" type="button" style="display: none" aria-label="Очистити пошук">
                            <svg>
                                <use href="#icon-close-modal">
                                    <symbol id="icon-close-modal" viewBox="0 0 24 24">
                                        <path d="m18.295 7.11511c.3894-.38936.3894-1.02064 0-1.41-.3893-.38936-1.0206-.38936-1.41 0l-4.885 4.88499-4.88499-4.88499c-.38936-.38936-1.02063-.38936-1.41 0-.38936.38936-.38936 1.02064 0 1.41l4.88499 4.88499-4.88499 4.885c-.38936.3894-.38936 1.0206 0 1.41.38937.3894 1.02064.3894 1.41 0l4.88499-4.885 4.885 4.885c.3894.3894 1.0207.3894 1.41 0 .3894-.3894.3894-1.0206 0-1.41l-4.885-4.885z"></path>
                                    </symbol>
                                </use>
                            </svg>
                        </button>
                    </div>
                </div>

                // Submit button
                <button class="button button_color_green button_size_medium search-form__submit">
                    {"Знайти"}
                </button>
            </form>
        </div>
    }
}