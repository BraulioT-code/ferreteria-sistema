# Ferretería Sistema

Sistema de gestión para ferretería construido con Rust (Backend) y React (Frontend).

## Estructura del Proyecto

```mermaid
graph TD
    subgraph Project [Ferretería Sistema]
        subgraph Backend [Backend (Rust/Actix)]
            M[main.rs] --> H[handlers.rs]
            M --> P[products.rs]
            H --> Mod[models.rs]
            P --> Mod
            Mig[Migrations] --> DB[(PostgreSQL)]
        end
        subgraph Frontend [Frontend (React/Vite)]
            App[App.tsx] --> Pages[Pages]
            Pages --> Comp[Components]
            Comp --> Hooks[Hooks]
            Hooks --> API[API Service]

            subgraph PagesDetail [Pages]
                HP[HomePage.tsx]
                PP[productsPage.tsx]
            end

            subgraph CompDetail [Components]
                RP[RegistroProducto.tsx]
                TP[TablaProductos.tsx]
            end

            subgraph HooksDetail [Hooks]
                UP[useProducts.ts]
                UAS[useAppStatus.ts]
            end
        end
        Frontend -- HTTP Requests --> Backend
    end
```

## Tecnologías

- **Backend**: Rust, Actix-web, SQLx, PostgreSQL
- **Frontend**: React, TypeScript, Vite, TailwindCSS
