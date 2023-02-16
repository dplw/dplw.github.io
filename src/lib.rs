use leptos::*;
use leptos_meta::*;
// use leptos_router::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <h1>"Dumb People Linux Wiki"</h1>
            <h2>"Under construction!"</h2>
        </div>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! { cx,
        <Stylesheet id="leptos" href="/pkg/styles/global_styles.css"/>
        <HomePage/>
    }
}
