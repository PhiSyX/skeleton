pub mod layout {
    mod another;
    mod default;

    pub use self::{another::LayoutDefault, default::{LayoutAnother};

    use web::markup::{html, MarkupHTML, RenderHTML, DOCTYPE};

    // --------- //
    // Structure //
    // --------- //

    pub struct Layout {
        // code...
    }

    // -------------- //
    // Implémentation // -> Interface
    // -------------- //

    impl RenderHTML for Layout {
        fn render(&self) -> web::markup::MarkupHTML {
            let title = self
                .title
                .as_ref()
                .map(|title| format!("{title} | Skeleton"))
                .unwrap_or("Skeleton".into());

            let metadata = &self.metadata.take();
            let styles = &self.styles.take();
            let scripts = &self.scripts.take();

            let body = self.body.take().unwrap_or(html!());

            html! {
            (DOCTYPE)
            html lang="fr" data-js="off" {
            head {
                meta charset="UTF-8";
                title { (title) }
                meta name="viewport" content="width=device-width,initial-scale=1.0";
                @for meta in metadata { (meta) }
                @for style in styles { (style) }
            }
            body {
                div id="app" {
                    (body)
                }
                @for script in scripts { (script) }
            }
            }
            }
        }
    }
}
