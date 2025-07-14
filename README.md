# AnyDesk Full-stack Test Project 🛠️

| Layer                | Tech                                        |
|----------------------|---------------------------------------------|
| [Backend](backend)   | **Rust 1.88**, Axum 0.8, SQLx 0.8           |
| [Frontend](frontend) | **Vue 3**, Vuetify 3.9, Pinia 3, Axios 1.10 |
| Docs                 | Swagger / OpenAPI 3 (utoipa + swagger-ui)   |
| Database             | Postgres 16.9                               |


## How to run locally:
```shell
docker compose up --build
```

## Than you can access next URLs:
* frontend - http://localhost
* backend http://localhost:8888
* swagger - http://localhost:8888/swagger-ui
* postgres - postgres://anydesk:mysecretpassword@db:5432/anydesk