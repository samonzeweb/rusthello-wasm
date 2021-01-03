// The real entrypoint is index.js.
// See : https://github.com/webpack/webpack/issues/6615#issuecomment-668177931
import "rusthello-wasm";
import { createApp } from "vue";
import App from "./App.vue";

import 'bootstrap'
import 'bootstrap/dist/css/bootstrap.min.css'

createApp(App)
    .mount("#app");
