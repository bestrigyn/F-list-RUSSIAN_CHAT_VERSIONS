use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);
    let (is_waiting, set_is_waiting) = create_signal(false);

    let open_login_window = move |_| {
        set_is_waiting.set(true);
        let _ = window().open_with_url_and_target_and_features(
            "https://www.f-list.net/login.php",
            "f_list_login",
            "width=500,height=600",
        );
    };

    // Функция, которую ты нажмешь, когда увидишь, что в маленьком окне вход выполнен
    let confirm_login = move |_| {
        set_is_waiting.set(false);
        // Тут мы имитируем получение ника (в будущем заменим на реальный токен)
        set_user_name.set(Some("Semen_Shevekhov".to_string())); 
    };

    view! {
        <div style="background: #121212; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="display: flex; justify-content: space-between; align-items: center; padding: 15px 30px; background: #1f1f1f; border-bottom: 2px solid #d62d2d;">
                <h2 style="margin: 0; color: #d62d2d;">"F-LIST RUSSIAN"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! {
                            <button on:click=open_login_window style="background: #d62d2d; color: white; border: none; padding: 10px 20px; cursor: pointer; border-radius: 5px;">
                                "ВОЙТИ"
                            </button>
                        }.into_view(),
                        Some(name) => view! {
                            <span style="color: #4cd137; font-weight: bold;">"Привет, " {name} "!"</span>
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="padding: 40px; text-align: center;">
                {move || if is_waiting.get() && user_name.get().is_none() {
                    view! {
                        <div style="background: #1f1f1f; padding: 20px; border: 1px solid #4cd137; border-radius: 10px;">
                            <p>"Вы залогинились в маленьком окне?"</p>
                            <button on:click=confirm_login style="background: #4cd137; color: #000; border: none; padding: 10px 20px; cursor: pointer; font-weight: bold; border-radius: 5px;">
                                "ДА, Я ВОШЕЛ!"
                            </button>
                        </div>
                    }.into_view()
                } else if user_name.get().is_some() {
                    view! { <h1 style="color: #4cd137;">"Добро пожаловать в русский чат!"</h1> }.into_view()
                } else {
                    view! { <p>"Нажмите кнопку входа выше."</p> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
