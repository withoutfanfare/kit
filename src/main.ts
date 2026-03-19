import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
import "./assets/global.css";

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.mount("#app");

// Expose navigation bridge for tray menu deep links
(window as any).__navigateTo = (path: string) => {
  router.push(path);
};
