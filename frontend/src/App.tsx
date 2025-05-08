import './App.css'

function App() {
  const handleClick = () => {
    fetch('/press', { method: 'POST' });
  };

  return (
    <div style={{ display: 'flex', height: '100vh', alignItems: 'center', justifyContent: 'center' }}>
      <button onClick={handleClick}>Press me</button>
    </div>
  );
}

export default App
