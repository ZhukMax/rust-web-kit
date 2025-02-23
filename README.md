# Rust Web Kit

## Getting Started

### Prerequisites
To work with SeaMS, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Docker (to quickly set up PostgreSQL and/or use Docker image)

### Set Up PostgreSQL
You can run the following command to start a PostgreSQL instance using Docker:

```bash
docker run --name sea_ms_db -e POSTGRES_PASSWORD=secret_password -d postgres
```

### Clone the Repository
Clone the project repository from GitHub:

```bash
git clone https://github.com/ZhukMax/rust-web-kit.git
```

### Setup Environment Variables
Create a `.env` file in the root directory with the following variables:

```env
SEAMS_HOST=0.0.0.0
SEAMS_PORT=8080

DATABASE_USER=dbuser
DATABASE_PASSWORD=secret_password
DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_NAME=my_db

DEV_MODE=fasle
```

Replace `secret_password` with the password you set for your Docker PostgreSQL instance.

### Install Dependencies
Run the following command to install the required dependencies:

```bash
make build
```

## License
SeaMS is open-source software licensed under the [MIT License](https://opensource.org/licenses/MIT).
