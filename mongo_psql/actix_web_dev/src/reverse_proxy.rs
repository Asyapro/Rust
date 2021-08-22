use std::net::ToSocketAddrs;

use actix_web::client::Client;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use url::Url;
use crate::db::Auth;


pub async fn forward(
    req: HttpRequest,
    service_name: web::Path<(String,String)>,
    secret: web::Data<AuthSecret>,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    let url = pass_addr(service_name.into_inner().1, "8088".to_owned());
    println!("{:?}",url);
    let mut new_url = url; new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());
    let client = Client::new();
    println!("{:?}",new_url.as_str());

    // TODO: This forwarded implementation is incomplete as it only handles the inofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };
    let jwt = req.headers().get("jwt").unwrap().to_str().unwrap(); 
    let auth = Auth::validate_jwt(jwt, secret.get()).unwrap();
    forwarded_req.header("auth", format!("{}",auth.id));

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;
    println!("{:?}",res);

    let mut client_resp = HttpResponse::build(res.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in
        res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    let d = client_resp.body(res.body().await?);
    println!("{:?}",d);
    Ok(d)
}

pub fn pass_addr(
    forwarded_addr: String, 
    forwarded_port: String
) -> Url {
    let forwarded_port: u16 = forwarded_port.parse().unwrap();
    Url::parse(&format!("http://{}:{}",forwarded_addr, forwarded_port))
    .unwrap()
}
