use leptos::*;

// Функция для работы с памятью браузера (упрощенная версия)
fn get_storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

#[component]
fn App() -> impl IntoView {
    let initial_user = get_storage()
        .and_then(|s| s.get_item("chat_user").ok().flatten());
    
    let (account, set_account) = create_signal(String::new());
    let (user_name, set_user_name) = create_signal(initial_user);

    let on_login = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name = account.get();
        if !name.is_empty() {
            if let Some(storage) = get_storage() {
                let _ = storage.set_item("chat_user", &name).unwrap();
            }
            set_user_name.set(Some(name));
        }
    };

    view! {
        <div style="background: #1b1b1b; color: #e0e0e0; min-height: 100vh; font-family: sans-serif; display: flex; flex-direction: column;">
            
            // ШАПКА
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; background: #2a2a2a; border-bottom: 2px solid #ff4444;">
                <div style="display: flex; gap: 10px; align-items: center;">
                    {move || match user_name.get() {
                        None => view! {
                            <form on:submit=on_login style="display: flex; gap: 5px;">
                                <input type="text" name="username" placeholder="Логин"
                                    on:input=move |ev| set_account.set(event_target_value(&ev))
                                    prop:value=account
                                    style="background: #333; color: white; border: 1px solid #555; padding: 5px; border-radius: 4px;"/>
                                <button type="submit" 
                                    style="background: #ff4444; color: white; border: none; padding: 5px 15px; cursor: pointer; border-radius: 4px; font-weight: bold;">
                                    "Вход"
                                </button>
                            </form>
                        }.into_view(),
                        Some(name) => view! {
                            <span style="font-weight: bold; color: #44ff44;">"Активен: " {name}</span>
                        }.into_view(),
                    }}
                </div>
                <h2 style="margin: 0; font-size: 1.2rem; color: #ff4444;">"RUSSIAN CHAT"</h2>
            </nav>

            // ОСНОВНОЙ КОНТЕНТ С ПОМОЩНИКОМ
            <div style="display: flex; flex: 1;">
                
                // Левая часть (Чат/Контент)
                <main style="flex: 1; padding: 20px; text-align: center;">
                    {move || if user_name.get().is_some() {
                        view! { <h1 style="color: #44ff44;">"Связь установлена!"</h1> }.into_view()
                    } else {
                        view! { <p>"Ожидание авторизации..."</p> }.into_view()
                    }}
                </main>

                // ПРАВАЯ ЧАСТЬ: HTML-ПОМОЩНИК
                <aside style="width: 300px; background: #252525; border-left: 2px solid #333; padding: 15px; font-size: 0.9rem;">
                    <h3 style="color: #ff4444; border-bottom: 1px solid #444; padding-bottom: 5px;">"🤖 Помощник"</h3>
                    <div style="background: #1a1a1a; padding: 10px; border-radius: 5px; border: 1px inset #333;">
                        <p><b>"Статус:"</b> " Ошибки исправлены"</p>
                        <p><b>"План на завтра:"</b></p>
                        <ul style="padding-left: 20px; color: #bbb;">
                            <li>"Рисуем шапку с драконом"</li>
                            <li>"Подключаем список API"</li>
                            <li>"Настраиваем стили"</li>
                        </ul>
                        <hr style="border: 0; border-top: 1px solid #444;"/>
                        <p style="font-style: italic; color: #888;">
                            "Если видишь красную ошибку в GitHub — проверь Cargo.toml. Rust очень строгий!"
                        </p>
                    </div>
                </aside>

            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
