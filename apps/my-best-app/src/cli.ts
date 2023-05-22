// NOTE: `@project` s'agit du nom de projet.
//
//       Dans le cadre d'un projet node :
//          `@project/{module}` -> `packages/npm/{module}`
//
// NOTE: '~' -> accès aux fichiers de l'application courante.
//
//       `~/app` -> `apps/{app}/app.ts`
//       `~/*`   -> `apps/{app}/src/*`

import cli, { parse_args } from "@project/cli";

// ---- //
// Type //
// ---- //

type cli_raw = { name: string };

// -------------- //
// Implémentation //
// -------------- //

class CLI {
  name: string;

  /**
   * Analyse les arguments de la CLI en respectant un format précis et les
   * retourne.
   */
  static parse(): cli.Result<CLI> {
    return parse_args({
      // Drapeau --name ou -n
      name: {
        type: "string",
        short: "n",
        default: "world",
      },
    }).map((args_raw: cli_raw) => new CLI(args_raw));
  }

  constructor(args_raw: cli_raw) {
    this.name = args_raw.name;
  }
}

// ------ //
// Export //
// ------ //

export { CLI };
