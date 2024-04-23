use yew::{function_component, html, Html};


#[function_component(HelpCentre)]
pub fn help_centre() -> Html {
    html! {
        <a class="button button--medium button--with-icon main-links__help" href="#help_centre">
            <svg>
                <use href="#icon-question-empty">
                    <symbol viewBox="0 0 24 24" id="icon-question-empty">
                        <g>
                            <path d="m10 9c0-.33998.1102-.8722.4115-1.2907.2646-.3675.7144-.7093 1.5885-.7093s1.3239.3418 1.5885.7093c.3013.4185.4115.95072.4115 1.2907 0 .66725-.3898 1.0138-1.3312 1.6863l-.0841.0599c-.8152.5799-2.0847 1.4832-2.0847 3.2538 0 .5523.4477 1 1 1s1-.4477 1-1c0-.6673.3898-1.0138 1.3312-1.6863l.0841-.0599c.8152-.5799 2.0847-1.4832 2.0847-3.2538 0-.66002-.1898-1.6278-.7885-2.4593-.6354-.8825-1.6856-1.5407-3.2115-1.5407s-2.57614.6582-3.21153 1.5407c-.59869.8315-.78847 1.79928-.78847 2.4593 0 .55228.44772 1 1 1 .55229 0 1-.44772 1-1z"></path>
                            <path d="m12.5 17c0-.5523-.4477-1-1-1s-1 .4477-1 1v.5c0 .5523.4477 1 1 1s1-.4477 1-1z"></path>
                            <path clip-rule="evenodd" d="m23 12c0 6.0751-4.9249 11-11 11-6.07513 0-11-4.9249-11-11 0-6.07513 4.92487-11 11-11 6.0751 0 11 4.92487 11 11zm-2 0c0 4.9706-4.0294 9-9 9-4.97056 0-9-4.0294-9-9 0-4.97056 4.02944-9 9-9 4.9706 0 9 4.02944 9 9z" fill-rule="evenodd"></path>
                        </g>
                    </symbol>
                </use>
            </svg>
            {"Help Centre"}
        </a>
    }
}

