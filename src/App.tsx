import news from "../src-tauri/newsData.json";
import "./App.css";

function App() {


  return (
    <div class="container">
      <h1>Mews!!</h1>
      <h2>Get latest updates of Market News.</h2>
      <h3></h3>

      <p>Click on the News to learn more.</p>

      <p>
  {news.map((item) => (
    <p>{item.title}</p>
  ))}
</p>
    </div>
  );
}

export default App;
