// src/App.jsx
import './App.css';
import RegistroProducto from './components/RegistroProducto';
import TablaProductos from './components/TablaProductos';

function App() {
  return (
    <div className="container">
      <h2>Inventario Ferretería (React + Rust)</h2>
      <RegistroProducto />
      <hr/>
      <TablaProductos />
    </div>
  );
}

export default App;