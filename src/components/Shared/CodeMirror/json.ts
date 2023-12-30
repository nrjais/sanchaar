import { json, jsonParseLinter } from "@codemirror/lang-json";
import { linter } from "@codemirror/lint";
import { Extension } from "@codemirror/state";

export const jsonExtensions: Extension[] = [json(), linter(jsonParseLinter())];
