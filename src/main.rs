use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Создаем эффект для прослушивания сообщений один раз при запуске
    create_effect(move |_| {
        let set_name = set_user_name.clone();
        let cb = Closure::wrap(Box::new(move |ev: MessageEvent| {
            if let Some(data) = ev.data().as_string() {
                if data.starts_with("login:") {
                    set_name.set(Some(data.replace("login:", "")));
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let _ = window().add_event_listener_with_callback(
            "message",
            cb.as_ref().unchecked_ref(),
        );
        cb.forget(); 
    });

    // Функция клика теперь "чистая" и не съедает переменные
    let open_login_window = move |_| {
        let _ = window().open_with_url_and_target_and_features(
            "https://www.f-list.net/login.php",
            "login_popup",
            "width=500,height=600",
        );
        set_user_name.set(Some("Ожидание входа...".into()));
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d;">
                <h2 style="margin: 0; color: #d62d2d;">"RUSSIAN CHAT"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window style="background: #d62d2d; color: white; border: none; padding: 10px 20px; cursor: pointer; border-radius: 5px;">
                                "ВХОД"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 15px; align-items: center;">
                                <span style="color: #4cd137;">"Привет, " {name} "!"</span>
                                <button on:click=move |_| set_user_name.set(None) style="background: #444; color: #ccc; border: none; padding: 5px 10px; cursor: pointer;">
                                    "Выйти"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="padding: 40px; text-align: center;">
                <div style="background: #1a1a1a; padding: 20px; border-radius: 10px; border: 1px solid #333;">
                    <h3 style="color: #d62d2d;">"Статус сборки: Стабилен"</h3>
                    <p>"E0525 побеждена через разделение логики клика и прослушивания событий."</p>
                </div>
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
