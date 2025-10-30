# Baby API

**[বাংলা README](README.bn.md)** | English Documentation

---

A simple Q&A API service built with Rust that allows users to query a knowledge base, teach the system new question-answer pairs, and securely download the database.

## 🚀 Features

- **Query Knowledge Base**: Ask questions and get stored answers
- **Teach System**: Add new question-answer pairs to the knowledge base
- **Database Management**: Securely download the SQLite database with password protection
- **RESTful API**: Clean HTTP endpoints with proper status codes
- **SQLite Storage**: Persistent local database storage

## 🛠️ Technology Stack

- **Language**: Rust (2021 edition)
- **Web Framework**: Axum
- **Database**: SQLite with sqlx ORM
- **Async Runtime**: Tokio
- **Serialization**: Serde

## 📋 Prerequisites

- Rust 1.70+ 
- Cargo package manager

## 🔧 Installation

1. Clone or download the project:
```bash
# If you have the source code
cd baby_api
```

2. Install dependencies:
```bash
cargo build --release
```

## ⚙️ Configuration

Create a `.env` file in the project root:

```bash
# Database configuration
DATABASE_URL=sqlite://baby.db

# Server configuration
PORT=2832

# Database download password (optional - defaults to empty)
DB_PASSWORD=your_secure_password
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite://baby.db` | SQLite database path |
| `PORT` | `2832` | Server port |
| `DB_PASSWORD` | (empty) | Password for database download |

## 🏃‍♂️ Running the Application

### Development Mode
```bash
cargo run
```

### Production Mode
```bash
cargo run --release
```

The server will start on `http://0.0.0.0:2832` by default.

## 📡 API Endpoints

### 1. Query Knowledge Base

**GET** `/baby`

Query the stored knowledge base for an answer.

**Parameters:**
- `ask` (query parameter): The question to ask

**Response:**
```json
{
  "ans": "Your stored answer here"
}
```

**Example:**
```bash
curl "http://localhost:2832/baby?ask=What%20is%20Rust%3F"
```

### 2. Teach System

**POST** `/baby/teach`

Add a new question-answer pair to the knowledge base.

**Parameters:**
- `ask` (query parameter): The question
- `ans` (query parameter): The answer

**Response:**
- `"success res"` - Successfully added
- `"already haved"` - Question already exists

**Example:**
```bash
curl "http://localhost:2832/baby/teach?ask=What%20is%20Rust%3F&ans=A%20systems%20programming%20language"
```

### 3. Download Database

**GET** `/baby/download`

Download the SQLite database file (password required).

**Parameters:**
- `password` (query parameter): Database password

**Response:**
- SQLite database file (if authenticated)
- `401 Unauthorized` - Invalid password
- `404 Not Found` - Database file doesn't exist
- `500 Internal Server Error` - Failed to read database

**Example:**
```bash
curl "http://localhost:2832/baby/download?password=your_password" --output baby.db
```

## 🗄️ Database Schema

The application uses a SQLite database with the following schema:

**Table: `baby`**
- `ask` (TEXT, PRIMARY KEY) - The question
- `ans` (TEXT) - The answer

The database is automatically created when the application starts if it doesn't exist.

## 🔒 Security Features

- **Password Protection**: Database downloads require authentication
- **SQL Injection Protection**: Uses parameterized queries via sqlx
- **Input Validation**: Basic validation on all endpoints
- **Error Handling**: Proper HTTP status codes and error messages

## 📝 Usage Examples

### Basic Workflow

1. **Teach the system a new Q&A pair:**
```bash
curl "http://localhost:2832/baby/teach?ask=What%20is%20the%20capital%20of%20France%3F&ans=Paris"
```

2. **Query for the answer:**
```bash
curl "http://localhost:2832/baby?ask=What%20is%20the%20capital%20of%20France%3F"
```

3. **Response:**
```json
{
  "ans": "Paris"
}
```

### Database Management

1. **Set up password protection** (add to `.env`):
```bash
DB_PASSWORD=my_secure_password_123
```

2. **Download database:**
```bash
curl "http://localhost:2832/baby/download?password=my_secure_password_123" --output backup.db
```

## 🐛 Error Handling

The API returns appropriate HTTP status codes:

- **200 OK**: Successful operation
- **404 Not Found**: Question not found or database file missing
- **401 Unauthorized**: Invalid password for database download
- **500 Internal Server Error**: Database or server error

## 🔄 Development

### Project Structure

```
baby_api/
├── src/
│   ├── main.rs          # Application entry point
│   ├── server.rs        # HTTP router setup
│   ├── baby.rs          # Query handling
│   ├── teach.rs         # Teaching functionality
│   ├── config.rs        # Configuration management
│   └── download_db.rs   # Database download handler
├── Cargo.toml           # Project configuration
├── .env                 # Environment variables
└── README.md           # This file
```

### Key Dependencies

- **axum**: Web framework
- **sqlx**: Async SQL toolkit with SQLite support
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **dotenv**: Environment variable management

## 🚀 Deployment

### Docker Deployment
```dockerfile
FROM rust:1.70-slim
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 2832
CMD ["cargo", "run", "--release"]
```

### Systemd Service
```ini
[Unit]
Description=Baby API Service
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/baby_api
ExecStart=/opt/baby_api/target/release/baby_api
EnvironmentFile=/opt/baby_api/.env

[Install]
WantedBy=multi-user.target
```

## 📄 License

This project is open source. Feel free to use, modify, and distribute as needed.

## 🤝 Contributing

1. Fork the project
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📞 Support

For issues, questions, or contributions, please refer to the project repository.
