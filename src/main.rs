use leptos::*;

fn main() {
    mount_to_body(|| view! {
        <div style="background: #1a1a1a; color: #e0e0e0; min-height: 100vh; text-align: center; padding: 50px;">
            <div style="border: 2px solid #ff4444; display: inline-block; padding: 20px; border-radius: 15px;">
                <h1 style="color: #ff4444; margin: 0;">"RUSSIAN CHAT VERSIONS"</h1>
                <p style="color: #888;">"Сделано на Rust"</p>
            </div>
            <br/>
            <button on:click=|_| { window().alert_with_message("Подключаемся к F-list API..."); }
                style="margin-top: 20px; padding: 10px 20px; background: #ff4444; color: white; border: none; border-radius: 5px; cursor: pointer;">
                "Проверить API"
            </button>
        </div>
    })
}
