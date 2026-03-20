use github_actions_demo::handler::ItemHandler;
use github_actions_demo::model::CreateItemRequest;
use github_actions_demo::store::InMemoryStore;

fn main() {
    let handler = ItemHandler::new(InMemoryStore::new());

    let item = handler
        .create(CreateItemRequest {
            name: "Example item".to_owned(),
            description: Some("Created locally".to_owned()),
            owner_id: "local-dev".to_owned(),
        })
        .expect("failed to create item");

    println!("Created: {} ({})", item.name(), item.id());

    let items = handler.list().expect("failed to list items");
    println!("Total items: {}", items.len());
}
