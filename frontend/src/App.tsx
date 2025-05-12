import { BrowserRouter, Routes, Route } from "react-router-dom";
import Navbar from "@components/Navbar";
import About from "./pages/About";
import Home from "./pages/Home";

function App() {
  return (
    <BrowserRouter>
      <div className="App">
        <Navbar />
        <div className="container is-max-desktop mt-4 px-3">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/about" element={<About />} />
          </Routes>
        </div>
      </div>
    </BrowserRouter>
  );
}

export default App;
