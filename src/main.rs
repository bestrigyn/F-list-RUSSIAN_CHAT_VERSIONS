// Добавь эти сигналы в начало функции App
let (account, set_account) = create_signal(String::new());
let (password, set_password) = create_signal(String::new());
let (is_loading, set_is_loading) = create_signal(false);

// В Header, там где была кнопка "Вход", замени на это:
{move || match user_name.get() {
    None => view! {
        <div style="display: flex; gap: 5px;">
            <input type="text" placeholder="Логин" 
                on:input=move |ev| set_account.set(event_value(&ev))
                style="background: #333; border: 1px solid #555; color: white; padding: 2px 5px; width: 100px;"/>
            <input type="password" placeholder="Пароль" 
                on:input=move |ev| set_password.set(event_value(&ev))
                style="background: #333; border: 1px solid #555; color: white; padding: 2px 5px; width: 100px;"/>
            <button 
                on:click=move |_| {
                    set_is_loading.set(true);
                    // СЮДА мы завтра пропишем fetch запрос к API f-list
                    log!("Попытка входа для: {}", account.get());
                }
                style="background: #ff4444; color: white; border: none; padding: 2px 10px; cursor: pointer;">
                {move || if is_loading.get() { "..." } else { "Войти" }}
            </button>
        </div>
    }.into_view(),
    Some(name) => view! {
        <span style="font-weight: bold; color: #ff4444;">{name}</span>
    }.into_view(),
}}
