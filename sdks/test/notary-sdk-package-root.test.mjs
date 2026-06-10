import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

const packageRoots = [
  {
    root: "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript",
    familyRoot: "sdks/sdkwork-notary-app-sdk",
    packageName: "@sdkwork/notary-app-sdk",
    clientAlias: "SdkworkNotaryAppClient",
    generatedClient: "SdkworkAppClient",
    generatedFactory: "createGeneratedNotaryAppClient",
    factory: "createNotaryAppClient",
    configType: "SdkworkAppConfig",
    dependencies: [
      "@sdkwork/appbase-app-sdk",
      "@sdkwork/commerce-app-sdk",
      "@sdkwork/drive-app-sdk",
    ],
  },
  {
    root: "sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript",
    familyRoot: "sdks/sdkwork-notary-backend-sdk",
    packageName: "@sdkwork/notary-backend-sdk",
    clientAlias: "SdkworkNotaryBackendClient",
    generatedClient: "SdkworkBackendClient",
    generatedFactory: "createGeneratedNotaryBackendClient",
    factory: "createNotaryBackendClient",
    configType: "SdkworkBackendConfig",
    dependencies: [
      "@sdkwork/appbase-backend-sdk",
      "@sdkwork/commerce-backend-sdk",
      "@sdkwork/drive-backend-sdk",
    ],
  },
];

test("notary TypeScript SDK package roots expose generated clients and composed facades", () => {
  for (const sdkPackage of packageRoots) {
    const packagePath = path.join(sdkPackage.root, "package.json");
    const tsconfigPath = path.join(sdkPackage.root, "tsconfig.json");
    const entryPath = path.join(sdkPackage.root, "src/index.ts");

    assert(existsSync(path.join(workspaceRoot, packagePath)), `${packagePath} must exist`);
    assert(existsSync(path.join(workspaceRoot, tsconfigPath)), `${tsconfigPath} must exist`);
    assert(existsSync(path.join(workspaceRoot, entryPath)), `${entryPath} must exist`);

    const packageJson = readJson(packagePath);
    const componentSpec = readJson(path.join(sdkPackage.familyRoot, "specs/component.spec.json"));
    assert.equal(packageJson.name, sdkPackage.packageName);
    assert.equal(packageJson.type, "module");
    assert.equal(packageJson.main, "./src/index.ts");
    assert.equal(packageJson.module, "./src/index.ts");
    assert.equal(packageJson.types, "./src/index.ts");
    assert.equal(packageJson.exports["."].import, "./src/index.ts");
    assert.equal(packageJson.exports["./generated"].import, "./generated/server-openapi/src/index.ts");
    assert.equal(packageJson.dependencies["@sdkwork/sdk-common"], "^1.0.3");
    for (const dependencyName of sdkPackage.dependencies) {
      assert.equal(
        packageJson.peerDependencies[dependencyName],
        "workspace:*",
        `${sdkPackage.packageName} must declare ${dependencyName} as a dependency SDK peer`,
      );
    }
    assert.equal(
      packageJson.peerDependenciesMeta[sdkPackage.dependencies[0]].optional,
      false,
      `${sdkPackage.packageName} must require appbase dependency SDK peer`,
    );
    assert.equal(
      packageJson.peerDependenciesMeta[sdkPackage.dependencies[2]].optional,
      false,
      `${sdkPackage.packageName} must require drive dependency SDK peer`,
    );
    assert.equal(
      packageJson.peerDependenciesMeta[sdkPackage.dependencies[1]].optional,
      true,
      `${sdkPackage.packageName} must keep commerce dependency SDK peer optional for composed runtime-owned order creation`,
    );

    const tsconfig = readJson(tsconfigPath);
    assert(tsconfig.include.includes("src/**/*.ts"));
    assert(tsconfig.include.includes("composed/**/*.ts"));
    assert(tsconfig.exclude.includes("generated/server-openapi/dist"));

    const entry = readText(entryPath);
    assert(entry.includes(sdkPackage.generatedClient));
    assert(entry.includes(sdkPackage.generatedFactory));
    assert(entry.includes(sdkPackage.clientAlias));
    assert(entry.includes(sdkPackage.factory));
    assert(entry.includes(sdkPackage.configType));
    assert(entry.includes("../generated/server-openapi/src/index"));
    assert(entry.includes("../generated/server-openapi/src/types/common"));
    assert(entry.includes("../composed/index"));
    assert(!entry.includes("fetch("), `${entryPath} must not use raw HTTP`);
    assert(!entry.includes("axios"), `${entryPath} must not use axios`);
    assert(!entry.includes("Authorization"), `${entryPath} must not assemble auth headers`);
    assert(!entry.includes("Access-Token"), `${entryPath} must not assemble access headers`);

    assert(
      componentSpec.contracts.publicExports.includes(`./${path.basename(sdkPackage.root)}/src`),
      `${sdkPackage.packageName} component spec must expose the package root`,
    );
    assert(
      componentSpec.contracts.publicExports.includes(`./${path.basename(sdkPackage.root)}/composed`),
      `${sdkPackage.packageName} component spec must expose the composed facade`,
    );
  }
});

test("notary app composed facade accepts real Drive SDK upload results", () => {
  const source = readText(
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );

  assert(source.includes("resolveDriveUploadNodeId"));
  assert(source.includes("uploadSession"));
  assert(source.includes("uploadItem"));
  assert(source.includes("commerce?: CommerceAppSdkPort"));
  assert(source.includes("driveNodeId"));
  assert(source.includes("nodeId"));
  assert(source.includes("drive.nodes.files.create"));
  assert(source.includes("drive.drive.nodes.files.create"));
  assert(source.includes("driveSpaceType: \"notary\""));
});
