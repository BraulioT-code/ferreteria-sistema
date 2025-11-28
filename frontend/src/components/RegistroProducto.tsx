import React, { useState } from "react";
import type { ChangeEvent, FormEvent } from "react";
import { useCreateProduct, type NewProduct } from "../hooks/useProducts";
import { AxiosError } from "axios";

const RegistroProducto: React.FC = () => {
    const [formData, setFormData] = useState<NewProduct>({
        nombre: "",
        precio_venta: 0,
        stock: 0,
        codigo_barras: "",
    });

    const createMutation = useCreateProduct();

    const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setFormData((prev: NewProduct) => ({
            ...prev,
            [name]: name === "precio_venta" || name === "stock" ? parseFloat(value) || 0 : value,
        }));
    };

    const handleSubmit = (e: FormEvent) => {
        e.preventDefault();

        const payload: NewProduct = {
            ...formData,
            precio_venta: Number(formData.precio_venta),
            stock: Number(formData.stock),
        };
        
        createMutation.mutate(payload, {
            onSuccess: () => {
                alert("Producto creado exitosamente");
                setFormData({
                    nombre: "",
                    precio_venta: 0,
                    stock: 0,
                    codigo_barras: "",
                });
            },
            onError: (error: Error) => {
                let errorMessage = 'Error desconocido';
                if (error instanceof AxiosError) {
                    errorMessage = (error.response?.data as { message?: string })?.message || error.message;
                } else {
                    errorMessage = error.message;
                }
                alert(`Error al crear producto: ${errorMessage}`);
            },
        });
    };

    return (
        <div className="card">
            <h3>Crear Nuevo Producto</h3>
            <form onSubmit={handleSubmit}>
                <input name="nombre" value={formData.nombre} onChange={handleChange} placeholder="Nombre" required /><br />
                <input name="precio_venta" value={formData.precio_venta} onChange={handleChange} placeholder="Precio de Venta" type="number" step="1" required /><br />
                <input name="stock" value={formData.stock} onChange={handleChange} placeholder="Stock" type="number" required /><br />
                <input name="codigo_barras" value={formData.codigo_barras} onChange={handleChange} placeholder="Codigo de Barras" required /><br />

                <button type="submit" disabled={createMutation.isPending}>
                    {createMutation.isPending ? "Creando..." : "Crear Producto"}
                </button>
            </form>
        </div>
    );
};

export default RegistroProducto;
