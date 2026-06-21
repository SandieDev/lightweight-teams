import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
const webview = new WebviewWindow('my-label', {
  url: 'https://github.com/tauri-apps/tauri'
});
webview.once('tauri://created', function () {
 // webview successfully created
});
webview.once('tauri://error', function (e) {
 // an error happened creating the webview
});