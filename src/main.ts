import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";

const meta = document.createElement("meta");
meta.name = "naive-ui-style";
document.head.appendChild(meta);
document.documentElement.classList.add("dark");

createApp(App).mount("#app");
