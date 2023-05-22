// NOTE: `@project` s'agit du nom de projet.
//
//       Dans le cadre d'un projet node :
//          `@project/{module}` -> `packages/npm/{module}`
//
// NOTE: '~' -> accès aux fichiers de l'application courante.
//
//       `~/app` -> `apps/{app}/app.ts`
//       `~/*`   -> `apps/{app}/src/*`

import config, { find_config_from, parse_json } from "@project/config";
import Option from "@project/std";

import { APP_NAME } from "~/app";

// ---- //
// Type //
// ---- //

type config_raw = {
  config_name: string;
  [p: string]: Option<any>;
};

// -------------- //
// Implémentation //
// -------------- //

class Config {
  config_name: string;

  field_of_config: string;
  non_null_field: Option<string>;

  static fetch(): config.Result<{ [p: string]: unknown }> {
    return find_config_from(APP_NAME, parse_json).map(
      (config_raw) => new Config(config_raw)
    );
  }

  constructor(config_raw: config_raw) {
    this.config_name = config_raw.config_name;

    // NOTE: sur les valeurs des propriétés, `parse_json` retourne Object<T>

    this.field_of_config = config_raw
      .get("field_of_config")
      .expect("Le champ `field_of_config`, requis.");

    this.non_null_field = config_raw.get("non_null_field");
  }
}

// ------ //
// Export //
// ------ //

export { Config };
