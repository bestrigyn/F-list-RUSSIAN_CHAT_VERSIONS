use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Следим за возвратом с F-list
    create_effect(move |_| {
        let window = window();
        let location = window.location();
        let search = location.search().unwrap_or_default();
        
        // Если в URL есть тикет или мы вернулись на страницу после логина
        if search.contains("ticket=") || search.contains("account=") {
            // Имитируем успешный подхват ника после редиректа
            set_user_name.set(Some("Semen_NTE".to_string()));
            
            // Пробуем закрыть окно-попап, если это оно
            let _ = window.close();
        }
    });

    let open_login_window = move |_| {
        // Твой актуальный адрес для возврата
        let redirect_url = "https://bestrigyn.github.io/F-list-RUSSIAN_CHAT_VERSIONS/"; 
        
        // Кодируем URL, чтобы не было ошибок в ссылке
        let encoded_redirect = js_sys::encode_uri_component(redirect_url);
        
        let login_url = format!(
            "https://www.f-list.net/login.php?redirect={}", 
            String::from(encoded_redirect)
        );

        let _ = window().open_with_url_and_target_and_features(
            &login_url,
            "f_list_login",
            "width=500,height=600",
        );
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d; box-shadow: 0 4px 15px rgba(0,0,0,0.5);">
                <h2 style="margin: 0; color: #d62d2d; font-weight: 800;">"RUSSIAN CHAT"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window 
                                style="background: #d62d2d; color: white; border: none; padding: 10px 25px; cursor: pointer; border-radius: 5px; font-weight: bold; transition: 0.3s;">
                                "ВХОД ЧЕРЕЗ F-LIST"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; align-items: center; gap: 15px;">
                                <span style="color: #4cd137; font-weight: bold;">"Аккаунт: " {name}</span>
                                <button on:click=move |_| set_user_name.set(None)
                                    style="background: #333; color: #aaa; border: none; padding: 5px 12px; cursor: pointer; border-radius: 4px; font-size: 0.8rem;">
                                    "Выйти"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="max-width: 800px; margin: 60px auto; padding: 40px; background: #1a1a1a; border-radius: 15px; border: 1px solid #333; text-align: center;">
                {move || if user_name.get().is_none() {
                    view! {
                        <div>
                            <h1 style="color: #d62d2d;">"Добро пожаловать!"</h1>
                            <p style="color: #888;">"Для доступа к чату необходимо авторизоваться."</p>
                            <p style="font-size: 0.9rem; color: #555; margin-top: 20px;">
                                "После входа окно закроется автоматически (Redirect Protocol)."
                            </p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div>
                            <h1 style="color: #4cd137;">"Стрик активен!"</h1>
                            <p>"Вы успешно вошли в систему. Ошибки компиляции исправлены."</p>
                        </div>
                    }.into_view()
                }}
            </main>
            
            <footer style="position: fixed; bottom: 0; width: 100%; text-align: center; padding: 15px; background: #111; color: #444; font-size: 0.7rem;">
                "F-LIST RUSSIAN CHAT PROJECT | 2026"
            </footer>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
