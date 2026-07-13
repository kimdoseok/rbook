use tonic::transport::Channel;
use inventory::item_service_client::ItemServiceClient;
use inventory::warehouse_service_client::WarehouseServiceClient;
use system::user_group_service_client::UserGroupServiceClient;

pub mod inventory {
    tonic::include_proto!("inventory");
}

pub mod system {
    tonic::include_proto!("system");
}

#[derive(Debug)]
struct UserGroup {
    id: i64,
    yug_code: String,
    yug_name: String,
    yug_memo: String,
    yug_active: bool,
}

impl From<system::UserGroupSetRequest> for UserGroup {
    fn from(req: system::UserGroupSetRequest) -> Self {
        UserGroup {
            id: req.id,
            yug_code: req.yug_code,
            yug_name: req.yug_name,
            yug_memo: req.yug_memo,
            yug_active: req.yug_active,
        }
    }
}
impl From<system::UserGroupAddRequest> for UserGroup {
    fn from(req: system::UserGroupAddRequest) -> Self {
        UserGroup {
            id: 0,
            yug_code: req.yug_code,
            yug_name: req.yug_name,
            yug_memo: req.yug_memo,
            yug_active: req.yug_active,
        }
    }
}

impl From<system::UserGroupResponse> for UserGroup {
    fn from(resp: system::UserGroupResponse) -> Self {
        UserGroup {
            id: resp.id,
            yug_code: resp.yug_code,
            yug_name: resp.yug_name,
            yug_memo: resp.yug_memo,
            yug_active: resp.yug_active,
        }
    }
}

impl From<UserGroup> for system::UserGroupAddRequest {
    fn from(yug: UserGroup) -> Self {
        system::UserGroupAddRequest {
            yug_code: yug.yug_code,
            yug_name: yug.yug_name,
            yug_memo: yug.yug_memo,
            yug_active: yug.yug_active,
        }
    }
}

impl From<UserGroup> for system::UserGroupSetRequest {
    fn from(yug: UserGroup) -> Self {
        system::UserGroupSetRequest {
            id: yug.id,
            yug_code: yug.yug_code,
            yug_name: yug.yug_name,
            yug_memo: yug.yug_memo,
            yug_active: yug.yug_active,
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

async fn get_item_by_id(
    client: &mut ItemServiceClient<tonic::transport::Channel>,
    id: &str,
) -> inventory::ItemResponse {
    let request = tonic::Request::new(inventory::ItemRequest { id: id.parse().unwrap() });
    client.get_item(request).await.unwrap().into_inner()
}
async fn get_item_list(
    client: &mut ItemServiceClient<tonic::transport::Channel>,
    filter: &str,
) -> inventory::ItemListResponse {
    let request = tonic::Request::new(inventory::ItemListRequest { filter: filter.into() });
    client.get_item_list(request).await.unwrap().into_inner()
}

async fn get_user_group_list(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    filter: &str,
) -> system::UserGroupListResponse {
    let request = tonic::Request::new(system::UserGroupListRequest { filter: filter.into() });
    client.get_user_group_list(request).await.unwrap().into_inner()
}

async fn get_warehouse_by_id(
    client: &mut WarehouseServiceClient<tonic::transport::Channel>,
    id: &str,
) -> inventory::WarehouseResponse {
    let request = tonic::Request::new(inventory::WarehouseRequest { id: id.parse().unwrap() });
    client.get_warehouse(request).await.unwrap().into_inner()
}
async fn get_warehouse_list(
    client: &mut WarehouseServiceClient<tonic::transport::Channel>,
    filter: &str,
) -> inventory::WarehouseListResponse {
    let request = tonic::Request::new(inventory::WarehouseListRequest { filter: filter.into() });
    client.get_warehouse_list(request).await.unwrap().into_inner()
}

async fn get_user_group_by_id(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    id: i64,
) -> system::UserGroupResponse {
    let request = tonic::Request::new(system::UserGroupRequest { id, yug_code: String::new() });
    client.get_user_group_by_id(request).await.unwrap().into_inner()
}
async fn get_user_group_by_code(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    yug_code: &str,
) -> Result<system::UserGroupResponse, tonic::Status> {
    let request = tonic::Request::new(system::UserGroupRequest {
        id: 0,
        yug_code: yug_code.into(),
    });
    let response = client.get_user_group_by_code(request).await?;
    Ok(response.into_inner())
}
async fn get_user_group_count(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    filter: &str,
) -> system::UserGroupCountResponse {
    let request = tonic::Request::new(system::UserGroupListRequest { filter: filter.into() });
    client.get_user_group_count(request).await.unwrap().into_inner()
}
async fn add_user_group(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    yug: UserGroup,
) -> system::UserGroupResponse {
    let request = tonic::Request::new(system::UserGroupAddRequest {
        yug_code: yug.yug_code,
        yug_name: yug.yug_name,
        yug_memo: yug.yug_memo,
        yug_active: yug.yug_active,
    });
    client.add_user_group(request).await.unwrap().into_inner()
}
async fn set_user_group(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    yug: UserGroup,
) -> system::UserGroupResponse {
    let request = tonic::Request::new(system::UserGroupSetRequest {
        id: yug.id,
        yug_code: yug.yug_code,
        yug_name: yug.yug_name,
        yug_memo: yug.yug_memo,
        yug_active: yug.yug_active,
    });
    client.set_user_group(request).await.unwrap().into_inner()
}
async fn delete_user_group_by_id(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    id: i64,
) -> system::UserGroupDeleteResponse {
    let request = tonic::Request::new(system::UserGroupRequest {
        id,
        yug_code: String::new(),
    });
    client.delete_user_group(request).await.unwrap().into_inner()
}
async fn delete_user_group_by_code(
    client: &mut UserGroupServiceClient<tonic::transport::Channel>,
    yug_code: &str,
) -> system::UserGroupDeleteResponse {
    let request = tonic::Request::new(system::UserGroupRequest {
        id: 0,
        yug_code: yug_code.into(),
    });
    client.delete_user_group(request).await.unwrap().into_inner()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://localhost:50051")
    .connect()
    .await?;

    let mut item_client = ItemServiceClient::new(channel.clone());
    let mut user_group_client = UserGroupServiceClient::new(channel.clone());
    let mut warehouse_client = WarehouseServiceClient::new(channel.clone());
    //let response = client
    //    .get_item(tonic::Request::new(inventory::ItemRequest { id: "1".into() }))
    //    .await?;
    let response = get_item_by_id(&mut item_client, "1").await;
    println!("RESPONSE={:?}", response);
    let response1 = get_item_list(&mut item_client, "1").await;
    println!("RESPONSE1={:?}", response1);

    let response4 = get_warehouse_by_id(&mut warehouse_client, "1").await;
    println!("RESPONSE4={:?}", response4); 
    let response5 = get_warehouse_list(&mut warehouse_client, "1").await;
    println!("RESPONSE5={:?}", response5);

    let yug = UserGroup {
        id: 0,
        yug_code: "TEST".to_string(),
        yug_name: "Test Group".to_string(),
        yug_memo: "This is a test user group".to_string(),
        yug_active: true,
    };
    let mut id: i64 = 1;
    println!("=1===============================");
    let response2 = get_user_group_by_id(&mut user_group_client, id).await;
    println!("RESPONSE2={:?}", &response2);
    println!("Name={:?}", &response2.yug_name);
    println!("=2===============================");
    let response3 = get_user_group_list(&mut user_group_client, "").await;
    println!("RESPONSE6={:?}", response3);
    println!("=3===============================");
    // Map the generated protobuf message to your custom struct
    let usergroup_struct: UserGroup = response2.into();
    println!("Mapped User: {:?}", usergroup_struct);
    println!("=4===============================");

    let yug = UserGroup {
        id: 0,
        yug_code: "TEST".to_string(),
        yug_name: "Test Group".to_string(),
        yug_memo: "This is a test user group".to_string(),
        yug_active: true,
    };
    let filter = "";

    match get_user_group_by_code(&mut user_group_client, "ADMINS").await {
        Ok(yug_response) => {
            println!("User group by code: {} {:?}", &yug_response.yug_code, &yug_response);
            print_type_of(&yug_response);
            println!("{:?}", &yug_response);
            println!("=5===============================");
            if yug_response.id > 0 {
                let result = delete_user_group_by_id(&mut user_group_client, yug_response.id).await;
                println!("User group deleted by ID: {} {:?}", yug_response.id, result);
            } else {
                println!("User group exists but does not have a valid id.");
            }
        }
        Err(status) => {
            println!("get_user_group_by_code failed: {status:?}");
        }
    }
    let yug = add_user_group(&mut user_group_client, yug).await;
    println!("User group added: {:?}", &yug);
    println!("=6===============================");
    let yugs = get_user_group_list(&mut user_group_client, filter).await;
    for yug in &yugs.usergroups {
        println!("Yug List {}: {:?}", yug.id, yug);
    }
    let yug_count = get_user_group_count(&mut user_group_client, filter).await;
    println!("User group count: {}", yug_count.count);
    println!("=7===============================");
    match get_user_group_by_code(&mut user_group_client, "TEST").await {
        Ok(yug) => {
            println!("User group by code: {} {:?}", yug.yug_code, &yug);
            println!("=8===============================");
            let mut yug: UserGroup = yug.into();
            yug.yug_memo = "Updated memo".to_string();
            let yug = set_user_group(&mut user_group_client, yug).await;
            println!("User group updated: {} {:?}", yug.yug_code, &yug);
            println!("=9===============================");
            let yug = get_user_group_by_id(&mut user_group_client, yug.id).await;
            println!("User group by ID: {} {:?}", yug.id, yug);
            println!("=10===============================");
            let result = delete_user_group_by_id(&mut user_group_client, yug.id).await;
            println!("User group deleted by ID: {} {:?}", yug.id, result);
        }
        Err(status) => {
            println!("get_user_group_by_code(TEST) failed: {status:?}");
        }
    }


    Ok(())
}
