use yew::{function_component, html, Html};
use yew_i18n::use_translation;

use crate::routes::Prop;

#[function_component(InnerTop)]
pub fn inner_top(prop: &Prop) -> Html {
    
    let mut i18n = use_translation();

    i18n.set_translation_language(&prop.selected_language);
    
    html! {

        // Socials
        <div class="footer-top__social">
            
            // Text line
            <a class="button button--small button--with-icon button--link footer-top__schedule" href="#contacts/">
                <svg viewBox="0 0 512 512" aria-hidden="true" xmlns="http://www.w3.org/2000/svg">
                    <path d="M464 256A208 208 0 1 1 48 256a208 208 0 1 1 416 0zM0 256a256 256 0 1 0 512 0A256 256 0 1 0 0 256zM232 120V256c0 8 4 15.5 10.7 20l96 64c11 7.4 25.9 4.4 33.3-6.7s4.4-25.9-6.7-33.3L280 243.2V120c0-13.3-10.7-24-24-24s-24 10.7-24 24z"/>
                </svg>
                { i18n.t("title") }
            </a>

            // Social Icons
            <div class="social-icons">
                <ul class="socials__list">
                    
                    // Facebook Icon
                    <li class="socials__list-item">
                        <a class="socials__link socials__link--facebook" target="_blank" href="#go_to_facebook" title="Facebook" aria-lable="Facebook">
                            <svg aria-hidden="true">
                                <use href="#icon-facebook">
                                    <symbol id="icon-facebook" viewBox="0 0 24 24">
                                        <path d="m15.6376 11.9807h-2.6695v10.0193h-4.04452v-10.0193h-1.92358v-3.52113h1.92358v-2.27859c0-1.62945.75551-4.18098 4.08052-4.18098l2.9959.01284v3.41789h-2.1737c-.3566 0-.8579.18251-.8579.9598v2.07232h3.0225z"></path>
                                    </symbol>
                                </use>
                            </svg>
                        </a>
                    </li>

                    // X (Twitter) Icon
                    <li class="socials__list-item">
                        <a class="socials__link socials__link--twitter" target="_blank" href="#go_to_twitter" title="Twitter" aria-lable="Twitter">
                            <svg aria-hidden="true">
                                <use href="#icon-twitter">
                                    <symbol id="icon-twitter" viewBox="0 0 24 24">
                                        <path d="m22 5.89987c-.7489.3263-1.5432.54067-2.3566.63602.8572-.50444 1.4984-1.29865 1.804-2.23431-.8055.4702-1.6866.80158-2.6055.97984-1.2699-1.33746-3.2951-1.66713-4.9351-.80333-1.6399.86379-2.4843 2.705-2.0575 4.48655-3.29949-.16296-6.37356-1.6966-8.45725-4.21928-1.09105 1.84483-.53504 4.20655 1.26926 5.39134-.65171-.0197-1.28909-.19302-1.85868-.50529v.05054c.00012 1.92235 1.3771 3.57835 3.29203 3.95895-.60424.1618-1.23811.1855-1.85309.0692.53743 1.6449 2.07787 2.7718 3.83345 2.8044-1.45347 1.1224-3.24841 1.731-5.09601 1.7279-.32717.0003-.65407-.0184-.97901-.056 1.87639 1.1863 4.06019 1.8159 6.29047 1.8136 7.54743 0 11.67443-6.1526 11.67443-11.48904 0-.17466-.0044-.34931-.0122-.52287.8036-.57021 1.497-1.27745 2.0473-2.08822z"></path>
                                    </symbol>
                                </use>
                            </svg>
                        </a>
                    </li>

                    // Youtube Icon
                    <li class="socials__list-item">
                        <a class="socials__link socials__link--youtube" target="_blank" href="#go_to_youtube" title="Youtube" aria-lable="Youtube">
                            <svg aria-hidden="true">
                                <use href="#icon-youtube">
                                    <symbol id="icon-youtube" viewBox="0 0 24 24">
                                        <path fill-rule="evenodd" clip-rule="evenodd" d="M19.8144 5.41728C20.6786 5.65197 21.352 6.32558 21.5822 7.18578C22 8.74562 22 12 22 12C22 12 22 15.2544 21.5811 16.8131C21.3517 17.6738 20.678 18.3477 19.8133 18.5816C18.2544 19 12 19 12 19C12 19 5.74556 19 4.18556 18.5827C3.32137 18.348 2.64803 17.6744 2.41778 16.8142C2 15.2544 2 12 2 12C2 12 2 8.74562 2.41778 7.18578C2.64776 6.32526 3.32186 5.65173 4.18667 5.41839C5.74556 5 12 5 12 5C12 5 18.2544 5 19.8144 5.41728ZM9.95465 9.04588V14.9541L15.1824 12L9.95465 9.04588Z"></path>
                                    </symbol>
                                </use>
                            </svg>
                        </a>
                    </li>

                    // Instagram Icon
                    <li class="socials__list-item">
                        <a class="socials__link socials__link--instagram" target="_blank" href="#go_to_instagram" title="Instagram" aria-lable="Instagram">
                            <svg aria-hidden="true">
                                <use href="#icon-instagram">
                                    <symbol id="icon-instagram" viewBox="0 0 24 24">
                                        <path d="m21.94 7.87667c-.0489-1.06445-.2178-1.79111-.4644-2.42778-.2556-.65778-.5978-1.21556-1.1534-1.77111-.5555-.55556-1.1133-.89778-1.7711-1.15334-.6367-.24666-1.3633-.41666-2.4278-.46444-1.0666-.04889-1.4077-.06-4.1233-.06-2.71556 0-3.05667.01111-4.12333.06-1.06445.04889-1.79111.21778-2.42667.46444-.65778.25556-1.21556.59778-1.77111 1.15334-.55556.55555-.89778 1.11333-1.15333 1.77111-.24778.63667-.41667 1.36333-.46556 2.42778-.04889 1.06666-.06 1.40777-.06 4.12333 0 2.7156.01111 3.0567.06 4.1233.04889 1.0645.21778 1.7911.46444 2.4278.25556.6578.59778 1.2156 1.15334 1.7711.55555.5567 1.11333.8978 1.77111 1.1534.63667.2477 1.36333.4166 2.42778.4644 1.06666.0489 1.40777.06 4.12333.06 2.7156 0 3.0567-.0122 4.1233-.06 1.0645-.0489 1.7911-.2178 2.4278-.4644.6578-.2556 1.2156-.5978 1.7711-1.1534.5556-.5555.8978-1.1133 1.1534-1.7711.2466-.6355.4166-1.3633.4644-2.4278.0489-1.0666.06-1.4077.06-4.1233 0-2.71556-.0111-3.05667-.06-4.12333zm-1.7878 8.17003c-.0444.9766-.2078 1.5066-.3444 1.86-.1607.4347-.4165.828-.7489 1.1511-.3229.3325-.7163.5884-1.1511.7489-.3534.1377-.8834.3011-1.86.3444-1.0556.0478-1.3722.0589-4.0467.0589-2.67443 0-2.9911-.01-4.04666-.0589-.97666-.0444-1.50666-.2078-1.85999-.3444-.46778-.1811-.80112-.3989-1.15112-.7489s-.56777-.6845-.74889-1.1511c-.13666-.3534-.3-.8834-.34444-1.86-.05-1.0556-.06-1.3723-.06-4.0467 0-2.67444.01-2.99111.05889-4.04667.04444-.97666.20778-1.50666.34444-1.86.18223-.46777.39889-.80111.74889-1.15111.32294-.33249.71628-.58839 1.15111-.74889.35334-.13666.88334-.3 1.86-.34444 1.05556-.04889 1.37223-.06 4.04667-.06 2.6744 0 2.9911.01 4.0467.05889.9766.04444 1.5066.20778 1.86.34444.4677.18222.8011.39889 1.1511.74889s.5678.68445.7489 1.15111c.1366.35334.3.88334.3444 1.86.0478 1.05667.0578 1.37334.0578 4.04668.0011 2.6755-.0089 2.9922-.0567 4.0478zm-8.1522-9.17892c-2.83445 0-5.13222 2.29777-5.13222 5.13222 0 2.8344 2.29777 5.1322 5.13222 5.1322 2.8344 0 5.1322-2.2978 5.1322-5.1322 0-2.83445-2.2978-5.13222-5.1322-5.13222zm0 8.47442c-1.1943 0-2.2978-.6371-2.89493-1.6714-.59713-1.0342-.59713-2.3085 0-3.3427.59713-1.0343 1.70063-1.67143 2.89493-1.67143 1.8462 0 3.3428 1.49663 3.3428 3.34273 0 1.8462-1.4966 3.3428-3.3428 3.3428zm5.3289-9.86887c-.6618 0-1.1983.53652-1.1983 1.19834s.5365 1.19833 1.1983 1.19833 1.1983-.53651 1.1983-1.19833-.5365-1.19834-1.1983-1.19834z"></path>
                                    </symbol>
                                </use>
                            </svg>
                        </a>
                    </li>

                    // Viber Icon
                    <li class="socials__list-item">
                        <a class="socials__link socials__link--viber" target="_blank" href="#go_to_viber" title="Viber" aria-lable="Viber">
                            <svg aria-hidden="true">
                                <use href="#icon-viber">
                                    <symbol id="icon-viber" viewBox="0 0 24 24">
                                        <g>
                                            <path d="m19.4936 13.21c-.479 4.0643-3.2999 4.3206-3.8199 4.4966-.222.0751-2.2789.6144-4.8659.4353l-2.80389 3.1988-.003-3.6856-.022-.0062c-3.7669-1.1023-3.6399-5.2355-3.5979-7.4051.042-2.16953.42899-3.94696 1.57896-5.14186 2.06394-1.96989 6.31583-1.67553 6.31583-1.67553 3.5909.01646 5.3129 1.15579 5.7108 1.53762 1.326 1.19593 2.001 4.05608 1.507 8.24597zm-.526-9.26076c-.472-.45697-2.3779-1.91534-6.6228-1.9349 0 0-5.00587-.31802-7.4468 2.04091-1.35797 1.43161-1.83595 3.52604-1.88695 6.12375-.05 2.5977-.116 7.4648 4.33688 8.7852l.004.0011-.002 2.0141s-.028.8162.48099.9819c.61598.2017.97797-.4179 1.56695-1.0858.323-.3664.76903-.9047 1.10493-1.3164 3.046.2686 5.3879-.3479 5.6539-.4384.615-.211 4.0949-.6803 4.6609-5.5464.5829-5.02042-.284-8.19344-1.85-9.62506z"></path>
                                            <path d="m16.2517 13.8168c-.2782-.2451-.5612-.4843-.849-.7173-.262-.2028-.68-.4755-.914-.6145-.419-.2459-.8479-.0916-1.0249.1503l-.368.4868c-.188.247-.479.2048-.555.1894-.543-.14-1.121-.4899-1.711-1.0786-.598-.5939-.95894-1.2145-1.08894-1.8598l-.009-.0422c-.03959-.1973.00126-.40267.113-.56809l.02-.01647c.24804-.23774.70794-.52695.76994-.80174.181-.47858-.6039-1.47485-.71594-1.65084 0 0-.57099-.77807-.77398-.92628-.219-.18834-.55899-.29538-.90098-.07822l-.017.01338c-.98397.68545-1.33996 1.0786-1.24296 1.66628l.05099.23671c.49899 1.61893 1.46196 3.34487 2.89893 4.88147 1.42094 1.5181 2.95894 2.4269 4.47384 3.0568.394.1348.802.0185 1.221-.35l.007-.0051c.321-.3088.575-.6392.765-.9932l.002-.0072c.183-.3767.12-.7307-.151-.9716zm-2.359-3.5456c-.061-.0005-.1108-.0504-.1129-.1132-.02-.41989-.13-.741-.327-.9561-.196-.21305-.49-.33244-.872-.35405-.0629-.00494-.1105-.06072-.107-.12556.001-.03131.0142-.06089.0365-.08216.0224-.02126.0521-.03244.0825-.03105.442.0247.786.16879 1.023.42712.2359.2573.3679.62987.3899 1.1095.0035.0652-.0446.1211-.108.1255z"></path>
                                            <path d="m15.0135 10.6674h-.003c-.0632-.0028-.1123-.0575-.11-.1225.017-.80584-.211-1.48615-.68-2.02545-.2339-.26993-.5214-.48507-.843-.6309-.3588-.15801-.7414-.25131-1.131-.27582-.0629-.006-.1096-.06274-.1049-.12762.0037-.065.0578-.11467.1209-.11116.904.06999 1.618.40139 2.125.98495.509.58458.758 1.32252.739 2.1912 0 .0644-.0505.1167-.113.1173z"></path>
                                            <path d="m16.1613 11.1404c-.0302-.0002-.0591-.0129-.0803-.0351s-.0329-.0521-.0327-.0832c-.005-.7122-.1089-1.35651-.3109-1.91537-.1935-.5442-.4996-1.03864-.897-1.44911-.3816-.39642-.8365-.71-1.338-.92216-.5199-.21127-1.074-.31885-1.6329-.317-.0632-.00168-.1132-.05544-.112-.12041 0-.06458.0502-.11724.113-.11836 1.2399.00926 2.2929.44873 3.1289 1.30605.419.43124.739.9479.951 1.53557.2109.58458.3199 1.25769.3259 1.99769.0012.065-.0488.1187-.112.1204z"></path>
                                        </g>
                                    </symbol>
                                </use>
                            </svg>
                        </a>
                    </li>

                    // Telegram Icon
                    <li class="socials__list-item">
                        <a class="socials__link socials__link--telegram" target="_blank" href="#go_to_viber" title="Telegram" aria-lable="Telegram">
                            <svg aria-hidden="true">
                                <use href="#icon-telegram">
                                    <symbol id="icon-telegram" viewBox="0 0 24 24">
                                        <path clip-rule="evenodd" d="m2.3749 11.3184c5.36869-2.3991 8.9487-3.98073 10.7399-4.7449 5.1144-2.18184 6.1771-2.56085 6.8698-2.57337.1523-.00275.493.03598.7136.21961.1863.15506.2376.36452.2621.51154.0245.14701.0551.48191.0308.74359-.2771 2.98678-1.4764 10.23493-2.0865 13.58013-.2581 1.4155-.7664 1.8901-1.2585 1.9366-1.0695.1009-1.8816-.7249-2.9174-1.4214-1.6209-1.0897-2.5366-1.7681-4.1099-2.8315-1.81827-1.229-.63957-1.9044.3967-3.0083.2711-.2889 4.9832-4.68493 5.0744-5.08372.0114-.04987.022-.23579-.0857-.33395-.1077-.09817-.2666-.0646-.3813-.0379-.1625.03784-2.7519 1.79327-7.76804 5.26617-.73498.5177-1.4007.7699-1.99716.7567-.65755-.0146-1.92242-.3814-2.86272-.6949-1.15332-.3845-2.069953-.5878-1.99013-1.2408.04157-.3402.49825-.688 1.37005-1.0436z" fill-rule="evenodd"></path>
                                    </symbol>
                                </use>
                            </svg>
                        </a>
                    </li>

                </ul>
            </div>

        </div>

    }
}