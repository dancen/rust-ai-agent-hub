# 🤖 Rust API Hub for AI Agents

**Secure, Lightweight & Token-Efficient Communication Between AI Agents (Prototype)**

This project is a prototype of a secure API hub designed for collaboration between heterogeneous AI agents. Agents can authenticate, publish encrypted data, and retrieve only the information strictly required to complete their tasks — dramatically reducing token usage, bandwidth, and computation costs.

Built with **Rust**, **Axum**, **SQLx**, **MySQL**, and **JWT**, this prototype focuses on speed, security, and scalability, with room for future improvements.

---

## 🚀 Features

* 🔐 **AI Agent Authentication** via API Key + Username
* 🗝️ **JWT-Based Authorization** for secure access
* 📡 **Encrypted Data Exchange** with one-time or reusable tokens
* 🪶 **Ultra-lightweight payloads** to minimize tokens exchanged
* ⚡ **Rust Performance** → High throughput, minimal RAM usage
* 🧩 **Heterogeneous Agent Support** → Can be used by any LLM or agent framework

> ⚠️ **Note:** This is a prototype — functional, but can be optimized and extended for production.

---

## 📦 Technology Stack

* **Language:** Rust (stable)
* **Web Framework:** Axum
* **JSON Serialization:** `serde` / `serde_json`
* **JWT Handling:** `jsonwebtoken` crate
* **Database:** SQLx for MySQL (SQLite optional for local testing)
* **Environment Variables:** `.env` for DB connection and JWT secret

---

## 🔐 1. Login

Authenticate an AI agent and receive a JWT.

**POST** `http://localhost:3000/login`

**Headers:**

```http
Content-Type: application/json
x-api-key: <API_KEY>
username: <USERNAME>
```

**Response 200 OK:**

```json
{
  "token": "<JWT_TOKEN>",
  "expires_at": "2025-12-06T16:32:19Z"
}
```

**Response 401 Unauthorized:**
Invalid username or API key

---

## 📤 2. Publish Agent Data

Upload encrypted data to share with other agents.

**POST** `http://localhost:3000/secure/agent/publish`

**Headers:**

```http
Content-Type: application/json
x-api-key: <API_KEY>
Authorization: Bearer <JWT_TOKEN>
```

**Payload:**

```json
{
  "name": "ai_agent_name",
  "data": "crypted_data_to_share_between_ai_agents_or_services",
  "ttl": 86400
}
```

**Response 200 OK:**

```json
{
  "status": "ok",
  "token": "<DATA_TOKEN>"
}
```

**Response 401 Unauthorized:**
Invalid or expired JWT

---

## 📥 3. Retrieve Published Data

Fetch previously published encrypted content.

**GET** `http://localhost:3000/secure/retrieve/{token}`

**Headers:**

```http
Content-Type: application/json
x-api-key: <API_KEY>
Authorization: Bearer <JWT_TOKEN>
```

**Response 200 OK:**

```json
{
  "token": "<DATA_TOKEN>",
  "name": "ai_agent_name",
  "data": "crypted_data_to_share_between_ai_agents_or_services",
  "ttl": 86400,
  "created_at": "2025-12-06T08:54:04Z"
}
```

**Response 401 Unauthorized:**
Invalid or expired JWT

---

## 🔧 Running Locally

1. Clone the repository:

```bash
git clone https://github.com/dancen/rust-ai-agent-hub.git
cd rust-ai-agent-hub
```

2. Install dependencies:

```bash
cargo build
```

3. Configure the `.env` file:

```env
DATABASE_URL=mysql://rust_user:password@localhost:3306/rust_ai_agent_hub
JWT_SECRET=super_secret_key_123
```

4. Initialize the database (run SQL scripts in `migrations/`)

5. Start the server:

```bash
cargo run
```

**API Base URL:** `http://localhost:3000`

---

## 🔒 Security Notes

* All requests require both **API Key** and **JWT**
* JWTs expire after a configurable period
* API keys should be safely stored and rotated
* Data exchanged between agents should be encrypted by the client agent before publishing

---

## 📘 Use Cases

* Multi-agent orchestration
* Secure knowledge sharing between LLMs
* Distributed reasoning
* Token-efficient agent coordination
* Agent-to-agent communication for marketplaces, workflows, and pipelines

---

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

---

## 📄 License

MIT License

---

## 🧠 Author

**Daniele Centamore**
GitHub: [@dancen](https://github.com/dancen)

Feel free to reach out for suggestions or collaborations!
