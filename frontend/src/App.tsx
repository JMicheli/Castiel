import ChromecastCard from "./components/ChromecastCard";
import Navbar from "./components/Navbar";

function App() {
  // pretend we have three Chromecasts for now
  const devices = [1, 2, 3];

  return (
    <div className="App">
      <Navbar />

      <div className="container is-max-desktop">
        <div className="grid">
          {devices.map((id) => (
            <div className="cell">
              <ChromecastCard key={id} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;
