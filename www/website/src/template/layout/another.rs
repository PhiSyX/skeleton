use web::markup::{html, Markup};

use super::Layout;

// --------- //
// Structure //
// --------- //

#[derive(Default)]
pub struct LayoutDefault {
    extend_layout: Layout,
    // code...
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl web::markup::RenderHTML for LayoutDefault {
    fn render(&self) -> web::markup::MarkupHTML {
        let layout = self.extend_layout.add_meta(html!(
            meta name="description" content="Another skeleton";
            link rel="stylesheets" href="http://xxx.css";
        ));
        layout.render()
    }
}
