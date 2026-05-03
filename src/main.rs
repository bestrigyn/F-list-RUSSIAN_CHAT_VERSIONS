use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    create_effect(move |_| {
        let window = window();
        let cb = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(js_obj) = e.data().dyn_into::<js_sys::Object>() {
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
        // Открываем login.html. На GitHub Pages путь будет относительным.
        let _ = window().open_with_url_and_target_and_features(
            "login.html", 
            "auth_window",
            "width=400,height=350"
        );
    };

    view! {
        <div style="background: #000; color: #eee; min-height: 100vh; font-family: sans-serif; display: flex; flex-direction: column;">
            <nav style="background: #1a1a1a; padding: 15px; border-bottom: 2px solid #d62d2d; display: flex; justify-content: space-between; align-items: center;">
                <h2 style="margin: 0; color: #d62d2d; font-weight: 800;">"F-LIST RUSSIAN"</h2>
                <button on:click=open_auth style="background: #d62d2d; color: white; border: none; padding: 10px 25px; cursor: pointer; border-radius: 5px; font-weight: bold; font-size: 16px;">
                    {move || user_name.get().unwrap_or_else(|| "ВХОД".to_string())}
                </button>
            </nav>

            <main style="flex-grow: 1; display: flex; align-items: center; justify-content: center; text-align: center;">
                {move || if let Some(name) = user_name.get() {
                    view! { 
                        <div style="padding: 40px; border: 2px solid #4cd137; border-radius: 15px; background: #111;">
                            <h1 style="color: #4cd137; margin: 0;">"ДОСТУП РАЗРЕШЕН"</h1>
                            <p style="font-size: 1.2em;">"Добро пожаловать, " {name} "!"</p>
                        </div> 
                    }.into_view()
                } else {
                    view! { 
                        <div style="color: #888;">
                            <h1>"Ожидание входа..."</h1>
                            <p>"Нажмите кнопку в правом верхнем углу"</p>
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
