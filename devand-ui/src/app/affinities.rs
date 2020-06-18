use crate::app::components::LanguageTag;
use crate::app::services::AffinitiesService;
use devand_core::{PublicUserProfile, UserAffinity};
use yew::{prelude::*, Properties};

#[derive(Default)]
pub struct State {
    affinities: Option<Vec<UserAffinity>>,
}

pub enum Msg {
    AffinitiesFetchOk(Vec<UserAffinity>),
    AffinitiesFetchErr,
}

pub struct AffinitiesPage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    affinities_service: AffinitiesService,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for AffinitiesPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();

        let callback = link.callback(|result: Result<Vec<UserAffinity>, anyhow::Error>| {
            if let Ok(affinities) = result {
                Msg::AffinitiesFetchOk(affinities)
            } else {
                Msg::AffinitiesFetchErr
            }
        });

        let mut affinities_service = AffinitiesService::new(callback);

        affinities_service.restore();

        Self {
            props,
            state,
            affinities_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AffinitiesFetchOk(mut affinities) => {
                affinities.sort_unstable_by_key(|x| x.affinity);
                self.state.affinities = Some(affinities);
            }
            Msg::AffinitiesFetchErr => {
                log::error!("Affinities fetch error");
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Affinities" }</h1>
                {
                if let Some(affinities) = &self.state.affinities {
                    view_affinities(affinities)
                } else {
                    view_loading()
                }
                }
            </>
        }
    }
}

fn view_affinities(affinities: &Vec<UserAffinity>) -> Html {
    if affinities.is_empty() {
        view_no_affinities()
    } else {
        html! {
            <table class="user-affinities pure-table-striped">
            { for affinities.iter().rev().enumerate().map(|(i, a)| view_affinity(a,i)) }
            </table>
        }
    }
}

fn view_affinity(affinity: &UserAffinity, i: usize) -> Html {
    let UserAffinity { user, affinity } = affinity;

    let PublicUserProfile {
        visible_name,
        languages,
        ..
    } = user;

    let languages = languages.clone().to_sorted_vec();

    let languages_tags = languages.iter().map(|(lang, pref)| {
        html! {
            <LanguageTag lang=lang pref=pref />
        }
    });

    html! {
        <tr class=("user-affinity")>
            <td class="affinity">{ affinity.to_string() }</td>
            <td class="visible_name">{ visible_name }</td>
            <td class="languages"> { for languages_tags } </td>
        </tr>
    }
}

fn view_loading() -> Html {
    html! {
        <p>{ "Loading..."}</p>
    }
}

fn view_no_affinities() -> Html {
    html! {
        <p>{ "No matching users found" }</p>
    }
}
