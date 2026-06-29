import { existsSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
export const workspaceRoot = path.resolve(testDir, "..", "..", "..");

export const notaryPcRoot = path.resolve(workspaceRoot, "apps", "sdkwork-notary-pc");

const defaultImPcRoot = path.resolve(workspaceRoot, "..", "sdkwork-im", "apps", "sdkwork-im-pc");

export const imPcRoot = process.env.SDKWORK_IM_PC_ROOT
  ? path.resolve(process.env.SDKWORK_IM_PC_ROOT)
  : defaultImPcRoot;

export const imPcAvailable = existsSync(path.join(imPcRoot, "package.json"));

export function imPcTest(name, fn) {
  if (!imPcAvailable) {
    test.skip(
      name,
      () => {},
      `skipped: IM PC app root not found at ${imPcRoot}; set SDKWORK_IM_PC_ROOT to run integration checks`,
    );
    return;
  }
  test(name, fn);
}
