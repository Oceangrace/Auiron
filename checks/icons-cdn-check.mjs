import { GameIconPlan } from './game-icon-plan.mjs';

class IconsCdnCheck {
  async probe(name) {
    try {
      const response = await fetch(
        'https://assets.playnccdn.com/static-aion2-gamedata/resources/' + name + '.png',
        { method: 'HEAD', redirect: 'error', signal: AbortSignal.timeout(10000) },
      );

      return response.status;
    } catch (error) {
      return String(error);
    }
  }

  async run() {
    const plan = new GameIconPlan(new URL('../', import.meta.url));
    const names = [...plan.downloads().keys()];
    const pending = names.values();
    const failures = [];

    await Promise.all(
      Array.from({ length: 4 }, async () => {
        for (const name of pending) {
          const status = await this.probe(name);

          if (status !== 200) {
            failures.push({ name, status });
          }
        }
      }),
    );

    console.log(`CDN: ${names.length - failures.length}/${names.length} images available.`);

    for (const failure of failures) {
      console.error(failure.name, failure.status);
    }

    process.exitCode = failures.length === 0 ? 0 : 1;
  }
}

await new IconsCdnCheck().run();
