# rocket-rust-backend

### Installation

- Clone this repo
```shell
git clone https://github.com/Geriano/rocket-rust-backend
cd rocket-rust-backend
```

- Copy .env.example to .env
```shell
cp .env.example .env
```

- Run container
```shell
docker compose up -d
```

- Migrate database
```shell
docker exec -it rocket-rust-boilerplate-backend diesel migration run
```

Wait until project compiled then your backend app running at http://localhost:8000

### API Documentation
You can see API documentation in http://localhost:8000/swagger-ui/#/
![Swagger UI](https://github.com/Geriano/rocket-rust-backend/assets/59258929/25a94879-7bda-4a5c-91ed-9e42b45db551)
