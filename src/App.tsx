import {open} from "@tauri-apps/api/shell";
import news from `/Users/<username>/.newsData.json`;
import { For, Show} from "solid-js";
import "./App.css";


function App() {

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
