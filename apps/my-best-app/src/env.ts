// NOTE: `@project` s'agit du nom de projet.
//
//       Dans le cadre d'un projet node :
//          `@project/{module}` -> `packages/npm/{module}`
//
// NOTE: '~' -> accès aux fichiers de l'application courante.
//
//       `~/app` -> `apps/{app}/app.ts`
//       `~/*`   -> `apps/{app}/src/*`

import env, { parse_vars_from } from "@project/env";

import { APP_NAME } from "~/app";

// ---- //
// Type //
// ---- //

type env_raw = { APP_SECRET: string };

// -------------- //
// Implémentation //
// -------------- //

class ENV {
  applications_secret_key: string;

  /**
   * Récupère les variables d'environnement de l'application.
   */
  static fetch(): env.Result<ENV> {
    parse_vars_from(APP_NAME).map((vars_raw) => new ENV(vars_raw));
  }

  constructor(vars_raw: env_raw) {
    this.applications_secret_key = vars_raw.APP_SECRET;
  }
}

// ------ //
// Export //
// ------ //

export { ENV };
