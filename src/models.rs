pub mod models{
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    #[derive(Serialize)]
    pub struct Extract{
        pub saldo: Amount,
        pub ultimas_transacoes: Vec<TransactionExtract>
    }
    #[derive(Serialize)]
    #[derive(Deserialize)]
    pub struct TransactionResult {
        pub limite: i32,
        pub saldo: i32,
    }
    #[derive(Serialize)]
    #[derive(Deserialize)]
    pub struct ClientResult {
        pub id_cliente: i32,
        pub limite: i32,
        pub saldo: i32,
    }
    #[derive(Serialize)]
    #[derive(Deserialize)]
    pub struct Transaction {
        pub valor: i32,
        pub tipo: char,
        pub descricao: String
    }
    #[derive(Serialize)]
    #[derive(Deserialize)]
    pub struct TransactionExtract {
        pub valor: i32,
        pub tipo: String,
        pub descricao: String,
        pub realizado_em: String
    }
    #[derive(Serialize)]
    #[derive(Deserialize)]
    pub struct Amount{
        pub total: i32,
        pub data_extrato: String,
        pub limite: i32
    }
}

