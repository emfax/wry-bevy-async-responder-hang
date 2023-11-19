# WRY Bevy Async Responder Hang

This repo uses the latest WRY to create a webview overlay on top of a Bevy `wgpu` window.

A custom async protocol is setup to receive events on the front end.

On calling `responder.respond(response)`, the app hangs.

Looking at the call stack on macOS there is about a billion of these: 

`1751 std::sync::condvar::Condvar::wait::h2cc6c63e4c785e6d`