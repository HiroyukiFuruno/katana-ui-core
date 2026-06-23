from __future__ import annotations


class PublicOptionRequirement:
    def __init__(self, page: str, source: str, source_token: str, setting: str) -> None:
        self.page = page
        self.source = source
        self.source_token = source_token
        self.setting = setting
