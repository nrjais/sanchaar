import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";
import * as monaco from "monaco-editor";
import {
  loader,
  install as VueMonacoEditorPlugin,
} from "@guolao/vue-monaco-editor";
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";

self.MonacoEnvironment = {
  getWorker(_, label) {
    if (label === "json") {
      return new jsonWorker();
    }
    // Add more workers here - css, html, ts, js
    return new editorWorker();
  },
};

loader.config({ monaco });

const meta = document.createElement("meta");
meta.name = "naive-ui-style";
document.head.appendChild(meta);
document.documentElement.classList.add("dark");

const app = createApp(App);
app.use(VueMonacoEditorPlugin);
app.mount("#app");
