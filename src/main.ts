import dayjs from "dayjs";
import duration from "dayjs/plugin/duration";
import { createApp } from "vue";
import App from "./App.vue";
import "./styles.scss";
import { createPinia } from "pinia";

dayjs.extend(duration);

const meta = document.createElement("meta");
meta.name = "naive-ui-style";
document.head.appendChild(meta);
document.documentElement.classList.add("dark");

const pinia = createPinia();
const app = createApp(App);
app.use(pinia);
app.mount("#app");
