use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct LoginData {
    user: String,
}

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Слушаем сообщение от callback.html
    create_effect(move |_| {
        let window = window();
        let cb = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Проверяем данные через JS Reflection, чтобы достать ник
            if let Ok(js_obj) = e.data().dyn_into::<js_sys::Object>() {
                if let Ok(user_val) = js_sys::Reflect::get(&js_obj, &"user".into()) {
                    if let Some(name) = user_val.as_string() {
                        set_user_name.set(Some(name));
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let _ = window.add_event_listener_with_callback("message", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let open_login = move |_| {
        let callback = "https://bestrigyn.github.io/F-list-RUSSIAN_CHAT_VERSIONS/callback.html";
        let login_url = format!(
            "https://www.f-list.net/login.php?redirect={}",
            js_sys::encode_uri_component(callback)
        );

        let _ = window().open_with_url_and_target_and_features(
            &login_url,
            "f_list_auth",
            "width=500,height=600"
        );
    };

    view! {
        <div style="background: #000; color: #fff; min-height: 100vh; font-family: sans-serif;">
            <nav style="background: #1a1a1a; padding: 20px; border-bottom: 2px solid #f00; display: flex; justify-content: space-between;">
                <h2 style="margin: 0; color: #f00;">"F-LIST RUSSIAN CHAT"</h2>
                {move || match user_name.get() {
                    None => view! {
                        <button on:click=open_login style="background: #f00; color: #fff; border: none; padding: 10px 20px; cursor: pointer; font-weight: bold; border-radius: 4px;">
                            "ВОЙТИ / LOGIN"
                        </button>
                    }.into_view(),
                    Some(name) => view! {
                        <span style="color: #0f0; font-weight: bold;">"В СЕТИ: " {name}</span>
                    }.into_view(),
                }}
            </nav>

            <main style="padding: 60px; text-align: center;">
                {move || if let Some(name) = user_name.get() {
                    view! {
                        <div style="border: 2px solid #0f0; padding: 20px; display: inline-block;">
                            <h1 style="color: #0f0;">"ДОСТУП ОТКРЫТ"</h1>
                            <p>"Добро пожаловать, " {name} "!"</p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div>
                            <h1>"АВТОРИЗАЦИЯ"</h1>
                            <p style="color: #888;">"Нажмите кнопку выше, чтобы войти через официальный сайт F-list."</p>
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
