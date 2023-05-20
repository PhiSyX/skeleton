use web::markup::{html, MarkupHTML, RenderHTML};

// --------- //
// Structure //
// --------- //

pub struct AuthRegisterView<'ctx> {
    // code...
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'ctx> RenderHTML for AuthRegisterView<'ctx> {
    fn render(&self) -> MarkupHTML {
        html! {
        form
            action="/auth/register"
            method="POST"
            id="auth-register-form"
         {
            div {
                label for="username" { "Pseudonyme : " }
                input name="username" type="text" required id="username";
            }

            div {
                label for="email" { "Adresse mail : " }
                input id="email" name="email" type="email" required;
            }

            div {
                label for="password" { "Mot de passe : " }
                input id="password" name="password" type="password" required;
            }

            div {
                label for="password_confirmation" { "Confirmer le mot de passe : " }
                input id="password_confirmation" name="password_confirmation" type="password" required;
            }

            div {
                button type="submit" { "Se connecter" }
            }
        }
        }
    }
}
