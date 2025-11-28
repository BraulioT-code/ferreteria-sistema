import { useQuery } from "@tanstack/react-query";
import { fetchHealth } from "../api/client";

export const useApiHealth = () => {
    return useQuery({
        queryKey: ["apiHealth"],
        queryFn: fetchHealth,
        staleTime: 5 * 60 * 1000,
    });
};