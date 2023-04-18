use axum::{routing::get,Router};
use std::net::SocketAddr;
#[tokio::main]
async fn main(){
    let app=Router::new().route("/", get(root));//アプリケーションのルーティング設定　メソッド設定はgetやpostに変更できる
    //またget(get_hundler).post(post_hundler）のようにメソッドチェーンで呼び起こす事ができる
    let adder=SocketAddr::from(([127,0,0,1],3000));//IPAdder from関数で[]でIPを指定し第二因数でポート番号を指定できる
    axum::Server::bind(&adder)//adder(IPアドレス)をサーバーにバインドしている
     .serve(app.into_make_service())//サーバーが立ち上がる
     .await
     .unwrap();
}
async fn root()->&'static str{//Hello world を返すだけなのでstrings指定である。Jsonやbodyを返すこともできる
    "Hello World"
}