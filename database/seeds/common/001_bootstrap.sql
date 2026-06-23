-- Notary bootstrap seed (common)
-- Domain-owned rows are created through runtime workflows (organization profile open,
-- case submission, party binding). This seed only validates database connectivity.
SELECT 1 AS notary_bootstrap_ready;
