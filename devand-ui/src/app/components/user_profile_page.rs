use crate::app::components::LanguageTag;
use crate::app::elements::busy_indicator;
use crate::app::services::UserProfileService;
use crate::app::{AppRoute, RouterButton};
use devand_core::PublicUserProfile;
use yew::{prelude::*, Properties};

pub struct UserProfilePage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    service: UserProfileService,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub username: String,
}

pub enum Msg {
    OtherUserLoaded(Option<PublicUserProfile>),
}

#[derive(Default)]
struct State {
    other_user: Option<PublicUserProfile>,
}

impl Component for UserProfilePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let other_user_loaded_callback = link.callback(Msg::OtherUserLoaded);

        let mut service = UserProfileService::new(other_user_loaded_callback);

        service.load_other_user(&props.username);

        let state = State::default();

        Self {
            props,
            service,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OtherUserLoaded(user) => {
                log::info!("user loaded {:?}", &user);
                self.state.other_user = user;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut changed = false;

        if self.props.username != props.username {
            self.service.load_other_user(&props.username);
            changed = true;
        }

        self.props = props;
        changed
    }

    fn view(&self) -> Html {
        if let Some(other_user) = &self.state.other_user {
            let PublicUserProfile { languages, .. } = other_user;

            let languages = languages.clone().to_sorted_vec();

            let languages_tags = languages.iter().map(|(lang, pref)| {
                html! { <LanguageTag lang=lang pref=pref /> }
            });

            html! {
            <>
                <h1><RouterButton route=AppRoute::Chat(other_user.username.clone())>{ "💬 " }</RouterButton>{ other_user.full_name() }</h1>
                <h2>{"Languages"}</h2>
                <p class="devand-user-bio">{ &other_user.bio }</p>
                <div>
                    { for languages_tags }
                </div>
            </>
            }
        } else {
            busy_indicator()
        }
    }
}

impl UserProfilePage {}
