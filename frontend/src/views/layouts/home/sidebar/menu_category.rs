use yew::{function_component, html, Html, Properties};

#[function_component(PerfumeIcon)]
pub fn perfume_icon() -> Html {
    html! {
        <svg width="24" height="24">
            <use href="#icon-fat-12258">
                <symbol viewBox="0 0 24 24" id="icon-fat-12258">
                    <path clip-rule="evenodd" d="m3.5625 6.90625c.46955 0 1.5625-.65877 1.5625-2.45313 0-1.79435-1.09295-2.45312-1.5625-2.45312s-1.5625.65877-1.5625 2.45312c0 1.79436 1.09295 2.45313 1.5625 2.45313zm0 2c1.96751 0 3.5625-1.99373 3.5625-4.45313 0-2.45939-1.59499-4.45312-3.5625-4.45312s-3.5625 1.99373-3.5625 4.45312c0 2.4594 1.59499 4.45313 3.5625 4.45313z" fill-rule="evenodd" transform="translate(2 13.0938)"></path>
                    <path d="m0 1c0-.552285.447715-1 1-1h7c.55228 0 1 .447715 1 1 0 .55228-.44772 1-1 1h-7c-.552285 0-1-.44772-1-1z" transform="translate(11 20)"></path>
                    
                    <g clip-rule="evenodd" fill-rule="evenodd">
                        <path d="m8.32549 7.89648c.44257-1.18669.67451-2.41354.67451-3.32511 0-1.20065-.38769-1.68526-.77122-1.95844-.50621-.36055-1.39254-.61293-2.72878-.61293s-2.22257.25238-2.72878.61293c-.38353.27318-.77122.75779-.77122 1.95844 0 .91157.23194 2.13842.67451 3.32511.32224.86406.7107 1.58528 1.10036 2.10352h3.45026c.38966-.51824.77812-1.23946 1.10036-2.10352zm-.2185 4.10352c1.72245-1.5678 2.89301-5.03507 2.89301-7.42863 0-3.47141-2.46248-4.57137-5.5-4.57137s-5.5 1.09996-5.5 4.57137c0 2.39356 1.17056 5.86083 2.89301 7.42863z" transform="translate(10 10)"></path><path d="m5 2h-3v2h3zm-3-2c-1.104569 0-2 .895431-2 2v2c0 1.10457.895431 2 2 2h3c1.10457 0 2-.89543 2-2v-2c0-1.104569-.89543-2-2-2z" transform="translate(12 6)"></path><path d="m3 2h-1v1h1zm-1-2c-1.104569 0-2 .895431-2 2v1c0 1.10457.895431 2 2 2h1c1.10457 0 2-.89543 2-2v-1c0-1.104569-.89543-2-2-2z" transform="translate(13 3)"></path>
                    </g>

                    <path d="m0 0h1.1875v1.1875h-1.1875z" transform="translate(15.0625 6.5625)"></path>
                    <path clip-rule="evenodd" d="m2.97878 2.28846c1.76485-1.620724 4.01939-2.28846 6.02122-2.28846v2c-1.58437 0-3.32984.53226-4.66844 1.76154-1.31736 1.20978-2.33156 3.17782-2.33156 6.23846h-2c0-3.53936 1.19269-6.07132 2.97878-7.71154z" fill-rule="evenodd" transform="translate(5 4)"></path>
                </symbol>
            </use>
        </svg>
    }
}

#[function_component(GraphicalTabletIcon)]
pub fn graphical_tablet_icon() -> Html {
    html! {
        <svg width="24" height="24">
            <use href="#icon-fat-2416">
                <symbol viewBox="0 0 24 24" id="icon-fat-2416">
                    <path clip-rule="evenodd" d="m2 2v10h12v-10zm-1-2c-.552285 0-1 .447715-1 1v13h16v-13c0-.552285-.4477-1-1-1z" fill-rule="evenodd" transform="translate(4 3)"></path>
                    <path clip-rule="evenodd" d="m2 2v2h16v-2zm-1-2c-.552285 0-1 .447715-1 1v4c0 .55228.447715 1 1 1h18c.5523 0 1-.44772 1-1v-4c0-.552285-.4477-1-1-1z" fill-rule="evenodd" transform="translate(2 15)"></path>
                    <path d="m2 1c0 .55228-.44772 1-1 1-.552285 0-1-.44772-1-1 0-.552285.447715-1 1-1 .55228 0 1 .447715 1 1z" transform="translate(10 6)"></path>
                    <path d="m2 1c0 .55228-.44772 1-1 1-.552285 0-1-.44772-1-1 0-.552285.447715-1 1-1 .55228 0 1 .447715 1 1z" transform="translate(7 6)"></path>
                    <path d="m2 1c0 .55228-.44772 1-1 1-.552285 0-1-.44772-1-1 0-.552285.447715-1 1-1 .55228 0 1 .447715 1 1z" transform="translate(7 9)"></path>
                </symbol>
            </use>
        </svg>
    }
}

#[derive(Properties, PartialEq)]
pub struct CategoryItemProps {
    link: String,
    title: String,
    icon: Html,
}

#[function_component(CategoryItem)]
pub fn category_item(props: &CategoryItemProps) -> Html {
    
    // JUST FORMATE AND SHOW
    
    html! {
        <li class="menu-categories__item">
            <a class="menu-categories__link" apprzroute="" href={props.link.clone()}>
                <span class="menu-categories__icon">
                    {props.icon.clone()}
                </span>
                {props.title.clone()}
            </a>
        </li>
    }
}

#[function_component(MenuCategories)]
pub fn menu_categories() -> Html {
    
    // GET ALL THE CATEGORIES FROM THE DB, ITER THEM AND SHOW

    html! {
        
        <ul class="menu-categories">
            
            <CategoryItem
                title={String::from("Graphical tablets")}
                link={String::from("#GraphicalTablets")}
                icon={ html! { <GraphicalTabletIcon/> } }
            />

            <CategoryItem
                title={String::from("Parfumery")}
                link={String::from("#Parfumery")}
                icon={ html! { <PerfumeIcon /> } }
            />           
                    
        </ul>
        
    }
}