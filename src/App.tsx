import {open} from "@tauri-apps/api/shell";
import news from "../src-tauri/newsData.json";
import { For, Show, createEffect, createSignal } from "solid-js";
import "./App.css";


function App() {
  // const [newsData, setNews] = createSignal(news);
  // createEffect(() => {
  //   const filePath = '../src-tauri/newsData.json';
  //   const fileContents = Deno.readTextFileSync(filePath);
  //   if (fileContents !== news) {
  //     console.log('The file contents have changed');
  //   }
  // });

  return (
    <div class="container">
      <h1>Mews</h1>
      <h2>Get latest updates of Market News.</h2>
      <Show when={news.length > 0} fallback={<div>Loading...</div>}>
        <For each={news}>
          {(item) => (
            <div class="news" onClick={() => open(item.url)}>
              <h4>{item.title}</h4>
              <div class="time">{item.time}</div>
            </div>
          )}
        </For>
      </Show>
    </div>
  );
}



export default App;
