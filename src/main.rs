use leptos::*;
use web_sys::storage::Storage;

// Функция для работы с локальным хранилищем браузера
fn get_storage() -> Option<Storage> {
    window().local_storage().ok().flatten()
}

#[component]
fn App() -> impl IntoView {
    // При запуске пытаемся достать имя из памяти браузера
    let initial_user = get_storage()
        .and_then(|s| s.get_item("chat_user").ok().flatten());
    
    let (account, set_account) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (user_name, set_user_name) = create_signal(initial_user);

    // Функция входа
    let on_login = move |ev: ev::SubmitEvent| {
        ev.prevent_default(); // Чтобы страница не перезагружалась при отправке формы
        if !account.get().is_empty() {
            let name = account.get();
            // Сохраняем в браузер, чтобы не вылетало при обновлении
            if let Some(storage) = get_storage() {
                let _ = storage.set_item("chat_user", &name);
            }
            set_user_name.set(Some(name));
        }
    };

    // Функция выхода
    let on_logout = move |_| {
        if let Some(storage) = get_storage() {
            let _ = storage.remove_item("chat_user");
        }
        set_user_name.set(None);
    };

    view! {
        <div style="background: #1b1b1b; color: #e0e0e0; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; background: #2a2a2a; border-bottom: 2px solid #ff4444;">
                
                <div style="display: flex; gap: 10px; align-items: center;">
                    {move || match user_name.get() {
                        None => view! {
                            // Оборачиваем в <form>, чтобы браузер предложил "Запомнить пароль"
                            <form on:submit=on_login style="display: flex; gap: 5px;">
                                <input type="text" 
                                    name="username" // Важно для автозаполнения
                                    placeholder="Логин" 
                                    on:input=move |ev| set_account.set(event_target_value(&ev))
                                    prop:value=account
                                    style="background: #333; color: white; border: 1px solid #555; padding: 5px; border-radius: 4px;"/>
                                <input type="password" 
                                    name="password" // Важно для автозаполнения
                                    placeholder="Пароль" 
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                    prop:value=password
                                    style="background: #333; color: white; border: 1px solid #555; padding: 5px; border-radius: 4px;"/>
                                <button type="submit"
                                    style="background: #ff4444; color: white; border: none; padding: 5px 15px; cursor: pointer; border-radius: 4px; font-weight: bold;">
                                    "Войти"
                                </button>
                            </form>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 15px; align-items: center;">
                                <span style="font-weight: bold; color: #ff4444;">{name}</span>
                                <button on:click=on_logout
                                    style="background: transparent; color: #888; border: none; cursor: pointer; text-decoration: underline; font-size: 0.9rem;">
                                    "Выход"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>

                <h2 style="margin: 0; font-size: 1.2rem;">"RUSSIAN CHAT VERSIONS"</h2>
                <div style="width: 150px;"></div>
            </nav>

            <main style="padding: 20px; text-align: center;">
                {move || if user_name.get().is_some() {
                    view! { <p style="color: #44ff44;">"Авторизация сохранена в браузере."</p> }.into_view()
                } else {
                    view! { <p style="color: #666;">"Ожидание входа..."</p> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
