# Requisitos

## Stores da Aplicação

- ExamStore
- UserHistoryStore
- PermissionsStore
- MedicalHistoryStore

## Issues da Aplicação

- User libera permissão criação no histórico para o médico X
- User revoga permissão de criação para o médico X
- User lê histórico o proprio médico
<br/>
- Médico consulta histórico do paciente Y
- Médico cria dados no histórico do paciente Y
- Médico requisita permissão de criação no histórico do paciente Y

---

histórico -> transações/blocks

## Criação dos stores da Aplicação

### Criar os stores

- histórico do paciente
  - Qual médico criou
    - CRM
    - Nome
    - AccountId
  - Data de criação
  - Hash do arquivo CID (QmNLpCy6zc5m6xrJ3iuPygXRLVVfujVK5G6qYyNWgjoCBo)
- histórico do médico
  - Qual médico criou
    - CRM
    - Nome
    - AccountId
  - Qual paciente
    - AccountId
    - Nome
  - Data de criação
- Permissões do médico
  - Paciente que liberou
  - Quando liberou
  - Qual permissão liberada (RW)
- Histórico de Atestado Médico (2ª fase)
  - Qual médico criou
    - CRM
    - Nome
    - AccountId
  - Data de criação
- Histórico de permissões do médico (2ª fase)
  - Paciente
    - AccountId
    - Nome
  - Ação
    - Liberação
    - Revogação
  - Data de criação

### Regras de negócios

- Paciente busca o próprio histórico
  - Validar que o paciente é ele mesmo
- Paciente gerência permissões do médico
  - Permite acesso ao histórico
  - Permite a edição do histórico (criar novos dados)
  - Revogar acesso ao histórico
  - Revogar edição do histórico
- Médico busca o histórico do Paciente
  - Validar permissão de leitura do histórico do paciente
  - Retornar os dados do histórico (on chain)
  - Retornar os arquivos do histórico (off chain)
- Médico edita o histórico do Paciente
  - Validar a permissão de escrita no histórico do paciente
  - Permitir a adição de arquivos no histórico (off chain)
  - Permitir adicionar a transação no histórico do paciente (on chain)
- Ter um "ator" para a gerencia dos arquivos off chain
  - Salvamento dos arquivos no DB e retorno do hash gerado
- Ter validações em cada transação de escrita do medico no histórico do paciente (2ª fase)
  - Medico requisita confirmação de adição dos dados no histórico (2FA)
  - Ter um tempo limite para aceite ou recusa da inserção

## Motivos da aderência da HealthChain

- Vide arquivo [DS_HealthChain](./DS_HealthChain.pdf)
