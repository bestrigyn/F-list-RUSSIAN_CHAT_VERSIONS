use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (account, set_account) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (user_name, set_user_name) = create_signal(None::<String>);

    view! {
        <div style="background: #1b1b1b; color: #e0e0e0; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; background: #2a2a2a; border-bottom: 2px solid #ff4444;">
                
                // Слева: Логин или Имя
                <div style="display: flex; gap: 10px; align-items: center;">
                    {move || match user_name.get() {
                        None => view! {
                            <>
                                <input type="text" placeholder="Логин" 
                                    on:input=move |ev| set_account.set(event_target_value(&ev))
                                    prop:value=account
                                    style="background: #333; color: white; border: 1px solid #555; padding: 5px; border-radius: 4px;"/>
                                <input type="password" placeholder="Пароль" 
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                    prop:value=password
                                    style="background: #333; color: white; border: 1px solid #555; padding: 5px; border-radius: 4px;"/>
                                <button 
                                    on:click=move |_| {
                                        if !account.get().is_empty() {
                                            set_user_name.set(Some(account.get()));
                                        }
                                    }
                                    style="background: #ff4444; color: white; border: none; padding: 5px 15px; cursor: pointer; border-radius: 4px; font-weight: bold;">
                                    "Войти"
                                </button>
                            </>
                        }.into_view(),
                        Some(name) => view! {
                            <div style="display: flex; gap: 15px; align-items: center;">
                                <span style="font-weight: bold; color: #ff4444;">{name}</span>
                                <button 
                                    on:click=move |_| set_user_name.set(None)
                                    style="background: transparent; color: #888; border: none; cursor: pointer; text-decoration: underline; font-size: 0.9rem;">
                                    "Выход"
                                </button>
                            </div>
                        }.into_view(),
                    }}
                </div>

                // Центр: Лого
                <h2 style="margin: 0; font-size: 1.4rem; letter-spacing: 1px;">"RUSSIAN CHAT VERSIONS"</h2>
                
                <div style="width: 200px;"></div> // Заглушка для симметрии
            </nav>

            <main style="padding: 40px; text-align: center;">
                {move || if user_name.get().is_none() {
                    view! { <p style="color: #666;">"Введите данные для входа в систему..."</p> }.into_view()
                } else {
                    view! { <p style="color: #44ff44;">"Доступ разрешен. Модули связи готовы к работе."</p> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
