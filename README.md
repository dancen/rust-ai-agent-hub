README.md — Rust API Hub for AI Agents

# 🤖 Rust API Hub for AI Agents  
Secure, Lightweight & Token-Efficient Communication Between AI Agents

This project provides a **secure API hub** designed for collaboration between **heterogeneous AI agents**.  
Agents can authenticate, publish encrypted data, and retrieve only the information strictly required to complete their tasks — dramatically reducing token usage, bandwidth, and computation costs.

Built with **Rust**, **Axum**, **Tokio**, and **JWT**, this API is optimized for speed, security, and scalability.

---

## 🚀 Features

- 🔐 **AI Agent Authentication** via API Key + Username  
- 🔑 **JWT-Based Authorization** for secure access  
- 📡 **Encrypted Data Exchange** with one-time or reusable tokens  
- 🪶 **Ultra-lightweight payloads** to minimize tokens exchanged  
- ⚡ **Rust Performance** → High throughput, minimal RAM usage  
- 🧩 **Heterogeneous Agent Support** (can be used by any LLM or agent framework)

---

## 📦 Technology Stack

- **Rust** (stable)
- **Axum** — Web framework
- **Tokio** — Async runtime
- **Serde** — JSON serialization
- **jsonwebtoken** — JWT handling
- **SQLx / SQLite** (optional, depending on your use)
- **MySQL** (optional, depending on your use)

---

# 🔐 1. Login  
### Authenticate an AI agent and receive a JWT.

POST
http://localhost:3000/login


Headers
Content-Type: application/json
x-api-key: q3f8sG7mV2J5K9d1Z0a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t1u2v3w4
username: agent_1234


### ✔️ Response — 200 OK
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MTc2NTAzODczOX0.79ubfMCR_FLxTVoOui1RFVdrIaLmtuzmaSBlRC7Sy7A",
  "expires_at": "2025-12-06T16:32:19.915602400+00:00"
}

❌ Response — 401 Unauthorized
Invalid username or API key


📤 2. Publish
An AI agent uploads encrypted data to share with other agents.

POST
http://localhost:3000/secure/agent/publish

Headers
Content-Type: application/json
x-api-key: ...
Authorization: Bearer <JWT>

Payload
{
  "name": "ai_agent_name",
  "data": "crypted_data_to_share_between_ai_agents_or_services"
}

✔️ Response — 200 OK
{
  "status": "ok",
  "token": "ujRNn4nrI8jBdSPA3fa8PQG3RFuXdnIbCP0vG3vKa4AMMt4XMIIdhI5ELY8igxo3"
}

❌ Response — 401 Unauthorized
Invalid or expired JWT


📥 3. Retrieve
Fetch previously published encrypted content.

GET
http://localhost:3000/secure/retrieve/<token>

Headers
Content-Type: application/json
x-api-key: ...
Authorization: Bearer <JWT>

✔️ Response — 200 OK
{
  "created_at": "2025-12-06T08:54:04",
  "data": "crypted_data_to_share_between_ai_agents_or_services",
  "name": "ai_agent_name",
  "token": "f36Kn3KshTn7EQitTcLYoGemuyEBupGG5F8vGOwHU3XKyUO8jS9fbDuPV1kuHflg",
  "ttl": 86400
}

❌ Response — 401 Unauthorized
Invalid or expired JWT

🔧 Running Locally
1. Clone the repository
git clone https://github.com/dancen/rust-ai-agent-hub.git
cd rust-ai-agent-hub

2. Install dependencies
cargo build

3. Start the server
cargo run


The API runs at:

http://localhost:3000

🔒 Security Notes

All requests require both API Key and JWT

JWTs expire

API keys should be safely stored and rotated

Data exchanged between agents should be encrypted by the client agent before publishing

📘 Use Cases

✔ Multi-agent orchestration
✔ Secure knowledge sharing between LLMs
✔ Distributed reasoning
✔ Token-efficient agent coordination
✔ Agent-to-agent communication for marketplaces, workflows, and pipelines

🤝 Contributing

Contributions are welcome!
Feel free to open issues or submit pull requests.

📄 License

MIT License

🧠 Author

Daniele Centamore
GitHub: @dancen
Feel free to reach out for suggestions or collaborations!
