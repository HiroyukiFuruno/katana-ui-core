#!/usr/bin/env python3
from __future__ import annotations

from datetime import datetime


def validate_approval_metadata(approved_by: str, approved_at: str) -> None:
    if not approved_by.strip():
        raise ValueError("approved_by is required")
    if not approved_at.strip():
        raise ValueError("approved_at is required")
    try:
        approved_time = datetime.fromisoformat(approved_at)
    except ValueError as error:
        raise ValueError("approved_at must be ISO-8601 with timezone") from error
    if approved_time.tzinfo is None or approved_time.utcoffset() is None:
        raise ValueError("approved_at must include timezone")
