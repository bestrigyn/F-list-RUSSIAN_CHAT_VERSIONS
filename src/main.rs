use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);
    let (is_loading, set_is_loading) = create_signal(false);

    // Эта функция будет крутиться в фоне и искать признаки логина
    let check_login_status = move || {
        set_timeout(move || {
            if user_name.get().is_none() && is_loading.get() {
                // Если мы всё ещё ждём, пробуем имитировать успех для теста интерфейса
                // В реальности тут должен сработать редирект на твой URL
                let window = window();
                let search = window.location().search().unwrap_or_default();
                
                if search.contains("account=") || search.contains("ticket=") {
                    set_user_name.set(Some("Semen_NTE".to_string()));
                    set_is_loading.set(false);
                }
            }
        }, std::time::Duration::from_secs(2));
    };

    let open_login_window = move |_| {
        set_is_loading.set(true);
        // Пытаемся заставить F-list вернуть нас домой после входа
        let redirect_url = "https://bestrigyn.github.io/F-list-RUSSIAN_CHAT_VERSIONS/";
        let login_url = format!(
            "https://www.f-list.net/login.php?redirect={}",
            js_sys::encode_uri_component(redirect_url)
        );

        let _ = window().open_with_url_and_target_and_features(
            &login_url,
            "f_list_login",
            "width=500,height=600",
        );
        
        check_login_status();
    };

    view! {
        <div style="background: #000; color: #fff; min-height: 100vh; font-family: 'Segoe UI', sans-serif;">
            <nav style="background: #1a1a1a; padding: 20px; border-bottom: 3px solid #f00; display: flex; justify-content: space-between; align-items: center;">
                <h1 style="margin: 0; font-style: italic;">"F-LIST RUSSIAN CHAT"</h1>
                
                {move || match user_name.get() {
                    None => view! {
                        <button on:click=open_login_window 
                            style="background: #f00; color: #fff; border: none; padding: 10px 30px; font-weight: bold; cursor: pointer; border-radius: 5px;">
                            {move || if is_loading.get() { "ПРОВЕРКА..." } else { "ВОЙТИ В АККАУНТ" }}
                        </button>
                    }.into_view(),
                    Some(name) => view! {
                        <span style="color: #0f0; font-weight: bold;">"ОНЛАЙН: " {name}</span>
                    }.into_view(),
                }}
            </nav>

            <main style="padding: 50px; text-align: center;">
                {move || if user_name.get().is_none() {
                    view! {
                        <div style="border: 1px solid #333; padding: 30px; background: #111; border-radius: 10px;">
                            <h2>"ТРЕБУЕТСЯ АВТОРИЗАЦИЯ"</h2>
                            <p style="color: #666;">"После входа в маленькое окно, ваш ник появится здесь автоматически."</p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div style="color: #0f0;">
                            <h2>"ДОСТУП РАЗРЕШЕН"</h2>
                            <p>"Добро пожаловать в русскую зону, Семён."</p>
                        </div>
                    }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
