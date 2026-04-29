use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent};

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Создаем замыкание для обработки сообщений от окна логина
    let on_message = {
        let set_user_name = set_user_name.clone();
        Closure::wrap(Box::new(move |ev: MessageEvent| {
            if let Some(data_str) = ev.data().as_string() {
                // Если пришло сообщение с ником, сохраняем его
                if data_str.starts_with("login:") {
                    let nick = data_str.replace("login:", "");
                    set_user_name.set(Some(nick));
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>)
    };

    let open_login_window = move |_| {
        let win = window();
        
        // Устанавливаем слушатель, чтобы поймать ответ от F-list
        let _ = win.add_event_listener_with_callback(
            "message", 
            on_message.as_ref().unchecked_ref()
        );
        on_message.forget(); // Чтобы Rust не удалил обработчик из памяти

        let _ = win.open_with_url_and_target_and_features(
            "https://www.f-list.net/login.php", 
            "login_popup", 
            "width=500,height=600"
        );
        set_user_name.set(Some("Ожидание входа...".to_string()));
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
                {move || if user_name.get().is_none() {
                    view! { <p>"Пожалуйста, авторизуйтесь через всплывающее окно."</p> }.into_view()
                } else {
                    view! { <h1 style="color: #4cd137;">"Вы успешно вошли!"</h1> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
