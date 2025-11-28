import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import type { UseMutationResult, UseQueryResult } from "@tanstack/react-query";
import api from "../api/client";

export interface Product {
    id: number;
    nombre: string;
    precio_venta: number;
    stock: number;
    codigo_barras: string;
}

export interface NewProduct {
    nombre: string;
    precio_venta: number;
    stock: number;
    codigo_barras: string;
}

const fetchProducts = async (): Promise<Product[]> => {
    const { data } = await api.get<Product[]>("/productos");
    return data;
};

export const useGetProducts = (): UseQueryResult<Product[], Error> => {
    return useQuery({
        queryKey: ["productos"],
        queryFn: fetchProducts,
    });
};

const createProduct = async (newProduct: NewProduct): Promise<Product> => {
    const { data } = await api.post<Product>("/productos", newProduct);
    return data;
};

export const useCreateProduct = (): UseMutationResult<Product, Error, NewProduct> => {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: createProduct,
        onSuccess: () => {
            // Invalida el caché de 'productos' para que TanStack Query 
            // automáticamente re-ejecute useGetProducts y actualice la lista.
            queryClient.invalidateQueries({ queryKey: ["productos"] });
        },
    });
};
