#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const sdkName = "sdkwork-notary-backend-sdk";
const sdkType = "backend";
const apiPrefix = "/backend/v3/api";
const defaultBaseUrl = "http://127.0.0.1:18080";
const fixedSdkVersion = "0.1.0";
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(scriptDir, "..");
const workspaceRoot = path.resolve(sdkRoot, "../..");
const generatorBin = path.resolve(workspaceRoot, "../sdkwork-sdk-generator/bin/sdkgen.js");
const defaultInput = path.resolve(workspaceRoot, "generated/openapi/notary-backend-api.openapi.json");

run(process.argv.slice(2));

function run(argv) {
  const args = parseArgs(argv);
  const input = args.input ? resolveWorkspacePath(args.input) : defaultInput;
  if (!existsSync(input)) {
    fail(`OpenAPI input not found: ${input}`);
  }
  if (!existsSync(generatorBin)) {
    fail(`SDK generator not found: ${generatorBin}`);
  }

  const language = args.language;
  const outputPath = path.join(
    sdkRoot,
    `${sdkName}-${language}`,
    "generated",
    "server-openapi",
  );
  mkdirSync(outputPath, { recursive: true });

  const commandArgs = [
    "generate",
    "--input",
    input,
    "--output",
    outputPath,
    "--name",
    sdkName,
    "--type",
    sdkType,
    "--language",
    language,
    "--base-url",
    args.baseUrl,
    "--api-prefix",
    apiPrefix,
    "--fixed-sdk-version",
    fixedSdkVersion,
    "--sdk-root",
    sdkRoot,
    "--sdk-name",
    sdkName,
    "--package-name",
    `${sdkName}-generated-${language}`,
    "--standard-profile",
    "sdkwork-v3",
    ...args.passthrough
  ];

  const result = spawnSync("node", [generatorBin, ...commandArgs], {
    cwd: sdkRoot,
    stdio: "inherit"
  });
  if (result.error) {
    fail(result.error.message);
  }
  if (typeof result.status === "number" && result.status !== 0) {
    fail(`generator exited with code ${result.status}`);
  }
  if (result.signal) {
    fail(`generator terminated by signal ${result.signal}`);
  }

  writeFileSync(
    path.join(outputPath, "source-openapi.json"),
    `${JSON.stringify(JSON.parse(readFileSync(input, "utf8")), null, 2)}\n`,
    "utf8",
  );
}

function parseArgs(argv) {
  const parsed = {
    input: null,
    language: "typescript",
    baseUrl: defaultBaseUrl,
    passthrough: []
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--input") {
      parsed.input = argv[index + 1] || "";
      index += 1;
      continue;
    }
    if (arg === "--language") {
      parsed.language = argv[index + 1] || "typescript";
      index += 1;
      continue;
    }
    if (arg.startsWith("--language=")) {
      parsed.language = arg.slice("--language=".length);
      continue;
    }
    if (arg === "--base-url") {
      parsed.baseUrl = argv[index + 1] || defaultBaseUrl;
      index += 1;
      continue;
    }
    if (arg === "--") {
      parsed.passthrough.push(...argv.slice(index + 1));
      break;
    }
    parsed.passthrough.push(arg);
  }

  return parsed;
}

function resolveWorkspacePath(value) {
  return path.isAbsolute(value) ? value : path.resolve(workspaceRoot, value);
}

function fail(message) {
  process.stderr.write(`[${sdkName}] ${message}\n`);
  process.exit(1);
}
