import React from "react";
import { useGetProducts, type Product } from "../hooks/useProducts.ts";

const TablaProductos: React.FC = () => {
    const { data, isLoading, isError, error } = useGetProducts();

    if (isLoading) return <div>Cargando Productos...</div>;
    if (isError) return <div>Error al cargar productos: {error?.message}</div>;

    return (
        <div className="mt-4">
            <h3>Catálogo de Productos ({data?.length || 0} items)</h3>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Nombre</th>
                        <th>Precio de Venta</th>
                        <th>Stock</th>
                        <th>Codigo de Barras</th>
                    </tr>
                </thead>
                <tbody>
                    {data?.map((producto: Product) => (
                        <tr key={producto.id}>
                            <td>{producto.id}</td>
                            <td>{producto.nombre}</td>
                            <td>{producto.precio_venta.toFixed(2)}</td>
                            <td>{producto.stock}</td>
                            <td>{producto.codigo_barras}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

export default TablaProductos;