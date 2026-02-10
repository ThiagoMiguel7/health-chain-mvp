[![Vídeo Explicativo no YouTube](https://img.shields.io/badge/YouTube-Assistir-FF0000?logo=youtube&logoColor=white)](https://youtu.be/oi6qrWERVR0)

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/Cargo-000000?style=for-the-badge&logo=rust&logoColor=white)](https://doc.rust-lang.org/cargo/)
[![Rust Book](https://img.shields.io/badge/Rust%20Book-000000?style=for-the-badge&logo=rust&logoColor=white)](https://doc.rust-lang.org/book/)
[![Substrate](https://img.shields.io/badge/Substrate-282828?style=for-the-badge&logo=parity-substrate&logoColor=white)](https://docs.substrate.io/)
[![Polkadot](https://img.shields.io/badge/Polkadot-E6007A?style=for-the-badge&logo=polkadot&logoColor=white)](https://docs.polkadot.com/)
[![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=node.js&logoColor=white)](https://nodejs.org/en/docs)
[![npm](https://img.shields.io/badge/npm-CB3837?style=for-the-badge&logo=npm&logoColor=white)](https://docs.npmjs.com/)
[![Yarn](https://img.shields.io/badge/Yarn-2C8EBB?style=for-the-badge&logo=yarn&logoColor=white)](https://yarnpkg.com/getting-started)
[![pnpm](https://img.shields.io/badge/pnpm-F69220?style=for-the-badge&logo=pnpm&logoColor=white)](https://pnpm.io/)
[![IPFS](https://img.shields.io/badge/IPFS-65C2CB?style=for-the-badge&logo=ipfs&logoColor=white)](https://docs.ipfs.tech/)
[![Kubo](https://img.shields.io/badge/Kubo-IPFS%20Implementation-65C2CB?style=for-the-badge&logo=ipfs&logoColor=white)](https://github.com/ipfs/kubo)
[![IPFS CLI](https://img.shields.io/badge/IPFS%20CLI-65C2CB?style=for-the-badge&logo=ipfs&logoColor=white)](https://docs.ipfs.tech/reference/kubo/cli/)
[![IPFS HTTP API](https://img.shields.io/badge/IPFS%20HTTP%20API-65C2CB?style=for-the-badge&logo=ipfs&logoColor=white)](https://docs.ipfs.tech/reference/kubo/rpc/)

# 🏥 InfoHealth MVP ⛓️

> *Prontuário médico descentralizado com controle de acesso paciente→médico, trilha auditável on-chain e arquivos clínicos referenciados por hash.*

---

## 📑 Sumário

- [1. Visão geral](#1-visão-geral)
- [2. Status do MVP (escopo atual)](#2-status-do-mvp-escopo-atual)
- [3. Problema que o projeto resolve](#3-problema-que-o-projeto-resolve)
- [4. Arquitetura e separação de responsabilidades](#4-arquitetura-e-separação-de-responsabilidades)
- [5. Estrutura real do monorepo](#5-estrutura-real-do-monorepo)
- [6. Pallets, storage e extrinsics](#6-pallets-storage-e-extrinsics)
- [7. Front-end: componentes e integrações](#7-front-end-componentes-e-integrações)
- [8. Fluxos funcionais (passo a passo)](#8-fluxos-funcionais-passo-a-passo)
- [9. Pré-requisitos e execução local completa](#9-pré-requisitos-e-execução-local-completa)
- [10. Testes e validações](#10-testes-e-validações)
- [11. Troubleshooting (erros comuns)](#11-troubleshooting-erros-comuns)
- [12. Documentação complementar](#12-documentação-complementar)
- [13. Autores](#13-autores)
- [14. Licença](#14-licença)

---

## 1. Visão geral

O **InfoHealth MVP** é um projeto acadêmico da formação **Polkadot SDK / Substrate (2025/2026)** com foco em prontuário médico descentralizado. O sistema demonstra, de forma prática, como combinar:

- **governança de acesso em blockchain** (quem pode acessar dados);
- **registro auditável e imutável de operações**;
- **arquivos médicos fora da blockchain** (IPFS), vinculados por hash.

A ideia central é preservar auditabilidade sem expor dados sensíveis diretamente on-chain.

### Project Rename Notice

Este repositório foi originalmente criado com o nome **HealthChain MVP** durante a fase inicial da formação Polkadot SDK.

Durante a evolução do projeto e definição de posicionamento do produto, o nome da solução passou a ser:

> **InfoHealth MVP**

O repositório mantém o nome histórico (`health-chain-mvp`) apenas por questões de versionamento e rastreabilidade do desenvolvimento.

### Importante

Alguns scripts, comandos e caminhos ainda podem conter o nome **health-chain**.  
Isso **não afeta o funcionamento do projeto** e será ajustado gradualmente nas próximas versões.

Sempre que houver divergência:

| Antigo | Atual |
|---|---|
| HealthChain | InfoHealth |

## 2. Status do MVP (escopo atual)

Este repositório atualmente contém:

- `blockchain/` (node + runtime + pallets customizados)
- `frontend/` (React + Vite + Polkadot.js + IPFS client)
- `docs/` (requisitos e material acadêmico)

---

## 3. Problema que o projeto resolve

No modelo tradicional, dados clínicos ficam isolados por instituição, com pouca portabilidade e trilha de auditoria limitada para o paciente. O InfoHealth MVP demonstra um desenho onde:

- o **paciente controla** quem pode acessar;
- as ações ficam **auditáveis** na cadeia;
- os arquivos ficam off-chain, referenciados por identificador verificável.

---

## 4. Arquitetura e separação de responsabilidades

### 4.1 Camada On-chain (Substrate)

Responsável por:

- regras de autorização paciente→médico;
- criação de referências de registros médicos;
- leitura autorizada de registros;
- emissão de eventos auditáveis.

### 4.2 Camada Off-chain (IPFS / Kubo)

Responsável por:

- armazenamento/distribuição de arquivos clínicos;
- retorno de CID para referência no fluxo da aplicação.

> **Nota Técnica:** Utilizamos a implementação **Kubo (go-ipfs)** para instanciar o nó IPFS local e expor o Gateway HTTP necessário para a aplicação React.

### 4.3 Camada de Apresentação (Frontend)

Responsável por:

- conectar ao nó Substrate via WebSocket (`ws://127.0.0.1:9944`);
- enviar/executar transações;
- enviar/abrir arquivos no IPFS local (`localhost:5001` / `localhost:8080`).

---

## 5. Estrutura real do monorepo

```text
health-chain-mvp/
├── blockchain/
│   ├── node/                            # binário do nó (InfoHealth-node)
│   ├── runtime/                         # composição dos pallets no runtime
│   ├── pallets/
│   │   ├── medical-permissions/         # grant/revoke de acesso
│   │   ├── medical-history/             # criação e indexação de registros
│   │   ├── medical-history-reader/      # leitura própria e leitura autorizada
│   │   └── history/                     # legado/experimentos
│   ├── scripts/
│   ├── docs/
│   └── env-setup/
├── frontend/
│   ├── src/components/                  # telas do MVP
│   ├── src/contexts/                    # wallet/toast context
│   └── src/utils/                       # integração Polkadot/IPFS
└── docs/                                # requisitos e documentação acadêmica
```

---

## 6. Pallets, storage e extrinsics

## 6.1 `pallet-medical-permissions`

Gerencia concessão e revogação de acesso do médico aos dados do paciente.

| Extrinsic | Assina | Parâmetros | Finalidade |
|---|---|---|---|
| `grant_access` | Paciente | `doctor: AccountId` | concede acesso ao médico |
| `revoke_access` | Paciente | `doctor: AccountId` | revoga acesso do médico |

Regras:

- paciente não pode conceder permissão para si mesmo;
- permissões ficam mapeadas por `(patient, doctor)`.

---

## 6.2 `pallet-medical-history`

Cria e indexa registros médicos do paciente a partir de hash de arquivo.

| Extrinsic | Assina | Parâmetros | Finalidade |
|---|---|---|---|
| `create_record` | Médico | `patient: AccountId`, `file_hash: [u8; 64]` | registra referência médica do paciente |

Regras principais:

- médico precisa de permissão válida do paciente;
- médico não pode criar registro para si mesmo nesse fluxo;
- hash duplicado no índice global é rejeitado.

Índices relevantes no pallet:

- índice global por hash;
- índice por médico;
- índice por paciente.

---

## 6.3 `pallet-medical-history-reader`

Fornece leitura controlada dos registros médicos.

| Extrinsic | Assina | Parâmetros | Finalidade |
|---|---|---|---|
| `read_own_data` | Paciente | `file_hash` | lê próprio registro |
| `read_patient_data` | Médico | `patient`, `file_hash` | lê registro de paciente autorizado |

---

## 7. Front-end: componentes e integrações

### 7.1 Telas principais

- **Permissões:** concede/revoga acesso de médicos.
- **Meu Histórico:** lista histórico próprio.
- **Busca Médica:** médico consulta histórico de paciente autorizado.
- **Criar Registro:** upload + registro on-chain.
- **Histórico Completo:** visão consolidada disponível no app.

### 7.2 Conectividade

- Chain WS padrão: `ws://127.0.0.1:9944`
- IPFS API: `http://localhost:5001`
- IPFS Gateway: `http://localhost:8080/ipfs/<cid>`

### 7.3 Observação importante sobre autenticação no MVP

O contexto de wallet da UI é simplificado para UX do protótipo, enquanto utilitários de integração usam contas de desenvolvimento do keyring (`//Alice`, `//Bob`, `//Charlie`, etc.) para assinatura no ambiente local.

---

## 8. Fluxos funcionais (passo a passo)

## 8.1 Conceder acesso médico

1. Paciente conecta a aplicação.
2. Paciente informa/seleciona conta do médico.
3. Front-end envia `grant_access`.
4. Permissão `(patient, doctor)` passa a valer on-chain.

## 8.2 Criar registro médico

1. Médico realiza upload do arquivo para IPFS.
2. Aplicação obtém CID/hash do artefato.
3. Front-end envia `create_record(patient, file_hash)`.
4. Registro passa a ficar indexado na cadeia.

## 8.3 Ler histórico

- Paciente chama leitura própria.
- Médico chama leitura do paciente (se autorizado).

## 8.4 Revogar acesso

1. Paciente envia `revoke_access`.
2. Novas operações que dependem de permissão devem ser bloqueadas pela regra de acesso.

---

## 9. Pré-requisitos e execução local completa

## 9.1 Pré-requisitos

- Rust + Cargo
- Dependências nativas para compilação Substrate
- Node.js 18+
- npm (ou yarn/pnpm)
- **IPFS (Kubo)** - *Implementação oficial (binário `ipfs`)*

---

## 9.2 Subir a blockchain

```bash
cd blockchain
cargo build --release
./target/release/healthchain-node --dev
```

Endpoint esperado: `ws://127.0.0.1:9944`

Opcional (reset de estado local):

```bash
./target/release/healthchain-node purge-chain --dev
```

---

## 9.3 Subir IPFS local (Kubo Daemon)

Certifique-se de que o binário do Kubo está no seu PATH.

```bash
ipfs daemon
```

Portas esperadas pelo front-end:

- API: `5001`
- Gateway: `8080`

---

## 9.4 Subir o front-end

Em novo terminal:

```bash
cd frontend
npm install
npm run dev
```

Scripts úteis:

```bash
npm run dev
npm run build
npm run lint
npm run typecheck
npm run preview
```
---

## 10. Testes e validações

### 10.1 Blockchain

```bash
cd blockchain
cargo test
```

### 10.2 Front-end (qualidade estática)

```bash
cd frontend
npm run lint
npm run typecheck
npm run build
```

### 10.3 Teste funcional manual recomendado

1. Conceder acesso paciente→médico.
2. Criar registro com arquivo no IPFS.
3. Validar leitura por paciente.
4. Validar leitura por médico autorizado.
5. Revogar acesso e repetir tentativa de operação protegida.

---

## 11. Troubleshooting (erros comuns)

### Erro de conexão com chain

- Verifique se o nó está rodando em `ws://127.0.0.1:9944`.
- Confira firewall/portas locais.

### Upload IPFS falha

- Garanta `ipfs daemon` ativo.
- Confirme API em `http://localhost:5001`.

### Link de arquivo não abre

- Verifique gateway local em `http://localhost:8080`.
- Confira se o CID foi publicado corretamente.

### Extrinsic falha por permissão

- Certifique-se que o paciente executou `grant_access` antes da operação do médico.
- Revalide o par paciente/médico usado na transação.

---

## 12. Documentação complementar

Arquivos relevantes em `docs/`:

- requisitos funcionais e regras de negócio;
- modelagens e documentos de apoio acadêmico;
- materiais de apresentação/relatórios do projeto.

---

## 13. Autores

Projeto desenvolvido para a **Formação Polkadot SDK (2025/2026)**:

- André Luiz Oneti Carvalho
- Rodrigo Pimenta Carvalho
- Thiago da Rocha Miguel

---

## 14. Licença

Projeto acadêmico e experimental, desenvolvido para fins educacionais e de pesquisa.
