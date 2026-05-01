use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Слушаем данные от нашей формы
    create_effect(move |_| {
        let window = window();
        let cb = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Пытаемся достать объект из события
            if let Ok(js_obj) = e.data().dyn_into::<js_sys::Object>() {
                // Извлекаем поле 'account'
                if let Ok(val) = js_sys::Reflect::get(&js_obj, &"account".into()) {
                    if let Some(name) = val.as_string() {
                        set_user_name.set(Some(name));
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let _ = window.add_event_listener_with_callback("message", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let open_auth = move |_| {
        let _ = window().open_with_url_and_target_and_features(
            "login.html", // Открываем нашу простую форму
            "auth_window",
            "width=400,height=300"
        );
    };

    view! {
        <div style="background: #000; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="background: #1a1a1a; padding: 15px; border-bottom: 2px solid #d62d2d; display: flex; justify-content: space-between;">
                <h2 style="margin: 0; color: #d62d2d;">"F-LIST RUSSIAN"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! { 
                            <button on:click=open_auth style="background: #d62d2d; color: white; border: none; padding: 10px 20px; cursor: pointer; border-radius: 5px;">
                                "ВХОД"
                            </button> 
                        }.into_view(),
                        Some(name) => view! { 
                            <b style="color: #4cd137;">"В СЕТИ: " {name}</b> 
                        }.into_view(),
                    }}
                </div>
            </nav>

            <main style="padding: 50px; text-align: center;">
                {move || if let Some(name) = user_name.get() {
                    view! { <h3>"Привет, " {name} "! Теперь связь с Rust работает реально."</h3> }.into_view()
                } else {
                    view! { <p>"Нажми кнопку входа для авторизации."</p> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
