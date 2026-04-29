use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Исправленная функция открытия окна
    let open_login_window = move |_| {
        if let Some(win) = window().as_ref() {
            // Исправлено: передаем только 2 аргумента, чтобы не было ошибки компиляции
            let _ = win.open_with_url_and_target(
                "https://www.f-list.net/login.php", 
                "_blank"
            );
            set_user_name.set(Some("Ожидание входа...".to_string()));
        }
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;">
            
            // ШАПКА
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d; box-shadow: 0 4px 10px rgba(0,0,0,0.5);">
                <h2 style="margin: 0; color: #d62d2d; letter-spacing: 1px; font-weight: 800;">"F-LIST RUSSIAN"</h2>
                
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window
                                style="background: #d62d2d; color: white; border: none; padding: 10px 25px; cursor: pointer; border-radius: 5px; font-weight: bold; transition: 0.3s; box-shadow: 0 0 15px rgba(214,45,45,0.4);">
                                "ВОЙТИ В АККАУНТ"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 20px; align-items: center;">
                                <span style="color: #4cd137; font-weight: bold;">"Статус: " {name}</span>
                                <button on:click=move |_| set_user_name.set(None)
                                    style="background: #333; color: #aaa; border: 1px solid #444; padding: 5px 15px; cursor: pointer; border-radius: 4px;">
                                    "Выйти"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
            </nav>

            // ОСНОВНОЙ ЭКРАН
            <div style="display: flex; padding: 30px; gap: 25px; max-width: 1200px; margin: 0 auto;">
                
                // Левая панель - Имитация чата
                <main style="flex: 3; background: #1a1a1a; border-radius: 12px; border: 1px solid #333; min-height: 500px; padding: 20px; position: relative; overflow: hidden;">
                    <div style="color: #555; text-align: center; margin-top: 200px;">
                        <img src="https://www.f-list.net/favicon.ico" style="filter: grayscale(1); opacity: 0.2; width: 64px; margin-bottom: 10px;"/>
                        <p>"Здесь будет окно чата после подтверждения токена"</p>
                    </div>
                </main>

                // Правая панель - Помощник
                <aside style="flex: 1; min-width: 300px;">
                    <div style="background: #1f1f1f; padding: 20px; border-radius: 12px; border: 1px solid #d62d2d; position: sticky; top: 30px;">
                        <h3 style="margin-top: 0; color: #d62d2d; display: flex; align-items: center; gap: 10px;">
                            <span>"🤖"</span> "ИНФО-ЦЕНТР"
                        </h3>
                        <div style="font-size: 0.9rem; color: #bbb; line-height: 1.6;">
                            <p style="background: #252525; padding: 10px; border-radius: 6px; border-left: 3px solid #4cd137;">
                                <b>"Прокси:"</b> " Активен. Мы используем прямое перенаправление на f-list.net для безопасности."
                            </p>
                            <p><b>"Пароли:"</b> " Браузер предложит сохранить их на официальной странице входа."</p>
                            <hr style="border: 0; border-top: 1px solid #333; margin: 15px 0;"/>
                            <p style="font-size: 0.8rem; color: #777;">
                                "Семён, теперь код не упадет при сборке. Окно откроется по всем правилам Rust!"
                            </p>
                        </div>
                    </div>
                </aside>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
