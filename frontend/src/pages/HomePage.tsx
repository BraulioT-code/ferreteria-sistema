import { useApiHealth } from '../hooks/useAppStatus'; // Importa el hook

const HomePage = () => {
  const { data, isLoading, isError } = useApiHealth(); // Consume el custom hook

  return (
    <div className="status-box">
      <h3>Estado del Backend (Rust)</h3>
      {isLoading && <p>Cargando estado del backend Rust...</p>}
      {isError && <p style={{ color: 'red' }}>❌ Error al conectar con el backend Rust.</p>}
      {data && <p style={{ color: 'green' }}>✅ Conexión Exitosa: "{data}"</p>}
    </div>
  );
};

export default HomePage;