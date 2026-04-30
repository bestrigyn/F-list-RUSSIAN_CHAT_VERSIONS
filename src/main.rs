use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[component]
fn App() -> impl IntoView {
    let (user_email, set_user_email) = create_signal(None::<String>);

    // Слушатель сообщений от нашего HTML-помощника
    create_effect(move |_| {
        let window = window();
        
        let cb = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Some(data) = e.data().as_string() {
                if data == "f_list_login_success" {
                    // Как только получили сигнал — ставим Email (заглушку)
                    set_user_email.set(Some("semen_dev@f-list.net".to_string()));
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let _ = window.add_event_listener_with_callback("message", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let open_login_window = move |_| {
        // Указываем F-list вернуть нас на наш НОВЫЙ файл помощник
        let callback_url = "https://bestrigyn.github.io/F-list-RUSSIAN_CHAT_VERSIONS/callback.html";
        let encoded_callback = js_sys::encode_uri_component(callback_url);
        
        let login_url = format!(
            "https://www.f-list.net/login.php?redirect={}",
            String::from(encoded_callback)
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
                            <span style="color: #4cd137; font-weight: bold;">"Аккаунт: " {email}</span>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="padding: 50px; text-align: center;">
                {move || if user_email.get().is_none() {
                    view! { 
                        <div>
                            <h1>"Система авторизации"</h1>
                            <p>"Используем HTML-помощник для обхода блокировки F-list."</p>
                        </div>
                    }.into_view()
                } else {
                    view! { 
                        <h1 style="color: #4cd137;">"Связь установлена!"</h1> 
                    }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
