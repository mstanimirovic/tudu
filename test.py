#!/usr/bin/env python3
import os
import time
import json
from typing import Any, Dict, Optional

import requests


BASE_URL = os.getenv("BASE_URL", "http://localhost:3000").rstrip("/")

ts = int(time.time())
EMAIL = os.getenv("EMAIL", f"mladen+{ts}@example.com")
PASSWORD = os.getenv("PASSWORD", "P@ssw0rd123!")

CATEGORY_NAME = os.getenv("CATEGORY_NAME", "Work")
TODO_TITLE = os.getenv("TODO_TITLE", "Read emails")
TODO_DESCRIPTION = os.getenv("TODO_DESCRIPTION", "Clean inbox and reply")
TODO_STATUS = os.getenv("TODO_STATUS", "open")


def pretty(obj: Any) -> str:
    return json.dumps(obj, indent=2, ensure_ascii=False)


def request_json(
    method: str,
    path: str,
    *,
    headers: Optional[Dict[str, str]] = None,
    params: Optional[Dict[str, str]] = None,
    body: Optional[Dict[str, Any]] = None,
    timeout: float = 10.0,
) -> Dict[str, Any]:
    url = f"{BASE_URL}{path}"
    h = {"Accept": "application/json"}
    if headers:
        h.update(headers)
    if body is not None:
        h["Content-Type"] = "application/json"

    r = requests.request(method, url, headers=h, params=params, json=body, timeout=timeout)

    print(f"\n==> {method} {path}")
    if params:
        print(f"Params: {params}")
    if body is not None:
        print("Body:")
        print(pretty(body))
    print(f"HTTP: {r.status_code}")

    # Try parse JSON, but don't crash if server returns empty body
    try:
        data = r.json() if r.text.strip() else {}
    except Exception:
        data = {"_raw": r.text}

    print("Response:")
    print(pretty(data))

    if not (200 <= r.status_code <= 299):
        raise SystemExit(f"Request failed: {method} {path} -> {r.status_code}")

    return data


def main() -> None:
    print(f"BASE_URL = {BASE_URL}")
    print(f"EMAIL    = {EMAIL}")

    # 1) POST /register
    register_resp = request_json(
        "POST",
        "/auth/register",
        body={"email": EMAIL, "password": PASSWORD, "name": "Mladen"},
    )

    # 2) POST /login
    login_resp = request_json(
        "POST",
        "/auth/login",
        body={"email": EMAIL, "password": PASSWORD},
    )

    token = login_resp.get("token")
    if not token or not isinstance(token, str):
        raise SystemExit("Could not extract 'token' from /login response.")

    auth_headers = {"Authorization": f"Bearer {token}"}

    # 3) POST /categories
    cat_resp = request_json(
        "POST",
        "/categories",
        headers=auth_headers,
        body={"name": CATEGORY_NAME},
    )

    category_id = cat_resp.get("id")
    if category_id is None:
        raise SystemExit("Could not extract 'id' from /categories response.")

    # 4) POST /todos
    todo_resp = request_json(
        "POST",
        "/todos",
        headers=auth_headers,
        body={
            "title": TODO_TITLE,
            "description": TODO_DESCRIPTION,
            "priority": 1,
            "category_id": category_id,
            "due_at": "2026-02-21T11:33:03.588115Z"
        },
    )

    todo_id = todo_resp.get("id")
    if todo_id is None:
        raise SystemExit("Could not extract 'id' from /todos response.")

    # 5) GET /todos?status=open&q=ea
    list_resp = request_json(
        "GET",
        "/todos",
        headers=auth_headers,
        params={"status": "open", "q": "ea"},
    )

    print(list_resp)

    print("\nAll endpoint tests passed ✅")
    # If your API returns array, you can add extra assertions here:
    # - ensure list_resp is list or contains items, etc.


if __name__ == "__main__":
    main()
