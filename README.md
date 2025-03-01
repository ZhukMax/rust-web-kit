# Rust Web Kit
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![GitHub Clones](https://img.shields.io/badge/dynamic/json?color=green&label=Clones&query=$.clones&url=https://raw.githubusercontent.com/ZhukMax/rust-web-kit/counter/.github/.clone_count)

Basic setup for developing a web application, API or website in the 🦀 Rust language and the Actix-web framework.

## Features
- Built with **Actix-web** for high-performance HTTP handling.
- **SeaORM** for seamless database integration.
- **Handlebars** template engine for safe and efficient HTML rendering.
- `.env` support for environment-specific configuration.
- **Docker** for containerization application.

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
git clone https://github.com/ZhukMax/rust-web-kit.git my-new-project
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
