use std::{sync::Arc, thread, time};

use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Json, Router
};
use axum::extract::Path;
use models::models::{Amount, Extract, TransactionExtract, TransactionResult};
use tokio_postgres::{Client, Config, Error, NoTls};

use crate::models::models::{ClientResult, Transaction};
#[path="../src/models.rs"]
pub mod models;

#[tokio::main]
async fn main(){
    let millis = time::Duration::from_millis(5000);
    thread::sleep(millis);
    let client = connect_database().await;  
    eprintln!("post connection");
    let arch_client = Arc::<Client>::new(client.unwrap());
    let app = configure_routes(arch_client).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Rinha de Backend - 2024/Q1"
}


async fn process_extract(
    State(client): State<Arc<Client>>,
    Path(id) : Path<String>,
) -> (StatusCode, Json<Extract>){
    let stmt_select = format!("SELECT * FROM clients WHERE  id = {}", id);
    let mut list_transactions = Vec::<TransactionExtract>::new();
    let mut client_model = Amount {
        data_extrato: chrono::offset::Local::now().to_string(),
        total: 0,
        limite: 0
    };
    for row in client.query(&stmt_select, &[]).await.unwrap() {
        let id_query: i32 = row.get("id");
        if id_query <= 0 {
            let result = Extract{
                saldo: client_model,
                ultimas_transacoes: list_transactions
            };
            return (StatusCode::NOT_FOUND, Json(result));

        }
        client_model.total =  row.get("saldo");
        client_model.limite = row.get("limite");
    }
    let stmt_select_transactions = format!("select * from transactions where client_id = {} order by realizad_em desc limit 10", id);
    for row in client.query(&stmt_select_transactions, &[]).await.unwrap() {
        let transaction_model = TransactionExtract {
            valor: row.get("valor"),
            descricao: row.get("descricao"),
            realizado_em: row.get("realizad_em"),
            tipo: row.get("tipo")
        };
        list_transactions.push(transaction_model);
    }
    let result = Extract{
        saldo: client_model,
        ultimas_transacoes: list_transactions
    };
    return (StatusCode::OK, Json(result));

}

async fn process_transaction(
    State(client): State<Arc<Client>>,
    Path(id) : Path<String>,
    Json(mut payload): Json<Transaction>,
) -> (StatusCode, Json<Option<TransactionResult>>){
    if id.parse::<i32>().unwrap() <= 0 {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(None));
    }
    if payload.valor <= 0 {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(None));
    }
    if payload.tipo != 'c' || payload.tipo != 'd'{
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(None));
    }
    if payload.descricao.len() < 1 || payload.descricao.len() > 10 {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(None));
    }
    let stmt_select = format!("SELECT * FROM clients WHERE  id = {}", id);
    let mut transaction_model = TransactionResult {
        limite: 0,
        saldo: 0
    };
    let mut client_model = ClientResult {
        id_cliente: 0,
        saldo: 0,
        limite: 0
    };
    for row in client.query(&stmt_select, &[]).await.unwrap() {
        client_model.id_cliente =  row.get("id");
        client_model.saldo =  row.get("saldo");
        client_model.limite = row.get("limite");
        transaction_model.limite = client_model.limite;
        if client_model.id_cliente <= 0 {
            return (StatusCode::NOT_FOUND, Json(Some(transaction_model)));
        }
        if client_model.saldo + payload.valor < -1 * client_model.limite {
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(Some(transaction_model)));
        }
    }
    let stmt_insert = format!("INSERT INTO transactions values(nextval('transactions_id_seq'), {}, {}, '{}', '{}', now())", &id, &payload.valor, &payload.tipo, &payload.descricao); 
    client.execute(&stmt_insert, &[]).await.unwrap();

    if payload.tipo == 'd'{
        payload.valor = payload.valor * -1;
    }
    transaction_model.saldo = client_model.saldo + payload.valor;
    let stmt_second_insert = format!("UPDATE clients SET saldo = {} WHERE id = {}", client_model.saldo + payload.valor, id);
    client.execute(&stmt_second_insert, &[]).await.unwrap();
    return (StatusCode::OK, Json(Some(transaction_model)));

}

async fn configure_routes(client : Arc<Client>) -> Router {
    return Router::new()
    .route("/", get(root))
    .route("/clientes/:id/extrato", get(process_extract))
    .route("/clientes/:id/transacoes", post(process_transaction))
    .with_state(client)
}


async fn connect_database() -> Result<Client, Error>{
    eprintln!("connection initialize");
    let (client, connection) = Config::new()
    .host("db")
    .user("postgres")
    .port(5432)
    .password("rinha")
    .dbname("rinha")
    .connect(NoTls)
    .await?;
    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}