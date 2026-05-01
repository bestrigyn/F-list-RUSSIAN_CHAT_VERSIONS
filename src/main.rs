use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(None::<String>);

    // Слушатель для получения данных из окна входа
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
        // Открываем чистое окно
        if let Ok(Some(auth_win)) = window().open_with_url_and_target_and_features(
            "about:blank", 
            "auth_window",
            "width=400,height=350"
        ) {
            // Сами пишем HTML прямо внутрь открытого окна
            let document = auth_win.document().unwrap();
            let body = document.body().unwrap();
            
            body.set_inner_html(r#"
                <div style="background:#121212;color:white;font-family:sans-serif;padding:20px;text-align:center;height:100vh;">
                    <h3 style="color:#d62d2d;">F-LIST LOGIN</h3>
                    <p>Введите ваш ник:</p>
                    <input type="text" id="user" style="width:80%;padding:10px;margin-bottom:10px;background:#222;border:1px solid #444;color:white;">
                    <br>
                    <button id="btn" style="background:#d62d2d;color:white;border:none;padding:10px 20px;cursor:pointer;font-weight:bold;">ВОЙТИ</button>
                    <script>
                        document.getElementById('btn').onclick = function() {
                            const val = document.getElementById('user').value;
                            if(val) {
                                window.opener.postMessage({account: val}, "*");
                                window.close();
                            }
                        };
                    </script>
                </div>
            "#);
        }
    };

    view! {
        <div style="background: #000; color: #eee; min-height: 100vh; font-family: sans-serif;">
            <nav style="background: #1a1a1a; padding: 15px; border-bottom: 2px solid #d62d2d; display: flex; justify-content: space-between; align-items: center;">
                <h2 style="margin: 0; color: #d62d2d;">"F-LIST RUSSIAN"</h2>
                <div>
                    {move || match user_name.get() {
                        None => view! { 
                            <button on:click=open_auth style="background: #d62d2d; color: white; border: none; padding: 10px 20px; cursor: pointer; border-radius: 5px; font-weight: bold;">
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
                    view! { 
                        <div>
                            <h1 style="color:#4cd137;">"Связь установлена!"</h1>
                            <p>"Теперь ваш ник подтянулся в Rust-приложение."</p>
                        </div>
                    }.into_view()
                } else {
                    view! { <p>"Нажмите на кнопку 'ВХОД' в углу."</p> }.into_view()
                }}
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
