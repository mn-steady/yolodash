use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;

// Bind the JavaScript function get_wallet_address using wasm_bindgen
#[wasm_bindgen(module = "/static/wallet.js")]
extern "C" {
    #[wasm_bindgen]
    async fn get_wallet_address() -> JsValue;
}

// Fetch wallet address asynchronously
async fn fetch_wallet_address(set_wallet_address: WriteSignal<String>) {
    web_sys::console::log_1(&"Calling get_wallet_address...".into());
    match get_wallet_address().await.as_string() {
        Some(address) => {
            web_sys::console::log_1(&format!("Wallet address: {}", address).into());
            set_wallet_address.set(address);
        }
        None => {
            web_sys::console::log_1(&"Failed to load wallet address".into());
            set_wallet_address.set("Failed to load wallet address".to_string());
        }
    }
}

// Function to fetch the SHD price from the Oracle contract
async fn fetch_shd_price(set_shd_price: WriteSignal<String>) {
    let window = web_sys::window().expect("no global `window` exists");
    let func = js_sys::Reflect::get(&window, &JsValue::from_str("fetchSHDPrice"))
        .expect("fetchSHDPrice function not found")
        .dyn_into::<js_sys::Function>()
        .expect("fetchSHDPrice is not a function");

    // Call the JavaScript function, expecting a Promise
    let promise = func.call0(&JsValue::NULL)
        .expect("Error invoking fetchSHDPrice")
        .dyn_into::<js_sys::Promise>()
        .expect("Expected a Promise from fetchSHDPrice");

    match wasm_bindgen_futures::JsFuture::from(promise).await {
        Ok(price) => {
            if let Some(price_str) = price.as_string() {
                set_shd_price.set(format!("SHD = ${}", price_str));
            } else {
                set_shd_price.set("Price data unavailable".to_string());
            }
        }
        Err(err) => {
            web_sys::console::error_1(&err);
            set_shd_price.set("Error fetching SHD price".to_string());
        }
    }
}

// The main app component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (is_connected, set_connected) = create_signal(cx, false);
    let (wallet_address, set_wallet_address) = create_signal(cx, String::from("Not connected"));
    let (shd_price, set_shd_price) = create_signal(cx, String::from("Loading SHD price..."));
    let (selected_section, set_selected_section) = create_signal(cx, "Shade".to_string());

    let connect_wallet = move |_| {
        set_connected.set(true);
        spawn_local(fetch_wallet_address(set_wallet_address.clone()));
    };

    let disconnect_wallet = move |_| {
        set_connected.set(false);
        set_wallet_address.set(String::from("Not connected"));
    };

    let refresh_price = move |_| {
        spawn_local(fetch_shd_price(set_shd_price.clone()));
    };

    // UI with views
    view! {
        cx,
        <div class="container">
            <div class="top-bar">
                <a href="https://yolodash.com" class="logo">"YoloDash"</a>
                {move || if is_connected.get() {
                    view! { cx,
                        <button class="connect-wallet" on:click=disconnect_wallet>
                            "Logout"
                        </button>
                    }
                } else {
                    view! { cx,
                        <button class="connect-wallet" on:click=connect_wallet>
                            "Connect Wallet"
                        </button>
                    }
                }}
            </div>
            <hr class="gold-line" />
            <div class="links-wallet-container">
                <div class="links">
                    <button class="link-button" on:click=move |_| set_selected_section.set("Home".to_string())>"Home"</button>
                    <button class="link-button" on:click=move |_| set_selected_section.set("Shade".to_string())>"Shade"</button>
                </div>
                <div class="wallet-address">
                    {move || if is_connected.get() {
                        view! { cx, 
                            <span>"SCRT Address: " {wallet_address.get()}</span>
                        }
                    } else {
                        view! { cx, 
                            <span>{wallet_address.get()}</span>
                        }
                    }}
                </div>
            </div>
            <hr class="gold-line" />

            {move || if selected_section.get() == "Home" {
                view! { cx, 
                    <div>
                        <img src="./static/mn-steady.png" class="main-page-image" alt="Main Page Image" />
                    </div>
                }
            } else {
                view! {
                    cx,
                    <div class="section-content">
                        <div id="shd-price" class="price-display">{shd_price.get()}</div>
                        <button class="refresh-price" on:click=refresh_price>"Refresh SHD Price"</button>
                    </div>
                }                                                 
            }}
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(|cx| view! { cx, <App /> });
}
