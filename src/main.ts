import dayjs from "dayjs";
import duration from "dayjs/plugin/duration";
import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";

import { createPinia } from "pinia";

dayjs.extend(duration);

const meta = document.createElement("meta");
meta.name = "naive-ui-style";
document.head.appendChild(meta);
document.documentElement.classList.add("dark");

const app = createApp(App);
const pinia = createPinia();
app.use(pinia);
app.mount("#app");
