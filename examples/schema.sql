-- ARE Audit Pipeline: Database Schema & State Machine
-- -----------------------------------------------------------------------------

-- 1. Property Status Enum
-- -----------------------------------------------------------------------------
-- This enum acts as the state machine protocol for the pipeline.
-- Each module (Harvester, Auditor, Studio, Nexus) reads one input status 
-- and writes one output status.
CREATE TYPE property_status AS ENUM (
    'ingested',    -- Raw data collected by The Harvester
    'sanitized',   -- PII removed, GDPR-safe context created
    'audited',     -- Compliance check (PostGIS) complete
    'restricted',  -- Non-compliant: Red Natura 2000 or zoning violation
    'media_ready', -- The Studio processed imagery (OpenCV/Vertex AI)
    'published',   -- Delivered via The Nexus to dashboard/CRM
    'error'        -- Pipeline halted, needs manual inspection
);

-- 2. Main Properties Table
-- -----------------------------------------------------------------------------
CREATE TABLE properties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    raw_inquiry TEXT,
    sanitized_inquiry TEXT,
    status property_status NOT NULL DEFAULT 'ingested',
    compliance_result JSONB, -- Stores the output from PostGIS/Auditor
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Atomic Status Transition Function
-- -----------------------------------------------------------------------------
-- This function ensures that status updates are non-destructive and strictly
-- follow the expected sequence. Returns FALSE if the expected current status 
-- doesn't match — preventing silent data corruption or race conditions.
CREATE OR REPLACE FUNCTION advance_status(
    p_id UUID,
    p_expected property_status,
    p_next property_status
) RETURNS BOOLEAN AS $$
DECLARE 
    updated_rows INT;
BEGIN
    UPDATE properties
    SET 
        status = p_next, 
        updated_at = NOW()
    WHERE 
        id = p_id 
        AND status = p_expected;
        
    GET DIAGNOSTICS updated_rows = ROW_COUNT;
    RETURN updated_rows > 0;
END;
$$ LANGUAGE plpgsql;

-- --- Usage Example (Pseudo-code) ---
-- SELECT advance_status('uuid-here', 'sanitized', 'audited'); 
-- -> returns TRUE if the auditor successfully picks up a sanitized record.
