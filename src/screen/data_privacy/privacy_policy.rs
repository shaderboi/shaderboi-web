use crate::components::info_and_button_section::InfoAndButtonSection;
use crate::components::social_section::SocialSection;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct PrivacyPolicyScreen {
    link: ComponentLink<Self>,
}

impl Component for PrivacyPolicyScreen {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
                    <div>
                        <div class="container-middle-primary container-base">
                            <h1>{"Privacy & Policy"}</h1>
                            <p>{"Shaderboi is highly committed to protect our user’s privacy. Please take a moment to review this notice which explains what are the information that we collect about you, how we use it, and what are your rights toward your privacy."}</p>

<h2>{"Non-personal identification information"}</h2>
<p>{"We may collect non-personally identifiable information about users whenever they are using our service. Non-personal identification information may include technical information about users connected to our service, such as the operating system, Internet service provider used, and other similar information."}</p>

<h2>{"How we use the information collected"}</h2>
<p>{"Shaderboi may collect and use user's information for the following purposes: To personalize the user experience, we may use the overall information to understand how our users as a group use the services and resources provided on our service. To improve our service quality, we use telemetry to track non-personally identifiable information and measure user experiences using our services."}</p>

<h2>{"To make our service better"}</h2>
<p>{"We may use the feedback that users provide to improve our products and services, to run promotions, contests, surveys or other service features. In order to transmit information to users, users agree to receive on topics that we think are of interest to them. To send emails periodically we may use the email address to send user information and updates regarding user orders. And can be used to answer questions or other requests. If the user decides to opt-in to our mailing list, the user will receive an email which may include company news, updates, related product or service information, etc. If at any time the user wishes to unsubscribe from receiving future emails, we include detailed unsubscribe instructions at the bottom of each email."}</p>

<h2>{"How we protect your information"}</h2>
<p>{"We adopt appropriate data collection, storage and processing practices and security measures to protect against access, alteration, disclosure or destruction of personal information, passwords, transaction information and user data stored on our service. We also follow UU No 19, Tahun 2016 / GDPR / PDPA regulation about product law to make sure that any users data are save."}</p>

<p>{"Only the staff at Shaderboi who need access to the personal data necessary to provide our services have been granted access. They are also obliged to observe confidentiality. In the event that a data breach occurs, Shaderboi will, as soon as we become aware of it, notify data subjects in writing without undue delay in accordance with the provisions of applicable data protection legislation."}</p>

<h2>{"Sharing your personal information"}</h2>
<p>{"We do not sell, trade or rent User's personally identifiable information to others. We may share general collection demographic information that is not linked to any personally identifiable information about visitors and users with our business partners, trusted affiliates and advertisers for the purposes described above. We may use third party service providers to help us operate our business and the Site or administer activities on our behalf, such as sending out newsletters or surveys. We may share user information with these third parties for limited purposes provided that the User has given us permission. On third party websites, users may find advertisements or other content on our Sites that link to the sites and services of partners, suppliers, advertisers, sponsors, licensors and other third parties. We do not control the content or links that appear on this site and are not responsible for the practices of websites linked to or from our Site. In addition, these sites or services, including their content and links, are subject to change. These sites and services may have their own privacy policies and customer service policies. Browsing and interaction on any other website, including websites that have a link to our Site, is subject to the website's own terms and policies."}</p>

<h2>{"Advertising"}</h2>
<p>{"Advertisements appearing on our service or product may be sent to advertising partners, who may set cookies. These cookies allow the ad server to recognize a user's computer each time they send you an online advertisement to collect non-personally identifiable information about the User or other people who use your computer. This information allows the network, among other things, to serve targeted ads that are of most interest to them. This policy does not cover the use of cookies by any advertiser. For a service or product that includes children as the target, we followGoogle Families Ads and Monetization policy."}</p>

<h2>{"Compliance with children's online privacy protection measures"}</h2>
<p>{"Our team are fully aware about the concern of protecting children's privacy. Due to several important reasons, we never collect personal information on our service or product that are available for children under the age of 13 and no part of the service or product is structured to attract anyone under the age of 13. We do not collect personally identifiable information, we also strictly use the 3rd party SDKs, APIs, and ads that are safe for children."}</p>

<p>{"If you are the parent or guardian of a child and you believe that we have inadvertently received data about that child, please contact us through email that you can find at the very bottom of this notation. We will delete the data from our records as soon as possible. If you believe that we might have any data from or about a child under the age of 13, please contact immediately."}</p>

<h2>{"Changes to this site's privacy policy"}</h2>
<p>{"Shaderboi have the discretion to update the privacy policy at any time. When we update our privacy, we will post a notice on the main page of our site and send you an email. We encourage users to frequently check this page for changes to stay informed about how we can protect the personal information we collect. The user acknowledges and agrees that it is the user's responsibility to periodically review this privacy policy and be aware of any modifications."}</p>
<h2>{"Your acceptance of these terms"}</h2>
<p>{"By using our service or product (and this site), the user indicates acceptance of this policy. If you do not agree with this policy, please do not use our service or product. Continuing to use our service or product means that you are agree to following changes to the policy which will make users automatically accept those changes."}</p>

        <h2>{"According to the Data Protection Regulation data subjects have the right to"}</h2>
        <p>{"Request to receive a confirmation from Shaderboi about how we process personal data, and in such case request access to the personal data in the form of an extract from the register"}</p>
        <ul>
        <li>{"Correct incorrect or incomplete personal data"}</li>
        <li>{"Have your personal data deleted in some cases, e.g. if your personal data is no longer necessary for the purposes for which they were collected"}</li>
        <li>{"Object to processing that relies on legitimate interests, however only to the extent permitted by applicable data protection legislation"}</li>
        <li>{"Receive and transfer personal data to another controller in some cases (data portability)"}</li>
        </ul>
                    <p>
                        {"For any further question, feel free to contact us."}
                    </p>
                    <ul>
                        <li>{"Address: JL. Tapir I No. 10 M2 Kab. Cikarang Pusat, Prov. Jawa Barat, Indonesia"}</li>
                        <li>{"Email address: contact@shaderboi.id"}</li>
                    </ul>
                        </div>
                    </div>
                }
    }
}
