use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ApiResponse {
    character: Option<String>,
    ticket: Option<String>,
    error: Option<String>,
}

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);
    let (status_msg, set_status_msg) = create_signal("Готов к авторизации".to_string());

    // Взрослая функция: запрашивает данные через API
    let fetch_user_data = create_action(move |_: &()| async move {
        let client = reqwest::Client::new();
        // Запрос к API F-list (имитация вызова получения данных по сессии)
        // В реальном приложении тут должен быть твой API-ключ или тикет
        let res = client
            .get("https://www.f-list.net/json/get_ticket.php")
            .send()
            .await;

        match res {
            Ok(resp) => {
                if let Ok(data) = resp.json::<ApiResponse>().await {
                    if let Some(name) = data.character {
                        set_user_name.set(Some(name));
                        set_status_msg.set("Вход выполнен успешно!".into());
                    }
                }
            }
            Err(_) => set_status_msg.set("Ошибка API: Не удалось получить данные".into()),
        }
    });

    let open_login_window = move |_| {
        let _ = window().open_with_url_and_target_and_features(
            "https://www.f-list.net/login.php",
            "f_list_login",
            "width=500,height=600",
        );
        set_status_msg.set("Ожидание подтверждения от F-list...".into());
        
        // Запускаем проверку API через пару секунд после открытия окна
        set_timeout(move || {
            fetch_user_data.dispatch(());
        }, std::time::Duration::from_secs(3));
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d;">
                <h2 style="margin: 0; color: #d62d2d; font-weight: 800;">"F-LIST RUSSIAN"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window style="background: #d62d2d; color: white; border: none; padding: 10px 25px; cursor: pointer; border-radius: 5px;">
                                "АВТОРИЗАЦИЯ"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <span style="color: #4cd137; font-weight: bold;">"Аккаунт: " {name}</span>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="max-width: 600px; margin: 100px auto; text-align: center; background: #1a1a1a; padding: 40px; border-radius: 20px; border: 1px solid #333;">
                <h3 style="color: #666;">"Статус системы:"</h3>
                <div style="font-size: 1.2rem; margin: 20px 0; color: #d62d2d;">
                    {move || status_msg.get()}
                </div>
                
                {move || if fetch_user_data.pending().get() {
                    view! { <p style="color: #aaa;">"Связываюсь с серверами F-list..."</p> }.into_view()
                } else {
                    view! { <p>"Используется официальный API протокол."</p> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
