#!/usr/bin/env node

import process from 'node:process';

import {
  DEFAULT_DEV_PROFILE_ID,
  listHealthSurfaces,
  loadProfile,
  mergeRuntimeEnv,
  REPO_ROOT,
  resolveDevProfileId,
  resolveDefaultAppSdkBaseUrl,
  resolveDefaultBackendSdkBaseUrl,
  resolveGatewayBaseUrl,
  resolveIamDevEnv,
  resolveSurfaceHttpUrl,
} from './lib/notary-topology.mjs';

function parseArgs(argv) {
  const settings = {
    deploymentProfile: 'standalone',
    serviceLayout: 'split-services',
    printEnv: false,
    dryRun: false,
    help: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--help' || arg === '-h') {
      settings.help = true;
      continue;
    }
    if (arg === '--deployment-profile') {
      settings.deploymentProfile = argv[index + 1] ?? settings.deploymentProfile;
      index += 1;
      continue;
    }
    if (arg === '--service-layout') {
      settings.serviceLayout = argv[index + 1] ?? settings.serviceLayout;
      index += 1;
      continue;
    }
    if (arg === '--print-env') {
      settings.printEnv = true;
      continue;
    }
    if (arg === '--dry-run') {
      settings.dryRun = true;
    }
  }

  return settings;
}

function printHelp() {
  console.log(`Usage: node scripts/notary-dev.mjs [options]

Topology-aware Notary dev entry. Loads configs/topology profile env via @sdkwork/app-topology.

This repository ships route/runtime libraries only. Host applications wire
sdkwork-router-notary-* crates and consume the resolved profile env below.

Options:
  --deployment-profile <standalone|cloud>           Default: standalone
  --service-layout <split-services>                 Default: split-services
  --print-env                                       Print merged profile env keys
  --dry-run                                         Print resolved URLs only
  --help, -h
`);
}

function main() {
  const settings = parseArgs(process.argv.slice(2));
  if (settings.help) {
    printHelp();
    return;
  }

  const profileId = resolveDevProfileId(settings.deploymentProfile, settings.serviceLayout);
  const profileEnv = loadProfile(profileId);
  const mergedEnv = mergeRuntimeEnv(process.env, profileEnv, resolveIamDevEnv(process.env));

  const summary = {
    repoRoot: REPO_ROOT,
    profileId,
    deploymentProfile: settings.deploymentProfile,
    defaultDevProfileId: DEFAULT_DEV_PROFILE_ID,
    applicationPublicHttpUrl: resolveSurfaceHttpUrl(profileId, 'application.public-ingress', mergedEnv),
    applicationBackendHttpUrl: resolveSurfaceHttpUrl(profileId, 'application.backend-http', mergedEnv),
    platformApiGatewayHttpUrl: resolveGatewayBaseUrl(mergedEnv, settings.deploymentProfile),
    appSdkBaseUrl: resolveDefaultAppSdkBaseUrl(mergedEnv),
    backendSdkBaseUrl: resolveDefaultBackendSdkBaseUrl(mergedEnv),
    healthSurfaces: listHealthSurfaces(profileId),
  };

  console.log('[sdkwork-notary-dev] topology profile loaded');
  console.log(JSON.stringify(summary, null, 2));

  if (settings.printEnv) {
    const keys = Object.keys(mergedEnv)
      .filter((key) => key.startsWith('SDKWORK_') || key.startsWith('VITE_'))
      .sort();
    for (const key of keys) {
      console.log(`${key}=${mergedEnv[key]}`);
    }
  }

  if (settings.dryRun) {
    return;
  }

  console.log(
    '[sdkwork-notary-dev] host applications must wire route crates and wait for health surfaces before starting clients',
  );
}

main();
