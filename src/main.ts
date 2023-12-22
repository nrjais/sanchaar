import dayjs from "dayjs";
import duration from "dayjs/plugin/duration";
import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";

import { createPinia } from "pinia";
// @ts-ignore
import { InstallCodemirro } from "codemirror-editor-vue3";
import "codemirror/addon/display/placeholder.js";
import "codemirror/mode/javascript/javascript.js";
import "codemirror/addon/display/placeholder.js";
import "codemirror/addon/fold/foldcode.js";
import "codemirror/theme/dracula.css";

dayjs.extend(duration);

const meta = document.createElement("meta");
meta.name = "naive-ui-style";
document.head.appendChild(meta);
document.documentElement.classList.add("dark");

const app = createApp(App);
const pinia = createPinia();
app.use(pinia);
app.use(InstallCodemirro);
app.mount("#app");
