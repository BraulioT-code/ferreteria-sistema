import { useQuery } from '@tanstack/react-query';
import axios from 'axios';

const fetchHealth = async () => {
  const response = await axios.get('http://localhost:8000/api/health');
  return response.data;
};

function App() {
  const { data, isLoading, isError } = useQuery({
    queryKey: ['apiHealth'],
    queryFn: fetchHealth
  });

  return (
    <div>
      <h1>Sistema Ferretería</h1>
      {isLoading && <p>Cargando estado del backend Rust...</p>}
      {isError && <p style={{ color: 'red' }}>❌ Error al conectar con el backend Rust en puerto 8000.</p>}
      {data && <p style={{ color: 'green' }}>✅ Conexión Exitosa: "{data}"</p>}
    </div>
  );
}

export default App;