use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Эффект, который следит за URL. Если мы вернулись от F-list, забираем ник.
    create_effect(move |_| {
        let location = window().location();
        let search = location.search().unwrap_or_default();
        
        // Если в ссылке есть билет (ticket), значит вход успешен
        if search.contains("ticket=") {
            // В реальном API тут будет запрос за ником, пока вытащим заглушку
            set_user_name.set(Some("Semen_NTE".to_string()));
            
            // МАГИЯ: Закрываем текущее окно, если оно было открыто как попап
            let _ = window().close();
        }
    });

    let open_login_window = move |_| {
        // Указываем F-list, куда вернуть пользователя после логина
        let redirect_url = "https://твоя-страница.github.io/"; 
        let login_url = format!(
            "https://www.f-list.net/login.php?redirect={}", 
            js_sys::encode_uri_component(redirect_url)
        );

        let _ = window().open_with_url_and_target_and_features(
            &login_url,
            "f_list_login",
            "width=500,height=600",
        );
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d;">
                <h2 style="margin: 0; color: #d62d2d;">"RUSSIAN CHAT"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window style="background: #d62d2d; color: white; border: none; padding: 10px 20px; cursor: pointer; border-radius: 5px;">
                                "ВХОД ЧЕРЕЗ F-LIST"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <span style="color: #4cd137;">"Привет, " {name} "!"</span>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="padding: 50px; text-align: center;">
                {move || if user_name.get().is_none() {
                    view! { <p>"Нажмите вход, авторизуйтесь, и это окно обновится автоматически."</p> }.into_view()
                } else {
                    view! { <h1 style="color: #4cd137;">"Авторизация прошла успешно!"</h1> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
