use leptos::*;

#[component]
fn App() -> impl IntoView {
    // Состояние авторизации: None — не вошел, Some(String) — ник игрока
    let (user_name, set_user_name) = create_signal(None::<String>);

    view! {
        <div style="background: #1b1b1b; color: #e0e0e0; min-height: 100vh; font-family: Arial, sans-serif;">
            
            // ВЕРХНЯЯ ПАНЕЛЬ (Header)
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; background: #2a2a2a; border-bottom: 2px solid #ff4444;">
                
                // Левая часть: Вход или Ник
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button 
                                on:click=move |_| {
                                    // Тут позже будет логика вызова API для проверки тикета
                                    set_user_name.set(Some("Semyon".to_string())); 
                                }
                                style="background: #444; color: white; border: 1px solid #666; padding: 5px 15px; cursor: pointer; border-radius: 3px;">
                                "Вход"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <span style="font-weight: bold; color: #ff4444;">{name}</span>
                        }.into_view(),
                    }}
                </div>

                // Центр: Твой логотип
                <div style="text-align: center;">
                    <h2 style="margin: 0; font-size: 1.2rem; letter-spacing: 1px;">"RUSSIAN CHAT VERSIONS"</h2>
                </div>

                // Правая часть: Кнопка Выход (появляется только если залогинен)
                <div style="width: 80px; text-align: right;">
                    {move || user_name.get().is_some().then(|| view! {
                        <button 
                            on:click=move |_| set_user_name.set(None)
                            style="background: transparent; color: #888; border: none; cursor: pointer; text-decoration: underline;">
                            "Выход"
                        </button>
                    })}
                </div>
            </nav>

            // ОСНОВНОЙ КОНТЕНТ (Сетка как на оригинале)
            <main style="display: flex; height: calc(100vh - 50px);">
                
                // Слева: Список каналов
                <aside style="width: 200px; background: #252525; border-right: 1px solid #333; padding: 10px;">
                    <p style="font-size: 0.8rem; color: #666;">"КАНАЛЫ"</p>
                </aside>

                // Центр: Окно чата
                <section style="flex-grow: 1; padding: 20px; background: #1b1b1b; display: flex; flex-direction: column;">
                    <div style="flex-grow: 1; border: 1px solid #333; background: #141414; margin-bottom: 10px; border-radius: 5px;">
                        // Тут будут сообщения
                    </div>
                    <input type="text" placeholder="Написать сообщение..." 
                        style="width: 100%; padding: 10px; background: #2a2a2a; border: 1px solid #444; color: white; border-radius: 5px;"/>
                </section>

                // Справа: Список персонажей
                <aside style="width: 200px; background: #252525; border-left: 1px solid #333; padding: 10px;">
                    <p style="font-size: 0.8rem; color: #666;">"В СЕТИ"</p>
                </aside>

            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
