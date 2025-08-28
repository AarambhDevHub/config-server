// use config_client::{ConfigClientBuilder, get_config, get_config_or, init_config};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Initialize tracing
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     // Initialize configuration from config server
//     // This will load all configurations into environment variables
//     init_config(
//         "http://localhost:8888", // Config server URL
//         "application",           // Application name
//         "dev",                   // Profile (dev, prod, etc.)
//         Some("master"),          // Git branch/label
//     )
//     .await?;

//     println!("🚀 Application started with configuration loaded!");

//     // Access configuration values

//     let database_url = get_config("database.url")
//         .await
//         .unwrap_or_else(|| "postgresql://localhost/myapp".to_string());
//     println!("Database URL: {}", database_url);

//     let port = get_config_or("server.port", "8080").await;
//     println!("Server Port: {}", port);

//     let debug_mode = get_config("debug")
//         .await
//         .and_then(|v| v.parse::<bool>().ok())
//         .unwrap_or(false);
//     println!("Debug Mode: {}", debug_mode);

//     // You can also use the client directly for more advanced operations
//     // let client = ConfigClientBuilder::new()
//     //     .server_url("http://localhost:8888")
//     //     .application("myapp")
//     //     .profile("dev")
//     //     .label("master")
//     //     .build();

//     // Encrypt a sensitive value
//     // let encrypted = client.encrypt_value("my-secret-password").await?;
//     // println!("Encrypted value: {}", encrypted);

//     // Decrypt it back
//     // let decrypted = client.decrypt_value(&encrypted).await?;
//     // println!("Decrypted value: {}", decrypted);

//     // Simulate your application logic
//     run_application().await?;

//     Ok(())
// }

// async fn run_application() -> anyhow::Result<()> {
//     // Your application logic here
//     // All configuration is available via environment variables or get_config functions

//     let max_connections = std::env::var("database.max-connections")
//         .unwrap_or_else(|_| "10".to_string())
//         .parse::<i32>()
//         .unwrap_or(10);

//     println!("Max DB Connections: {}", max_connections);

//     // Simulate some work
//     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

//     println!("Application running with loaded configuration...");

//     Ok(())
// }

use config_client::{
    ConfigClientBuilder, get_all_config, get_config, get_config_or, init_config, print_all_config,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize configuration from config server
    // This will load all configurations into environment variables
    init_config(
        "http://localhost:8888", // Config server URL
        "myapp",                 // Application name
        "",                      // Profile (dev, prod, etc.)
        Some("master"),          // Git branch/label
    )
    .await?;

    println!("🚀 Application started with configuration loaded!");

    // Print all configurations loaded from the server
    print_all_config().await?;

    let all_configs = get_all_config().await;
    println!("Number of configurations loaded: {}", all_configs.len());

    // Print environment variables that were set
    println!("\n🌍 Environment variables set from config:");
    for (key, _) in &all_configs {
        if let Ok(env_value) = std::env::var(key) {
            println!("ENV[{}] = {}", key, env_value);
        }
    }

    // Simulate your application logic
    run_application().await?;

    Ok(())
}

async fn run_application() -> anyhow::Result<()> {
    // Your application logic here
    // All configuration is available via environment variables or get_config functions

    println!("Application running with loaded configuration...");

    Ok(())
}
