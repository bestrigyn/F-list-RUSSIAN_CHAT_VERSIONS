use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Исправленная функция: теперь аргументов ровно столько, сколько нужно Rust
    let open_login_window = move |_| {
        if let Some(win) = window().as_ref() {
            let _ = win.open_with_url_and_target_and_features(
                "https://www.f-list.net/login.php", 
                "_blank", 
                "width=500,height=600"
            );
            set_user_name.set(Some("Авторизация...".to_string()));
        }
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            
            // ШАПКА
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d;">
                <h2 style="margin: 0; color: #d62d2d; font-weight: 800;">"F-LIST RUSSIAN"</h2>
                
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window
                                style="background: #d62d2d; color: white; border: none; padding: 10px 25px; cursor: pointer; border-radius: 5px; font-weight: bold; box-shadow: 0 0 15px rgba(214,45,45,0.4);">
                                "ВОЙТИ В АККАУНТ"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 20px; align-items: center;">
                                <span style="color: #4cd137; font-weight: bold;">{name}</span>
                                <button on:click=move |_| set_user_name.set(None)
                                    style="background: #333; color: #aaa; border: none; padding: 5px 15px; cursor: pointer;">
                                    "Выход"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
            </nav>

            // ТВОЙ HTML-ПОМОЩНИК
            <div style="display: flex; padding: 30px; gap: 20px;">
                <main style="flex: 2; background: #1a1a1a; padding: 20px; border-radius: 10px; border: 1px solid #333; min-height: 400px;">
                    <h3 style="color: #d62d2d;">"Статус подключения"</h3>
                    <p>"Нажми кнопку входа. Откроется официальное окно F-list."</p>
                    <p style="color: #666;">"Браузер сам предложит тебе сохранить логин и пароль в этом окне."</p>
                </main>

                <aside style="flex: 1; background: #1f1f1f; padding: 20px; border-radius: 10px; border: 1px solid #d62d2d;">
                    <h4 style="margin-top: 0; color: #4cd137;">"🤖 Помощник"</h4>
                    <p style="font-size: 0.85rem; line-height: 1.5;">
                        "Семён, я исправил ошибку с аргументами (E0061). Теперь Rust понимает, что мы хотим открыть окно с заданным размером (500x600)."
                    </p>
                    <hr style="border: 0; border-top: 1px solid #333; margin: 15px 0;"/>
                    <p style="font-size: 0.8rem; color: #777;">"Жми коммит и пуш — билд должен позеленеть!"</p>
                </aside>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
