use tonic::{Request, Response, Status};

tonic::include_proto!("inventory");

#[derive(Debug, Default)]
pub struct MyItem;

#[derive(Debug, Default)]
pub struct MyWarehouse;

#[tonic::async_trait]
impl item_service_server::ItemService for MyItem {
    async fn get_item(
        &self,
        request: Request<ItemRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(ItemResponse {
            id: req.id,
            name: "Sample Item".to_string(),
            description: "Placeholder item".to_string(),
            price: 10.0,
        }))
    }

    async fn set_item(
        &self,
        request: Request<ItemSetRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(ItemResponse {
            id: req.id,
            name: req.name,
            description: req.description,
            price: req.price,
        }))
    }

    async fn add_item(
        &self,
        request: Request<ItemAddRequest>,
    ) -> Result<Response<ItemResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(ItemResponse {
            id: 1,
            name: req.name,
            description: req.description,
            price: req.price,
        }))
    }

    async fn get_item_list(
        &self,
        _request: Request<ItemListRequest>,
    ) -> Result<Response<ItemListResponse>, Status> {
        Ok(Response::new(ItemListResponse {
            items: vec![ItemResponse {
                id: 1,
                name: "Sample Item".to_string(),
                description: "Placeholder item".to_string(),
                price: 10.0,
            }],
        }))
    }

    async fn get_item_count(
        &self,
        _request: Request<ItemListRequest>,
    ) -> Result<Response<ItemCountResponse>, Status> {
        Ok(Response::new(ItemCountResponse { count: 1 }))
    }

    async fn delete_item(
        &self,
        _request: Request<ItemRequest>,
    ) -> Result<Response<ItemDeleteResponse>, Status> {
        Ok(Response::new(ItemDeleteResponse { success: true }))
    }

}

#[tonic::async_trait]
impl warehouse_service_server::WarehouseService for MyWarehouse {
    async fn get_warehouse(
        &self,
        request: Request<WarehouseRequest>,
    ) -> Result<Response<WarehouseResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(WarehouseResponse {
            id: req.id,
            name: "Sample Warehouse".to_string(),
            location: "Placeholder location".to_string(),
        }))
    }

    async fn add_warehouse(
        &self,
        request: Request<WarehouseAddRequest>,
    ) -> Result<Response<WarehouseResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(WarehouseResponse {
            id: 1,
            name: req.name,
            location: req.location,
        }))
    }

        async fn set_warehouse(
        &self,
        request: Request<WarehouseSetRequest>,
    ) -> Result<Response<WarehouseResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(WarehouseResponse {
            id: req.id,
            name: req.name,
            location: req.location,
        }))
    }

    async fn get_warehouse_list(
        &self,
        _request: Request<WarehouseListRequest>,
    ) -> Result<Response<WarehouseListResponse>, Status> {
        Ok(Response::new(WarehouseListResponse {
            warehouses: vec![WarehouseResponse {
                id: 1,
                name: "Sample Warehouse".to_string(),
                location: "Placeholder location".to_string(),
            }],
        }))
    }

    async fn get_warehouse_count(
        &self,
        _request: Request<WarehouseListRequest>,
    ) -> Result<Response<WarehouseCountResponse>, Status> {
        Ok(Response::new(WarehouseCountResponse { count: 1, }))
    }

    async fn delete_warehouse(
        &self,
        _request: Request<WarehouseRequest>,
    ) -> Result<Response<WarehouseDeleteResponse>, Status> {
        Ok(Response::new(WarehouseDeleteResponse { success: true }))
    }
}
