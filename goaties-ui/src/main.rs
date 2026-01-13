mod models;
mod repository;

use iced::widget::{button, column, container, text};
use iced::Element;
use repository::FileRepository;
use models::FileMetadata;

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .title("Hello World App")
        .run()
}

struct App {
    show_alert: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    CloseAlert,
}

impl Default for App {
    fn default() -> Self {
        // Example: Initialize repository
        std::thread::spawn(|| {
            if let Err(e) = init_repository() {
                eprintln!("Failed to initialize repository: {}", e);
            }
        });

        Self {
            show_alert: false,
        }
    }
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::ButtonPressed => {
            app.show_alert = true;
        }
        Message::CloseAlert => {
            app.show_alert = false;
        }
    }
}

fn view(app: &App) -> Element<'_, Message> {
    let content = if app.show_alert {
        column![
            text("Hello World").size(32),
            button("Click Me!").on_press(Message::ButtonPressed),
            text("Alert! Button was clicked!").size(24),
            button("Close Alert").on_press(Message::CloseAlert),
        ]
        .spacing(20)
        .padding(20)
    } else {
        column![
            text("Hello World").size(32),
            button("Click Me!").on_press(Message::ButtonPressed),
        ]
        .spacing(20)
        .padding(20)
    };

    container(content)
        .center(iced::Fill)
        .into()
}

// Example repository initialization and usage
fn init_repository() -> anyhow::Result<()> {
    // Initialize repository with file-based storage
    let repo = FileRepository::new("./data/files.db")?;

    // Example: Create a file metadata entry
    let file = FileMetadata::new(
        "example.txt".to_string(),
        "/path/to/example.txt".to_string(),
        1024,
    );

    let created = repo.create(file)?;
    println!("Created file metadata: {:?}", created);

    // Example: Get all files
    let all_files = repo.get_all()?;
    println!("Total files in database: {}", all_files.len());

    Ok(())
}