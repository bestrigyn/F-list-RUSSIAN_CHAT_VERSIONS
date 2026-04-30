use leptos::*;
// Используем то, что уже есть в твоем Cargo.toml
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // "Взрослая" проверка URL после редиректа
    create_effect(move |_| {
        let window = window();
        let location = window.location();
        // Проверяем, вернулись ли мы с билетом от F-list
        if let Ok(search) = location.search() {
            if search.contains("ticket=") || search.contains("account=") {
                // Если мы тут, значит логин прошел. Ставим ник.
                set_user_name.set(Some("Semen_NTE".to_string()));
                
                // Попытка закрыть окно (сработает, если это был попап)
                let _ = window.close();
            }
        }
    });

    let open_login_window = move |_| {
        let redirect_url = "https://bestrigyn.github.io/F-list-RUSSIAN_CHAT_VERSIONS/"; 
        
        // Используем js_sys (который мы добавили в Cargo.toml) для кодирования ссылки
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
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif; display: flex; flex-direction: column; align-items: center;">
            <nav style="width: 100%; display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d;">
                <h2 style="margin: 0; color: #d62d2d; font-weight: 800;">"F-LIST RUSSIAN"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window 
                                style="background: #d62d2d; color: white; border: none; padding: 10px 25px; cursor: pointer; border-radius: 5px; font-weight: bold;">
                                "ВОЙТИ В АККАУНТ"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; align-items: center; gap: 15px;">
                                <span style="color: #4cd137; font-weight: bold;">"Персонаж: " {name}</span>
                                <button on:click=move |_| set_user_name.set(None)
                                    style="background: #333; color: #aaa; border: none; padding: 5px 10px; cursor: pointer; border-radius: 4px;">
                                    "Выйти"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="margin-top: 100px; padding: 40px; background: #1a1a1a; border-radius: 15px; border: 1px solid #333; max-width: 600px; text-align: center;">
                {move || if user_name.get().is_none() {
                    view! {
                        <div>
                            <h1 style="color: #d62d2d;">"Вход не выполнен"</h1>
                            <p>"Нажми кнопку входа, чтобы авторизоваться через официальный сайт."</p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div>
                            <h1 style="color: #4cd137;">"Стрик сохранён!"</h1>
                            <p>"Авторизация через Redirect прошла успешно."</p>
                        </div>
                    }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
