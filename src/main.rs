use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Функция, которая открывает ОФИЦИАЛЬНОЕ окно входа
    let open_login_window = move |_| {
        let _ = window().open_with_url_and_target(
            "https://www.f-list.net/login.php", 
            "_blank", 
            "width=500,height=600"
        );
        // Временно ставим заглушку, пока не настроим получение тикета
        set_user_name.set(Some("Авторизация...".to_string()));
    };

    view! {
        <div style="background: #1b1b1b; color: #e0e0e0; min-height: 100vh; font-family: sans-serif;">
            
            // ШАПКА
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; background: #2a2a2a; border-bottom: 2px solid #ff4444;">
                <div style="display: flex; gap: 10px; align-items: center;">
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window
                                style="background: #ff4444; color: white; border: none; padding: 8px 20px; cursor: pointer; border-radius: 4px; font-weight: bold; box-shadow: 0 0 10px rgba(255,68,68,0.3);">
                                "ВОЙТИ ЧЕРЕЗ F-LIST"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 15px; align-items: center;">
                                <span style="font-weight: bold; color: #44ff44; text-shadow: 0 0 5px #000;">{name}</span>
                                <button on:click=move |_| set_user_name.set(None)
                                    style="background: transparent; color: #888; border: none; cursor: pointer; text-decoration: underline;">
                                    "Выход"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
                <h2 style="margin: 0; font-size: 1.2rem; color: #ff4444; letter-spacing: 2px;">"F-LIST RUSSIAN"</h2>
            </nav>

            // ТВОЙ ПОМОЩНИК (HTML ВНУТРИ RUST)
            <div style="display: flex; padding: 20px; gap: 20px;">
                <main style="flex: 2; background: #222; padding: 20px; border-radius: 8px; border: 1px solid #333;">
                    <h3 style="color: #ff4444;">"Консоль управления"</h3>
                    <p>"Нажми кнопку входа, чтобы браузер открыл официальный сайт."</p>
                    <p style="color: #888; font-size: 0.9rem;">
                        "Это гарантирует, что твои данные в безопасности и сохраняются в браузере."
                    </p>
                </main>

                <aside style="flex: 1; background: #252525; padding: 15px; border-radius: 8px; border: 1px solid #444;">
                    <h4 style="margin-top: 0; color: #44ff44;">"🤖 Статус Разработки"</h4>
                    <div style="font-size: 0.85rem; line-height: 1.4;">
                        <div style="margin-bottom: 8px;"><b>"Версия Rust:"</b> " Исправлена"</div>
                        <div style="margin-bottom: 8px;"><b>"Ошибки версий:"</b> " Устранены"</div>
                        <div style="color: #aaa;">"Завтра мы добавим автоматическое закрытие окна после логина."</div>
                    </div>
                </aside>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
