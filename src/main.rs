use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    // Состояние для хранения Email
    let (user_email, set_user_email) = create_signal(None::<String>);

    // Эффект для автоматического закрытия окна и подхвата данных
    create_effect(move |_| {
        let window = window();
        let location = window.location();
        let search = location.search().unwrap_or_default();
        
        // Если в ссылке появилось подтверждение возврата
        if search.contains("redirect=1") || search.contains("account") {
            // Ставим заглушку Email (в реальном API тут будет запрос к f-list)
            set_user_email.set(Some("user@f-list.net".to_string()));
            
            // ЗАКРЫВАЕМ ОКНО
            let _ = window.close();
        }
    });

    let open_login_window = move |_| {
        // Твой сайт, куда нужно вернуться
        let my_site = "https://bestrigyn.github.io/F-list-RUSSIAN_CHAT_VERSIONS/?redirect=1";
        let encoded_site = js_sys::encode_uri_component(my_site);
        
        // Ссылка сразу на настройки, где есть Email
        let login_url = format!(
            "https://www.f-list.net/login.php?redirect={}",
            String::from(encoded_site)
        );

        let _ = window().open_with_url_and_target_and_features(
            &login_url,
            "f_list_login",
            "width=600,height=700",
        );
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d;">
                <h2 style="margin: 0; color: #d62d2d;">"RUSSIAN CHAT"</h2>
                <div>
                    {move || match user_email.get() {
                        None => view! {
                            <button on:click=open_login_window style="background: #d62d2d; color: white; border: none; padding: 10px 20px; cursor: pointer; border-radius: 5px; font-weight: bold;">
                                "ВХОД / EMAIL"
                            </button>
                        }.into_view(),
                        Some(email) => view! {
                            <span style="color: #4cd137; font-weight: bold;">"Email: " {email}</span>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="padding: 50px; text-align: center;">
                {move || if user_email.get().is_none() {
                    view! { 
                        <div>
                            <h1>"Окно авторизации"</h1>
                            <p>"Нажмите вход. После логина на F-list окно закроется автоматически."</p>
                        </div>
                    }.into_view()
                } else {
                    view! { 
                        <h1 style="color: #4cd137;">"Авторизация по Email успешна!"</h1> 
                    }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
