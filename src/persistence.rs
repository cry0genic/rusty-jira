use crate::store::TicketStore;
use directories::ProjectDirs;
use std::fs::read_to_string;
use std::path::PathBuf;

const PROJECT_NAME: &str = "JIRA";
const ORGANISATION_NAME: &str = "RustLDNUserGroup";
const QUALIFIER: &str = "";

const TICKET_STORE: &str = "ticket_store.yaml";

fn data_store_filename() -> PathBuf {
    let project_dir = ProjectDirs::from(QUALIFIER, ORGANISATION_NAME, PROJECT_NAME)
        .expect("Failed to determine path of the configuration directory.");
    let data_dir = project_dir.data_dir();
    println!("Data storage directory: {:?}", data_dir);

    std::fs::create_dir_all(data_dir).expect("Failed to create data directory.");

    data_dir.join(TICKET_STORE)
}

pub fn load() -> TicketStore {
    let filename = data_store_filename();

    println!("Reading data from {:?}", filename);
    match read_to_string(filename) {
        Ok(data) => {
            serde_yaml::from_str(&data).expect("Failed to parse serialised data.")
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                TicketStore::new()
            }
            _ => panic!("Failed to read data."),
        },
    }
}

pub fn save(ticket_store: &TicketStore) {
    let filename = data_store_filename();

    let content = serde_yaml::to_string(ticket_store).expect("Failed to serialize tickets");

    println!("Saving tickets to {:?}", filename);
    std::fs::write(filename, content).expect("Failed to write tickets to disk.")
}
