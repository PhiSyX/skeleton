// NOTE: '~' -> accès au fichier de l'application courante.
//
//       `~/app` -> `apps/{app}/app.ts`
//       `~/*`   -> `apps/{app}/src/*`

import { APP_NAME, hello, Config, CLI, ENV } from "~/app";

// ---- //
// Main //
// ---- //

function main(env: typeof ENV, cli: typeof CLI) {
  let vars = env
    .fetch()
    .expect(`Les variables d'environnement de l'application ${APP_NAME}`);

  let args = cli
    .parse()
    .expect(`Les arguments de CLI de l'application ${APP_NAME}`);

  let config = Config.fetch().expect(
    `La configuration de l'application ${APP_NAME}`
  );

  let output = hello(args.name);

  console.log({ vars, args, config }, output);
}

// --------- //
// Execution //
// --------- //

main(ENV, CLI);
