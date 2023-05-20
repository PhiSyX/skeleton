use web::markup::{html, MarkupHTML, RenderHTML};

// --------- //
// Structure //
// --------- //

pub struct AuthLoginView<'ctx> {
    // code...
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'ctx> RenderHTML for AuthLoginView<'ctx> {
    fn render(&self) -> MarkupHTML {
        html! {
        form
            action="/auth/login"
            method="POST"
            id="auth-login-form"
         {
            div {
                label for="username" { "Pseudonyme : " }
                input name="username" type="text" required id="username";
            }

            div {
                label for="password" { "Mot de passe : " }
                input id="password" name="password" type="password" required;
            }

            div {
                button type="submit" { "Se connecter" }
            }
        }
        }
    }
}
