
Cálculo real dos pesos das Extrinsics:
=======================================

As extrincics desse pallet são duas e elas têm pesos diferentes de execução, dados em picoseconds.
Esses pesos podem ser calculados pelo benchmarking do runtime. Por exemplo, para que o cálculo seja feito, tem-se que rodar os seguinte comandos na raiz do projeto:

cargo clean
cargo build -p healthchain-node --release --features runtime-benchmarks -v
./target/release/healthchain-node benchmark pallet --chain dev --pallet pallet_medical_history_reader --extrinsic "*" --steps 50 --repeat 20 --wasm-execution compiled --output ./pallets/medical-history-reader/src/weights.rs

Esses comandos criam o arquivo weights.rs. Tal arquivo contem o valor de cada peso real da execução de cada extrinsic nesse pallet.

Após a criação do arquivo weights.rs, uma linha nele precisa ser substituída manualmente:

- impl<T: frame_system::Config> pallet_medical_history_reader::WeightInfo for WeightInfo<T> {
+ impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {

crate::WeightInfo refere-se à trait definida no próprio pallet; compila tanto quando você roda cargo build --release quanto quando o runtime o usa.

Mas o arquivo weights.rs não precisa ser gerado em todas as máquinas. Esse que já está commitado no projeto pode ser assumido como definitivo.
Portanto, desse ponto em diante no tempo podemo compilar usando apenas o comando 'cargo build --release', por exemplo.

Assim que o node for executado , dessa blockchain, os valores cobrados das contas que executarem essas extrinsics estarão corretos e proporcionais aos pesos definidos.
A convenção de pesos em dots é feita por outro pallet responsável por isso.
-------------------------------------------------------------------------------

Esse pallet foi baseado em código da Solochain. Tal código vem com pesos em valores default. Para alterar para os valores reais, os seguintes arquivos precisaram ser alterados:


	modified:   Cargo.lock
	modified:   Cargo.toml
	modified:   pallets/medical-history-reader/Cargo.toml
	modified:   pallets/medical-history-reader/src/benchmarking.rs
	modified:   pallets/medical-history-reader/src/lib.rs
	modified:   pallets/medical-history-reader/src/weights.rs
	modified:   pallets/medical-history/src/lib.rs
	modified:   pallets/medical-permissions/src/lib.rs
	modified:   runtime/Cargo.toml
	modified:   runtime/src/apis.rs
	modified:   runtime/src/lib.rs


-----------------------------
Rodrigo Pimenta Carvalho - 03/02/2026
