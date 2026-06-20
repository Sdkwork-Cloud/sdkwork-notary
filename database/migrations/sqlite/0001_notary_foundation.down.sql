-- sdkwork:migration
-- id: 0001_notary_foundation
-- engine: sqlite
-- module: notary
-- purpose: Drop notary foundation tables and indexes
-- reversible: true
-- transactional: true
-- lock: lightweight
-- contract_version: 1.0.0

DROP INDEX IF EXISTS idx_notary_case_event_case_time;
DROP INDEX IF EXISTS idx_notary_case_assignment_case;
DROP INDEX IF EXISTS idx_notary_case_assignment_member;
DROP INDEX IF EXISTS idx_notary_party_identity_hash;
DROP INDEX IF EXISTS idx_notary_party_sku_status;
DROP INDEX IF EXISTS idx_notary_party_order;
DROP INDEX IF EXISTS idx_notary_party_case_sort;
DROP INDEX IF EXISTS idx_notary_case_drive_folder;
DROP INDEX IF EXISTS idx_notary_case_sku_status;
DROP INDEX IF EXISTS idx_notary_case_primary_notary;
DROP INDEX IF EXISTS idx_notary_case_org_status_updated;
DROP INDEX IF EXISTS idx_notary_organization_profile_status;

DROP TABLE IF EXISTS notary_case_event;
DROP TABLE IF EXISTS notary_case_assignment;
DROP TABLE IF EXISTS notary_party;
DROP TABLE IF EXISTS notary_case;
DROP TABLE IF EXISTS notary_organization_profile;
