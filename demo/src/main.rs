#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use demo::TemplateApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
   // Log to stdout (if you run with `RUST_LOG=debug`).

   tracing_subscriber::fmt::init();

   let native_options = eframe::NativeOptions::default();
   eframe::run_native(
      "auto complete demo",
      native_options,
      Box::new(|_| Box::<TemplateApp>::default()),
   )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
   // Redirect `log` message to `console.log` and friends:
   eframe::WebLogger::init(log::LevelFilter::Debug).ok();

   let web_options = eframe::WebOptions::default();

   wasm_bindgen_futures::spawn_local(async {
      eframe::WebRunner::new()
         .start(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|_| Box::new(TemplateApp::default())),
         )
         .await
         .expect("failed to start eframe");
   });
}
