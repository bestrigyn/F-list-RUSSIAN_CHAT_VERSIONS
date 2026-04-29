use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (account, set_account) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (user_name, set_user_name) = create_signal(None::<String>);

    view! {
        <div style="background: #1b1b1b; color: #e0e0e0; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; padding: 10px; background: #2a2a2a; border-bottom: 2px solid #ff4444;">
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <div style="display: flex; gap: 5px;">
                                <input type="text" placeholder="Логин" 
                                    on:input=move |ev| set_account.set(event_value(&ev))
                                    style="background: #333; color: white; border: 1px solid #555;"/>
                                <input type="password" placeholder="Пароль" 
                                    on:input=move |ev| set_password.set(event_value(&ev))
                                    style="background: #333; color: white; border: 1px solid #555;"/>
                                <button on:click=move |_| set_user_name.set(Some(account.get()))
                                    style="background: #ff4444; border: none; color: white; cursor: pointer;">
                                    "Войти"
                                </button>
                            </div>
                        }.into_view(),
                        Some(name) => view! {
                            <span>"Привет, " {name} "!"</span>
                        }.into_view(),
                    }}
                </div>
                <h2 style="margin: 0;">"RUSSIAN CHAT"</h2>
                <div>
                    {move || user_name.get().is_some().then(|| view! {
                        <button on:click=move |_| set_user_name.set(None)>"Выход"</button>
                    })}
                </div>
            </nav>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
