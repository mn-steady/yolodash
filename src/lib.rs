use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;
use std::collections::HashMap;
use gloo_utils::format::JsValueSerdeExt;
use serde_json::Value; // Use serde_json for JSON deserialization

// Bind the JavaScript function get_wallet_address using wasm_bindgen
#[wasm_bindgen(module = "/static/wallet.js")]
extern "C" {
    #[wasm_bindgen]
    async fn get_wallet_address() -> JsValue;
}

// Bind JavaScript functions for fetching prices
#[wasm_bindgen(module = "/src/shade-import.js")]
extern "C" {
    #[wasm_bindgen]
    async fn fetchSHDPrice() -> JsValue;

    #[wasm_bindgen]
    async fn fetchBatchPrices() -> JsValue;
}

// Fetch wallet address asynchronously
async fn fetch_wallet_address(set_wallet_address: WriteSignal<String>) {
    match get_wallet_address().await.as_string() {
        Some(address) => {
            set_wallet_address.set(address);
        }
        None => {
            set_wallet_address.set("Failed to load wallet address".to_string());
        }
    }
}

// Fetch SHD price asynchronously
async fn fetch_shd_price(set_shd_price: WriteSignal<String>) {
    match fetchSHDPrice().await.as_string() {
        Some(price_str) => {
            set_shd_price.set(format!("SHD = ${}", price_str));
        }
        None => {
            set_shd_price.set("Price data unavailable".to_string());
        }
    }
}

// Fetch batch prices asynchronously
async fn fetch_batch_prices(set_prices: WriteSignal<HashMap<String, String>>) {
    let result = fetchBatchPrices().await;

    match result.into_serde::<HashMap<String, Value>>() {
        Ok(js_prices) => {
            let mut prices = HashMap::new();
            for (key, value) in js_prices {
                if let Some(price_str) = value.as_str() {
                    prices.insert(key, format!("${}", price_str));
                } else {
                    prices.insert(key, "Error fetching price".to_string());
                }
            }
            set_prices.set(prices);
        }
        Err(_) => {
            web_sys::console::error_1(&"Failed to parse batch prices".into());
            set_prices.set(HashMap::new());
        }
    }
}

// Main app component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (is_connected, set_connected) = create_signal(cx, false);
    let (wallet_address, set_wallet_address) = create_signal(cx, String::from("Not connected"));
    let (shd_price, set_shd_price) = create_signal(cx, String::from("Loading SHD price..."));
    let (prices, set_prices) = create_signal(cx, HashMap::new());
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

    let refresh_batch_prices = move |_| {
        spawn_local(fetch_batch_prices(set_prices.clone()));
    };

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
            <hr class="gold-line" />
            <div class="section-content">
                <div id="shd-price" class="price-display">{shd_price.get()}</div>
                <button class="refresh-price" on:click=refresh_price>"Refresh SHD Price"</button>
                <hr class="gold-line" />
                <button class="refresh-price" on:click=refresh_batch_prices>"Refresh Batch Prices"</button>
                <div class="price-display">
                    <p>{format!("SHD: ${}", prices.get().get("SHD").unwrap_or(&"Loading...".to_string()))}</p>
                    <p>{format!("ETH: ${}", prices.get().get("ETH").unwrap_or(&"Loading...".to_string()))}</p>
                    <p>{format!("BTC: ${}", prices.get().get("BTC").unwrap_or(&"Loading...".to_string()))}</p>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(|cx| view! { cx, <App /> });
}
