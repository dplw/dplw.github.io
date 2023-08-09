use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="home-page">
            // <DistroSpecificCodeBlock/>
        </div>
    }
}

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="header">
            <div class="logo-container">
                <img src="./assets/images/logo_white_thick.svg" class="logo"/>
                <p>"Dumb People Linux Wiki"</p>
            </div>
            <div class="nav-container">
                <div class="search-box-container">
                    <input type="text" placeholder="Search..." class="search-box"/>
                </div>
                <div class="nav-elements">
                    <div class="nav-element">"Home"</div>
                    <div class="nav-element">"Home"</div>
                    <div class="nav-element">"Home"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn DistroSpecificCodeBlock(cx: Scope) -> impl IntoView {
    let (current_distro, set_current_distro) = create_signal(cx, "Arch".to_string());

    view! { cx,
        <div class="distro-specific-code-block">
            <button
                class="distro-button"
                class:selected-distro={ move || current_distro() == "Ubuntu".to_string() }
                on:click=move |_| set_current_distro("Ubuntu".to_string())
            >
                "Ubuntu"
            </button>
            <button
                class="distro-button"
                class:selected-distro={ move || current_distro() == "Fedora".to_string() }
                on:click=move |_| set_current_distro("Fedora".to_string())
            >
                "Fedora"
            </button>
            <button
                class="distro-button"
                class:selected-distro={ move || current_distro() == "Arch".to_string() }
                on:click=move |_| set_current_distro("Arch".to_string())
            >
                "Arch"
            </button>
        </div>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // provide_meta_context(cx);
    view! { cx,
        <div class="root">
        <Router>
            <nav>
                <Header/>
            </nav>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=move |cx| view! { cx, <HomePage/> }
                    ></Route>
                </Routes>
            </main>
        </Router>
        </div>
    }
}
