# Backend

## How to run locally:

### Run database:
```shell
docker run -d \
  --name anydesk-test \
  -e POSTGRES_USER=anydesk \
  -e POSTGRES_PASSWORD=mysecretpassword \
  -e POSTGRES_DB=anydesk \
  -p 5432:5432 \
  postgres:16.9
```

### Apply migrations:
```shell
sqlx migrate run
```   
or if you want to add new migration - `sqlx migrate add <MIGRATION_NAME>`


