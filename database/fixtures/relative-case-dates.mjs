const DAY_IN_MILLISECONDS = 24 * 60 * 60 * 1000;
const CANONICAL_UTC_ISO_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/u;
const TODAY_CASE_ID = 'fixture-notary-case-completed';
const YESTERDAY_CASE_ID = 'fixture-notary-case-completed-yesterday';

export function resolveFixtureReferenceTime(value, fallback = new Date()) {
  if (value === undefined || value === null || value.trim() === '') {
    if (!(fallback instanceof Date) || Number.isNaN(fallback.getTime())) {
      throw new Error('Fixture reference time fallback must be a valid Date.');
    }
    return new Date(fallback.getTime());
  }

  const normalized = value.trim();
  const referenceTime = new Date(normalized);
  if (
    !CANONICAL_UTC_ISO_PATTERN.test(normalized)
    || Number.isNaN(referenceTime.getTime())
    || referenceTime.toISOString() !== normalized
  ) {
    throw new Error('Fixture reference time must be a canonical UTC ISO timestamp such as 2026-07-11T12:00:00.000Z.');
  }
  return referenceTime;
}

export function buildRelativeCaseDateRefreshSql(engine, referenceTime) {
  if (engine !== 'sqlite' && engine !== 'postgres') {
    throw new Error('Fixture engine must be sqlite or postgres.');
  }
  if (!(referenceTime instanceof Date) || Number.isNaN(referenceTime.getTime())) {
    throw new Error('Fixture reference time must be a valid Date.');
  }

  const today = buildWorkflowTimes(referenceTime, 0);
  const yesterday = buildWorkflowTimes(referenceTime, -1);
  const eventTimes = [
    ['fixture-notary-event-completed-submitted', today.submittedAt],
    ['fixture-notary-event-completed-accepted', today.acceptedAt],
    ['fixture-notary-event-completed', today.completedAt],
    ['fixture-notary-event-completed-yesterday-submitted', yesterday.submittedAt],
    ['fixture-notary-event-completed-yesterday-accepted', yesterday.acceptedAt],
    ['fixture-notary-event-completed-yesterday-completed', yesterday.completedAt],
  ];
  const eventTimestampCases = eventTimes
    .map(([eventId, timestamp]) => `    WHEN '${eventId}' THEN '${timestamp}'`)
    .join('\n');
  const eventIds = eventTimes.map(([eventId]) => `'${eventId}'`).join(', ');
  const begin = engine === 'sqlite' ? 'BEGIN IMMEDIATE;' : 'BEGIN;';

  return `-- SDKWORK DEV/TEST-ONLY relative date refresh.
${begin}

UPDATE notary_case
SET
  submitted_at = CASE id
    WHEN '${TODAY_CASE_ID}' THEN '${today.submittedAt}'
    WHEN '${YESTERDAY_CASE_ID}' THEN '${yesterday.submittedAt}'
    ELSE submitted_at
  END,
  accepted_at = CASE id
    WHEN '${TODAY_CASE_ID}' THEN '${today.acceptedAt}'
    WHEN '${YESTERDAY_CASE_ID}' THEN '${yesterday.acceptedAt}'
    ELSE accepted_at
  END,
  completed_at = CASE id
    WHEN '${TODAY_CASE_ID}' THEN '${today.completedAt}'
    WHEN '${YESTERDAY_CASE_ID}' THEN '${yesterday.completedAt}'
    ELSE completed_at
  END,
  created_at = CASE id
    WHEN '${TODAY_CASE_ID}' THEN '${today.submittedAt}'
    WHEN '${YESTERDAY_CASE_ID}' THEN '${yesterday.submittedAt}'
    ELSE created_at
  END,
  updated_at = CASE id
    WHEN '${TODAY_CASE_ID}' THEN '${today.completedAt}'
    WHEN '${YESTERDAY_CASE_ID}' THEN '${yesterday.completedAt}'
    ELSE updated_at
  END
WHERE tenant_id = '100001'
  AND organization_id = '200001'
  AND id IN ('${TODAY_CASE_ID}', '${YESTERDAY_CASE_ID}');

UPDATE notary_case_event
SET
  occurred_at = CASE id
${eventTimestampCases}
    ELSE occurred_at
  END,
  created_at = CASE id
${eventTimestampCases}
    ELSE created_at
  END
WHERE tenant_id = '100001'
  AND organization_id = '200001'
  AND case_id IN ('${TODAY_CASE_ID}', '${YESTERDAY_CASE_ID}')
  AND id IN (${eventIds});

COMMIT;
`;
}

function buildWorkflowTimes(referenceTime, dayOffset) {
  const startOfReferenceDay = Date.UTC(
    referenceTime.getUTCFullYear(),
    referenceTime.getUTCMonth(),
    referenceTime.getUTCDate(),
  );
  const startOfTargetDay = startOfReferenceDay + dayOffset * DAY_IN_MILLISECONDS;
  return {
    submittedAt: new Date(startOfTargetDay + 9 * 60 * 60 * 1000).toISOString(),
    acceptedAt: new Date(startOfTargetDay + 10 * 60 * 60 * 1000).toISOString(),
    completedAt: new Date(startOfTargetDay + 15 * 60 * 60 * 1000).toISOString(),
  };
}
