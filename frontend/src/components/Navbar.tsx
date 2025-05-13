import { Link } from "react-router-dom";

export default function Navbar() {
  return (
    <nav className="navbar" role="navigation" aria-label="main navigation">
      <div id="navbarBasicExample" className="navbar-menu">
        <div className="navbar-start">
          <Link to="/" className="navbar-item">
            Home
          </Link>

          <Link to="/about" className="navbar-item">
            About
          </Link>
        </div>
      </div>
    </nav>
  );
}
