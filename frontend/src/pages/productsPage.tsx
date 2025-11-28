// src/pages/ProductsPage.jsx

import RegistroProducto from '../components/RegistroProducto';
import TablaProductos from '../components/TablaProductos';

function ProductsPage() { // Renombra la función de App a ProductsPage
  return (
    <div className="container">
      <h2>Inventario Ferretería (React + Rust)</h2>
      <RegistroProducto />
      <hr/>
      <TablaProductos />
    </div>
  );
}

export default ProductsPage;