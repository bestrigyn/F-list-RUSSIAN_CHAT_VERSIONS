use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Теперь максимально просто, чтобы Rust не ругался
    let open_login_window = move |_| {
        let win = window(); // Просто берем окно
        // Используем правильный метод с 3 параметрами для размера окна
        let _ = win.open_with_url_and_target_and_features(
            "https://www.f-list.net/login.php", 
            "_blank", 
            "width=500,height=600"
        );
        set_user_name.set(Some("Авторизация...".to_string()));
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            
            // ШАПКА
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d; box-shadow: 0 2px 10px rgba(0,0,0,0.5);">
                <h2 style="margin: 0; color: #d62d2d; font-weight: 800; letter-spacing: 1px;">"F-LIST RUSSIAN"</h2>
                
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window
                                style="background: #d62d2d; color: white; border: none; padding: 10px 25px; cursor: pointer; border-radius: 5px; font-weight: bold; box-shadow: 0 0 15px rgba(214,45,45,0.3);">
                                "ВОЙТИ ЧЕРЕЗ F-LIST"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 20px; align-items: center;">
                                <span style="color: #4cd137; font-weight: bold;">{name}</span>
                                <button on:click=move |_| set_user_name.set(None)
                                    style="background: #333; color: #aaa; border: none; padding: 5px 15px; cursor: pointer; border-radius: 4px;">
                                    "Выход"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
            </nav>

            // ОСНОВНОЙ БЛОК
            <div style="display: flex; padding: 30px; gap: 25px; max-width: 1200px; margin: 0 auto;">
                <main style="flex: 2; background: #1a1a1a; padding: 25px; border-radius: 12px; border: 1px solid #333; min-height: 450px; text-align: center;">
                    <h3 style="color: #d62d2d;">"Система готова к работе"</h3>
                    <p style="color: #888;">"Нажми на кнопку входа в шапке, чтобы открыть окно авторизации."</p>
                </main>

                // ТВOЙ HTML-ПОМОЩНИК
                <aside style="flex: 1; min-width: 300px; background: #1f1f1f; padding: 20px; border-radius: 12px; border: 1px solid #d62d2d;">
                    <h4 style="margin-top: 0; color: #4cd137; display: flex; align-items: center; gap: 10px;">
                        <span>"🛠️"</span> "ТЕХ-ОТДЕЛ"
                    </h4>
                    <div style="font-size: 0.85rem; line-height: 1.6; color: #bbb;">
                        <p style="background: #252525; padding: 10px; border-radius: 6px; border-left: 3px solid #d62d2d;">
                            "Ошибка E0277 была в лишней команде .as_ref(). В Rust если объект уже существует, не надо спрашивать, есть ли он."
                        </p>
                        <p><b>"План:"</b> " Сейчас билд должен пройти. После входа браузер предложит тебе сохранить пароль."</p>
                    </div>
                </aside>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
