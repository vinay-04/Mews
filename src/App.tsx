import news from "../src-tauri/newsData.json";
import { invoke } from "@tauri-apps/api";
import "./App.css";


function App() {

  return (
    <div class="container">
      <h1>Mews!!</h1>
      <h2>Get latest updates of Market News.</h2>

      <h3>Click on the News to learn more.</h3>


  {news.map((item) => (
    <a href={`${item.url}`}>{item.title}</a>
  ))}

    </div>
  );
}

export default App;
