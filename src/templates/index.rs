use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, View};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

const SITE_LINK_CLASS: &str = "site-link";
const SITE_ITEM_CLASS: &str = "site-item";
const SITE_LIST_CLASS: &str = "site-list";
const SELF_SUMMARY_CLASS: &str = "self-summary";
const SELF_IMAGE_PATH: &str = "/.perseus/static/images/face.png";
const SELF_IMAGE_CLASS: &str = "self-image";
const MAIN_CLASS: &str = "main acrylic ";
const INNER_CLASS: &str = "inner border-clear";
const SUMMARY_TEXT_CLASS: &str = "summary-text";
const SELF_TITLE: &str = "self-title";

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
    view! {
        div(class=MAIN_CLASS) {
            div(class=INNER_CLASS) {
                div(class=SELF_SUMMARY_CLASS) {
                    h1(class=SELF_TITLE) { "Sean Myers" }
                    img(src=SELF_IMAGE_PATH, class=SELF_IMAGE_CLASS) {}
                    p(class=SUMMARY_TEXT_CLASS) { span() {"Software Engineer"} br() {} span() {"Washington D.C., USA."} }
                }
                div() {
                    ul(class=SITE_LIST_CLASS) {
                        li(class=SITE_ITEM_CLASS) {
                            a(
                                href="https://github.com/seanpmyers",
                                class=SITE_LINK_CLASS,
                                target="_blank"
                            ) { "GitHub" }
                        }
                        li(class=SITE_ITEM_CLASS) {
                            a(
                                href="https://www.linkedin.com/in/spmyers/",
                                class=SITE_LINK_CLASS,
                                target="_blank"
                            ) { "LinkedIn" }
                        }
                    }
                }
            }
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: String::from(""),
    })
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Sean Myers" }
    }
}
