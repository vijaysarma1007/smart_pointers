#[derive(Debug)]
enum FilesSystemEntity {
    File {
        name: String,
    },
    Folder {
        name: String,
        content: Vec<FilesSystemEntity>,
    },
}

// vector is smartpointer which automatically stores the data dynamically in heap and stores its references in stack like box

fn main() {
    let rust_file = FilesSystemEntity::File {
        name: String::from("my_rust_code.rs"),
    };

    let python_file = FilesSystemEntity::File {
        name: String::from("my_python_code.py"),
    };

    let code_folder = FilesSystemEntity::Folder {
        name: String::from("Code stuff"),
        content: vec![rust_file, python_file],
    };

    let screenplay = FilesSystemEntity::File {
        name: String::from("my screenplay.txt"),
    };

    let all_documents = FilesSystemEntity::Folder {
        name: String::from("Documents"),
        content: vec![screenplay, code_folder],
    };
}
