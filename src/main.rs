use leptos::*;
use web_sys::Storage;

// Попытка получить доступ к локальному хранилищу
fn get_storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

#[component]
fn App() -> impl IntoView {
    // Проверяем память при загрузке
    let initial_user = get_storage()
        .and_then(|s| s.get_item("chat_user").ok().flatten());
    
    let (account, set_account) = create_signal(String::new());
    let (user_name, set_user_name) = create_signal(initial_user);

    let on_login = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name = account.get();
        if !name.is_empty() {
            if let Some(storage) = get_storage() {
                let _ = storage.set_item("chat_user", &name);
            }
            set_user_name.set(Some(name));
        }
    };

    let on_logout = move |_| {
        if let Some(storage) = get_storage() {
            let _ = storage.remove_item("chat_user");
        }
        set_user_name.set(None);
    };

    view! {
        <div style="background: #1b1b1b; color: #e0e0e0; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; background: #2a2a2a; border-bottom: 2px solid #ff4444;">
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <form on:submit=on_login style="display: flex; gap: 5px;">
                                <input type="text" name="username" placeholder="Логин"
                                    on:input=move |ev| set_account.set(event_target_value(&ev))
                                    prop:value=account
                                    style="background: #333; color: white; border: 1px solid #555; padding: 5px;"/>
                                <input type="password" name="password" placeholder="Пароль"
                                    style="background: #333; color: white; border: 1px solid #555; padding: 5px;"/>
                                <button type="submit" style="background: #ff4444; color: white; border: none; padding: 5px 15px; cursor: pointer;">
                                    "Войти"
                                </button>
                            </form>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 15px; align-items: center;">
                                <span style="font-weight: bold; color: #ff4444;">{name}</span>
                                <button on:click=on_logout style="background: transparent; color: #888; border: none; cursor: pointer; text-decoration: underline;">
                                    "Выход"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>
                <h2 style="margin: 0; font-size: 1.2rem;">"RUSSIAN CHAT VERSIONS"</h2>
                <div style="width: 100px;"></div>
            </nav>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
