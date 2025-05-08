import "./App.css";
import ChromecastCard from "./components/ChromecastCard";

function App() {
  // pretend we have three Chromecasts for now
  const devices = [1, 2, 3];

  return (
    <div className="App">
      <header className="App-header">
        <h1>Chromecast Dashboard</h1>
      </header>

      <main className="App-grid">
        {devices.map((id) => (
          <ChromecastCard key={id} />
        ))}
      </main>
    </div>
  );
}

export default App;
