mod src {
    pub mod controller {
        pub mod auth;
    }

    pub mod model {
        pub mod entity {
            pub mod user;
        }

        pub mod repository {
            pub mod user;
        }
    }

    pub mod view {
        pub mod auth {
            pub mod login;
            pub mod register;
        }
    }

    pub mod error {
        pub mod auth;
    };

    pub mod template;
    pub mod routes;
}

// --------- //
// Structure //
// --------- //

pub struct App {
    // code...
}

// -------------- //
// Implémentation //
// -------------- //

impl App {
    const APP_NAME: &str = "website";
}

impl App {
	#[cfg(feature = "docker")]
	pub async fn register_service(self) -> Result<Self> {
        Ok(self)
    }

	pub async fn launch(self) -> Result<()> {
        Ok(())
    }
}
