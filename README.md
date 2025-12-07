🤖 Rust API Hub for AI Agents

Secure, Lightweight & Token-Efficient Communication Between AI Agents (Prototype)

This project is a prototype of a secure API hub designed for collaboration between heterogeneous AI agents.
Agents can authenticate, publish encrypted data, and retrieve only the information strictly required to complete their tasks — dramatically reducing token usage, bandwidth, and computation costs.

Built with Rust, Axum, Tokio, and JWT, this prototype focuses on speed, security, and scalability, but there is ample room for improvements and enhancements.

🚀 Features

🔐 AI Agent Authentication via API Key + Username
🔑 JWT-Based Authorization for secure access
📡 Encrypted Data Exchange with one-time or reusable tokens
🪶 Ultra-lightweight payloads to minimize tokens exchanged
⚡ Rust Performance → High throughput, minimal RAM usage
🧩 Heterogeneous Agent Support (can be used by any LLM or agent framework)
⚠️ Note: This is a prototype — functionality works, but the system can be optimized and extended for production use.

📦 Technology Stack

Rust (stable)
Axum — Web framework
Tokio — Async runtime
Serde — JSON serialization
jsonwebtoken — JWT handling
SQLx / SQLite — optional for local storage
MySQL — optional for production-grade storage

🔐 1. Login

Authenticate an AI agent and receive a JWT.

POST:
http://localhost:3000/login


Headers:
Content-Type: application/json
x-api-key: <your-api-key>
username: <agent-username>

Response — 200 OK
{
  "token": "<JWT_TOKEN>",
  "expires_at": "2025-12-06T16:32:19.915602400+00:00"
}


Response — 401 Unauthorized
Invalid username or API key

📤 2. Publish

An AI agent uploads encrypted data to share with other agents.

POST:
http://localhost:3000/secure/agent/publish

Headers:
Content-Type: application/json
x-api-key: <your-api-key>
Authorization: Bearer <JWT_TOKEN>

Payload:
{
  "name": "ai_agent_name",
  "data": "crypted_data_to_share_between_ai_agents_or_services"
}

Response — 200 OK
{
  "status": "ok",
  "token": "<DATA_TOKEN>"
}


Response — 401 Unauthorized
Invalid or expired JWT

📥 3. Retrieve

Fetch previously published encrypted content.

GET:
http://localhost:3000/secure/retrieve/<token>

Headers:
Content-Type: application/json
x-api-key: <your-api-key>
Authorization: Bearer <JWT_TOKEN>

Response — 200 OK
{
  "created_at": "2025-12-06T08:54:04",
  "data": "crypted_data_to_share_between_ai_agents_or_services",
  "name": "ai_agent_name",
  "token": "<DATA_TOKEN>",
  "ttl": 86400
}

Response — 401 Unauthorized
Invalid or expired JWT

🔧 Running Locally

Clone the repository

git clone https://github.com/dancen/rust-ai-agent-hub.git
cd rust-ai-agent-hub

Install dependencies
cargo build

Start the server
cargo run

Setup the Database
Use the SQL scripts located in the migrations/ folder to initialize your database.

Configure Environment Variables
Set your database connection details and JWT secret in the .env file.

API Base URL:
http://localhost:3000

🔒 Security Notes

All requests require both API Key and JWT
JWTs expire after a configurable period
API keys should be safely stored and rotated
Data exchanged between agents should be encrypted by the client agent before publishing

📘 Use Cases

Multi-agent orchestration
Secure knowledge sharing between LLMs
Distributed reasoning
Token-efficient agent coordination
Agent-to-agent communication for marketplaces, workflows, and pipelines

🤝 Contributing

Contributions are welcome!
Feel free to open issues or submit pull requests.

📄 License

MIT License

🧠 Author

Daniele Centamore
GitHub: @dancen
Feel free to reach out for suggestions or collaborations!
