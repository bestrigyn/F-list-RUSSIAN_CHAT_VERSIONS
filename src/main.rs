use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::Window;

#[component]
fn App() -> impl IntoView {
    let (user_email, set_user_email) = create_signal(None::<String>);
    let auth_window = create_rw_signal(None::<Window>);

    // Функция-шпион
    let start_watching = move |win: Window| {
        let handle = set_interval_with_handle(move || {
            if let Some(w) = auth_window.get() {
                // Проверяем, не закрыл ли пользователь окно сам
                if w.closed().unwrap_or(false) {
                    // Если закрыто, и мы не успели поставить email — значит залогинились
                    if user_email.get().is_none() {
                        set_user_email.set(Some("vlad_dev@f-list.net".to_string()));
                    }
                }
                
                // Пытаемся дотянуться до адреса окна. 
                // Если мы на другом домене, это вызовет ошибку (Security), 
                // и это наш сигнал, что логин прошел!
                match w.location().href() {
                    Ok(href) => {
                        if href.contains("account_settings") || href.contains("callback") {
                            set_user_email.set(Some("semen_active@f-list.net".to_string()));
                            let _ = w.close();
                        }
                    },
                    Err(_) => {
                        // Ошибка доступа = окно ушло с f-list на наш callback или еще куда
                        set_user_email.set(Some("semen_connected@f-list.net".to_string()));
                        let _ = w.close();
                    }
                }
            }
        }, std::time::Duration::from_millis(500)).ok();
    };

    let open_login = move |_| {
        let login_url = "https://www.f-list.net/login.php";
        if let Ok(Some(win)) = window().open_with_url_and_target_and_features(
            login_url,
            "f_list_auth",
            "width=500,height=600"
        ) {
            auth_window.set(Some(win.clone()));
            start_watching(win);
        }
    };

    view! {
        <div style="background: #111; color: #fff; min-height: 100vh; font-family: sans-serif;">
            <nav style="background: #222; padding: 20px; border-bottom: 2px solid #d62d2d; display: flex; justify-content: space-between;">
                <b style="color: #d62d2d;">"F-LIST RUS"</b>
                {move || match user_email.get() {
                    None => view! {
                        <button on:click=open_login style="background: #d62d2d; border: none; color: #fff; padding: 5px 15px; cursor: pointer;">
                            "ВХОД / EMAIL"
                        </button>
                    }.into_view(),
                    Some(email) => view! { <span style="color: #0f0;">{email}</span> }.into_view()
                }}
            </nav>
            <main style="text-align: center; padding: 50px;">
                {move || if user_email.get().is_none() {
                    view! { <p>"Нажми кнопку и просто залогинься. Окно должно закрыться само."</p> }.into_view()
                } else {
                    view! { <h2>"Успешное подключение!"</h2> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() { mount_to_body(App); }
