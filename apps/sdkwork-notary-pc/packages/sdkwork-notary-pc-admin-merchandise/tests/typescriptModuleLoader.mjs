import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import ts from 'typescript';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

export function loadTypescriptModule(relativePath, stubs = {}) {
  const filename = path.join(packageRoot, relativePath);
  const source = readFileSync(filename, 'utf8');
  const output = ts.transpileModule(source, {
    compilerOptions: {
      esModuleInterop: true,
      module: ts.ModuleKind.CommonJS,
      target: ts.ScriptTarget.ES2022,
    },
    fileName: filename,
  }).outputText;
  const module = { exports: {} };
  const requireStub = (specifier) => {
    if (Object.hasOwn(stubs, specifier)) {
      return stubs[specifier];
    }
    throw new Error(`Unstubbed module ${specifier} while loading ${relativePath}`);
  };
  const execute = new Function('require', 'module', 'exports', output);
  execute(requireStub, module, module.exports);
  return module.exports;
}

export { packageRoot };
