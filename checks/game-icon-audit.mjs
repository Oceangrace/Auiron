import { GameIconPlan } from './game-icon-plan.mjs';

const plan = new GameIconPlan(new URL('../', import.meta.url));
const downloads = plan.downloads();
console.log(
  `Game icon references verified (${downloads.size} CDN images; ${plan.unavailable().length} unavailable paths).`,
);
