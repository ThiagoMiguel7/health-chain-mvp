# 🏥 HealthChain MVP ⛓️

## Prontuário Médico Descentralizado

O **HealthChain MVP** é um *Produto Mínimo Viável* de um sistema de gestão de prontuários médicos descentralizados, focado na soberania do paciente. O projeto foi desenvolvido como **Projeto Final da Formação Polkadot SDK / Substrate (2025/2026)** e adota uma arquitetura Híbrida para garantir privacidade, segurança, integridade e imutabilidade dos dados médicos.

O grande diferencial da solução é o uso de **WebAssembly (Wasm)** para executar criptografia pesada diretamente no navegador do cliente, garantindo que dados sensíveis nunca trafeguem ou sejam armazenados em texto plano, nem mesmo na camada de armazenamento *off-chain*.

---

## Funcionalidades

* **Identidade Soberana (SSI)**
  Autenticação baseada em carteira (*wallet-based authentication*), eliminando a necessidade de e-mail e senha.

* **Criptografia Client-Side**
  Módulo **Rust/Wasm** responsável por cifrar os dados **antes do upload**, garantindo confidencialidade ponta a ponta.

* **Arquitetura Híbrida (On-Chain + Off-Chain)**

  * **On-Chain (Substrate)**: armazenamento de *hashes* de integridade e controle de permissões (ACL).
  * **Off-Chain (Firebase)**: armazenamento apenas de blobs criptografados.

* **Gestão de Acesso**
  O paciente concede e revoga permissões de leitura para médicos diretamente pela Blockchain.

* **Interoperabilidade**
  Arquitetura baseada em padrões Web3, facilitando integração futura com outros sistemas.

---

## Segurança e Privacidade (Arquitetura Híbrida)

O projeto segue rigorosamente os princípios da **LGPD**, adotando um modelo onde **dados sensíveis (PII) nunca tocam a camada pública da Blockchain**.

### Fluxo de Segurança

1. **Navegador (Client)**
   O arquivo é criptografado localmente utilizando uma **chave simétrica gerada em tempo de execução**.

2. **Firebase (Off-Chain)**
   Recebe apenas o **"lixo criptográfico"** (blob cifrado), sem capacidade de leitura.

3. **Substrate (On-Chain)**
   Armazena o **hash do arquivo** e gerencia **quem possui permissão para descriptografar**.

### Zero Knowledge Storage

Mesmo o administrador do banco de dados não consegue acessar ou ler os exames dos pacientes.

---

## Estrutura do Projeto

O projeto é organizado como um **monorepo**, contendo as três camadas principais da aplicação:

```bash
health-chain-mvp/
├── blockchain/                # ⛓️ Camada On-Chain
│   ├── pallets/
│   │   └── medical-record/    # Lógica de registro e permissões
│   ├── runtime/               # Configuração do Runtime Substrate
│   └── node/                  # Configuração do Nó (P2P, RPC)
│
├── wasm-crypto/               # 🔐 Camada de Segurança (Client-side)
│   ├── src/
│   │   └── lib.rs             # Funções Rust de criptografia/hash
│   ├── Cargo.toml
│   └── pkg/                   # Binário compilado para JS (Wasm)
│
├── frontend/                  # 🖥️ Interface do Usuário
│   ├── src/
│   │   ├── components/        # Upload, lista de exames
│   │   ├── services/          # Conexão Firebase e Polkadot.js
│   │   └── wasm/              # Integração com wasm-crypto
│   └── public/
│
└── docs/                      # 📚 Documentação e Atas
```

---

## Explicação dos Módulos

* **`blockchain/`**
  Baseado no *Substrate Node Template*. Contém o *pallet* customizado responsável por armazenar o mapeamento `Hash → Owner` e a lógica de permissões (`grant_access`, `revoke_access`).

* **`wasm-crypto/`**
  Biblioteca Rust compilada para WebAssembly utilizando `wasm-pack`. É o **núcleo de segurança** do projeto, executado diretamente no navegador.

* **`frontend/`**
  Aplicação React responsável por orquestrar a chamada ao módulo Wasm, realizar o upload no Firebase e assinar transações na carteira do usuário.

---

##  Extrinsics e Estruturas de Dados

### Blockchain – Pallet `medical-record`

| Extrinsic       | Parâmetros                           | Descrição                                | Quem Assina |
| --------------- | ------------------------------------ | ---------------------------------------- | ----------- |
| `create_record` | `hash: Vec<u8>`, `cid: Vec<u8>`      | Registra um novo exame e vincula ao dono | Paciente    |
| `grant_access`  | `target: AccountId`, `hash: Vec<u8>` | Concede permissão de leitura a um médico | Paciente    |
| `revoke_access` | `target: AccountId`, `hash: Vec<u8>` | Revoga permissão de leitura              | Paciente    |

---

## Execução Local

### Pré-requisitos

* Rust & Cargo (Stable ou Nightly)
* Node.js + Yarn ou NPM
* Docker (opcional, para testes)

### 1. Compilar o Wasm (Segurança)

```bash

```

### 2. Rodar a Blockchain (Substrate)

```bash

```

> O nó será iniciado na porta **9944 (WebSocket)**.

### 3. Rodar o Frontend

```bash

```

---

## Testes

```bash

```

---

## Stack Utilizada

* **Linguagem Core:** Rust 🦀
* **Blockchain Framework:** Substrate / Polkadot SDK
* **WebAssembly:** wasm-pack (Rust → Wasm)
* **Frontend:** React + Polkadot.js API
* **Banco de Dados:** Firebase Firestore (Google Cloud)
* **Containerização:** Docker

---

## Autores

Projeto desenvolvido para a **Formação Polkadot SDK (2025)**:

* **André Luiz Oneti Carvalho** 
* **Rodrigo Pimenta Carvalho** 
* **Thiago da Rocha Miguel** 

---

## 📜 Licença

Este projeto é acadêmico e experimental, desenvolvido para fins educacionais e de pesquisa.
