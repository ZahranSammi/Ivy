import unittest
import os
import json
import sqlite3
from datetime import datetime, timedelta
from server import (
    is_valid_domain,
    is_valid_ip_or_cidr,
    validate_target_against_scope,
    query_graph,
    run_active_scan,
    init_db,
    get_db,
    SCOPE_PATH
)

class TestGraphConSecurity(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        init_db()

    def setUp(self):
        # Reset scope file
        if SCOPE_PATH.exists():
            SCOPE_PATH.unlink()

    def test_domain_validation(self):
        self.assertTrue(is_valid_domain("target.com"))
        self.assertTrue(is_valid_domain("sub.target.com"))
        self.assertTrue(is_valid_domain("*.target.com"))
        self.assertFalse(is_valid_domain("invalid_domain"))
        self.assertFalse(is_valid_domain("target..com"))

    def test_ip_validation(self):
        self.assertTrue(is_valid_ip_or_cidr("192.168.1.1"))
        self.assertTrue(is_valid_ip_or_cidr("192.168.1.0/24"))
        self.assertFalse(is_valid_ip_or_cidr("999.999.999.999"))

    def test_scope_enforcement(self):
        # Create a mock scope.json
        scope_data = {
            "authorized_domains": ["target.com", "*.target.com"],
            "authorized_ips": ["192.168.1.0/24"]
        }
        with open(SCOPE_PATH, "w") as f:
            json.dump(scope_data, f)

        # In-scope
        self.assertTrue(validate_target_against_scope("target.com"))
        self.assertTrue(validate_target_against_scope("sub.target.com"))
        self.assertTrue(validate_target_against_scope("192.168.1.5"))

        # Out-of-scope
        self.assertFalse(validate_target_against_scope("other.com"))
        self.assertFalse(validate_target_against_scope("192.168.2.1"))

    def test_query_graph_write_prevention(self):
        # FR-9: enforce read-only
        res1 = query_graph("dummy-scope", "MATCH (n) RETURN n")
        # Should not be "WRITE_NOT_ALLOWED"
        self.assertNotEqual(res1.get("error"), "WRITE_NOT_ALLOWED")

        res2 = query_graph("dummy-scope", "CREATE (n:Test)")
        self.assertEqual(res2.get("error"), "WRITE_NOT_ALLOWED")

        res3 = query_graph("dummy-scope", "MATCH (n) DETACH DELETE n")
        self.assertEqual(res3.get("error"), "WRITE_NOT_ALLOWED")

    def test_gatekeeper_token_validation(self):
        # FR-7: run_active_scan with invalid/expired/consumed token
        # Insert a dummy active token
        conn = get_db()
        cursor = conn.cursor()
        
        # Test 1: Mismatch/Invalid token
        res = run_active_scan("invalid-token", "dummy-job")
        self.assertEqual(res.get("error"), "INVALID_OR_MISSING_APPROVAL")

        # Test 2: Active token
        token = "test-token-active"
        job_id = "test-job-1"
        req_id = "test-req-1"
        now = datetime.utcnow()
        expires = now + timedelta(minutes=5)
        
        cursor.execute("INSERT OR REPLACE INTO approvals (approval_request_id, job_id, targets_json, test_types_json, status, created_at) VALUES (?, ?, ?, ?, ?, ?)",
                       (req_id, job_id, '["target.com"]', '["sqli"]', 'APPROVED', now.isoformat()))
        cursor.execute("INSERT OR REPLACE INTO tokens (token, job_id, approval_request_id, status, issued_at, expires_at) VALUES (?, ?, ?, ?, ?, ?)",
                       (token, job_id, req_id, "ACTIVE", now.isoformat(), expires.isoformat()))
        conn.commit()
        
        # Scope is required for active target validation
        scope_data = {
            "authorized_domains": ["target.com"],
            "authorized_ips": []
        }
        with open(SCOPE_PATH, "w") as f:
            json.dump(scope_data, f)

        res = run_active_scan(token, job_id)
        self.assertEqual(res.get("status"), "success")

        # Test 3: Token is now consumed
        res_retry = run_active_scan(token, job_id)
        self.assertEqual(res_retry.get("error"), "INVALID_OR_MISSING_APPROVAL")

        conn.close()

if __name__ == "__main__":
    unittest.main()
