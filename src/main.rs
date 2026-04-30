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

    // Настоящий слушатель событий
    create_effect(move |_| {
        let window = window();
        let cb = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Пытаемся распарсить данные из JS
            if let Ok(js_data) = e.data().dyn_into::<js_sys::Object>() {
                // Если в объекте есть поле 'user', значит это наш успех
                let user = js_sys::Reflect::get(&js_data, &"user".into())
                    .unwrap_or_default()
                    .as_string();
                
                if let Some(name) = user {
                    set_user_name.set(Some(name));
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let _ = window.add_event_listener_with_callback("message", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let login = move |_| {
        let callback = "https://bestrigyn.github.io/F-list-RUSSIAN_CHAT_VERSIONS/callback.html";
        // Важно: F-list требует именно такой формат для редиректа
        let url = format!("https://www.f-list.net/login.php?redirect={}", js_sys::encode_uri_component(callback));
        
        let _ = window().open_with_url_and_target_and_features(
            &url, "auth", "width=500,height=600"
        );
    };

    view! {
        <div style="background: #000; color: #fff; min-height: 100vh; font-family: 'Segoe UI', sans-serif;">
            <nav style="background: #111; padding: 15px; border-bottom: 2px solid #f00; display: flex; justify-content: space-between;">
                <h2 style="margin:0;">"F-LIST REAL AUTH"</h2>
                {move || match user_name.get() {
                    None => view! { <button on:click=login style="background:#f00; color:#fff; border:none; padding:10px; cursor:pointer;">"ВОЙТИ"</button> }.into_view(),
                    Some(name) => view! { <b style="color:#0f0;">"В СЕТИ: " {name}</b> }.into_view(),
                }}
            </nav>
            <main style="padding: 40px; text-align: center;">
                {move || if let Some(name) = user_name.get() {
                    view! { <h3>"Привет, " {name} "! Теперь всё по-честному."</h3> }.into_view()
                } else {
                    view! { <p>"Нажми войти, чтобы авторизоваться через официальный F-list."</p> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() { mount_to_body(App); }
