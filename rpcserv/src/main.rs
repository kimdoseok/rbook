// src/main.rs
use tonic::transport::Server;

mod repository;
mod repo_system;
mod serv_inventory;
mod serv_system;
// Bring generated server traits into scope
use serv_inventory::item_service_server::ItemServiceServer;
use serv_inventory::warehouse_service_server::WarehouseServiceServer;
use serv_system::user_group_service_server::UserGroupServiceServer;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let addr = "[::1]:50051".parse()?;
    
    let inventory_impl = serv_inventory::MyItem::default();
    let warehouse_impl = serv_inventory::MyWarehouse::default();
    let system_impl = serv_system::MyUserGroup::default();

    println!("gRPC Server listening on {}", addr);

    // Combine multiple services onto one server port
    Server::builder()
        .add_service(ItemServiceServer::new(inventory_impl))
        .add_service(WarehouseServiceServer::new(warehouse_impl))
        .add_service(UserGroupServiceServer::new(system_impl))
        .serve(addr)
        .await?;

    Ok(())
}
