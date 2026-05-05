#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""HLDocS traceability checker.

This tool validates references between HLDocS documents and Rust/Python test or
implementation files.

Notes:
- Documentation files may contain illustrative `@hldocs.ref` examples.
- Those examples must not be treated as implementation references.
"""

import argparse
import os
import re
import sys
from pathlib import Path

DOC_ID_PATTERN = re.compile(r"doc_id:\s*(doc-[^\s]+)")
SEC_ID_PATTERN = re.compile(r"sec_[a-z0-9]+")
REF_PATTERN = re.compile(r"@hldocs\.ref\s+(doc-[^#\s]+#sec_[a-z0-9]+)")
TODO_PATTERN = re.compile(r"TODO\(trace\)")

EXCLUDE_DIRS = {".git", "target", "node_modules", "__pycache__"}
CODE_EXTENSIONS = {".rs", ".py", ".toml", ".yml", ".yaml"}


def normalize_path(path):
    return Path(path).as_posix()


def is_doc_path(path):
    normalized = normalize_path(path)
    return "/docs/" in f"/{normalized}" or normalized.startswith("docs/")


def is_spec_path(path):
    normalized = normalize_path(path)
    return "docs/ja-JP/02_仕様/" in normalized


def is_testspec_path(path):
    normalized = normalize_path(path)
    return "docs/ja-JP/03_テスト仕様/" in normalized


def is_code_or_test_path(path):
    normalized = normalize_path(path)
    suffix = Path(path).suffix

    if is_doc_path(path):
        return False

    if suffix not in CODE_EXTENSIONS:
        return False

    return (
        normalized.startswith("crates/")
        or normalized.startswith("tests/")
        or normalized.startswith("tools/")
    )


def collect_files(root):
    files = []
    for base, dirs, filenames in os.walk(root):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        for filename in filenames:
            files.append(os.path.join(base, filename))
    return files


def read_file(path):
    try:
        with open(path, "r", encoding="utf-8") as file:
            return file.read()
    except Exception:
        return ""


def extract_sec_ids(text):
    return SEC_ID_PATTERN.findall(text)


def extract_refs(text):
    return REF_PATTERN.findall(text)


def to_relative(path, root):
    try:
        return os.path.relpath(path, root)
    except ValueError:
        return path


def check(root, mode):
    root = os.path.abspath(root)
    files = collect_files(root)

    spec_sec_ids = set()
    testspec_sec_ids = set()
    code_refs = set()
    todos = []

    for path in files:
        rel_path = normalize_path(to_relative(path, root))
        text = read_file(path)

        if is_spec_path(rel_path):
            spec_sec_ids.update(extract_sec_ids(text))

        if is_testspec_path(rel_path):
            testspec_sec_ids.update(extract_sec_ids(text))

        if is_code_or_test_path(rel_path):
            code_refs.update(extract_refs(text))
            if TODO_PATTERN.search(text):
                todos.append(rel_path)

    errors = []
    warnings = []

    # CHECK-001: code/test refs must point to an existing spec sec_id.
    for ref in sorted(code_refs):
        _, sec = ref.split("#", 1)
        if sec not in spec_sec_ids:
            errors.append(("CHECK-001", ref, "sec_id not found in spec"))

    # CHECK-002: TODO(trace) must not remain in strict mode.
    for todo_path in sorted(todos):
        item = ("CHECK-002", todo_path, "TODO(trace) found")
        if mode == "strict":
            errors.append(item)
        else:
            warnings.append(item)

    # CHECK-003: testspec sec_id must exist in spec.
    for sec in sorted(testspec_sec_ids):
        if sec not in spec_sec_ids:
            errors.append(("CHECK-003", sec, "testspec sec_id not in spec"))

    referenced_secs = {ref.split("#", 1)[1] for ref in code_refs}

    # CHECK-004: spec sec_id should be implemented.
    for sec in sorted(spec_sec_ids):
        if sec not in referenced_secs:
            item = ("CHECK-004", sec, "spec sec_id not implemented")
            if mode == "strict":
                errors.append(item)
            else:
                warnings.append(item)

    # CHECK-005: testspec sec_id should be tested.
    for sec in sorted(testspec_sec_ids):
        if sec not in referenced_secs:
            item = ("CHECK-005", sec, "testspec sec_id not tested")
            if mode == "strict":
                errors.append(item)
            else:
                warnings.append(item)

    return errors, warnings


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".")
    parser.add_argument(
        "--mode",
        default="transitional",
        choices=["strict", "transitional"],
    )
    args = parser.parse_args()

    errors, warnings = check(args.root, args.mode)

    if errors:
        print("TRACE CHECK FAIL")
        for check_id, target, reason in errors:
            print(f"[{check_id}] {target} : {reason}")
        sys.exit(1)

    if warnings:
        print("TRACE CHECK PASS WITH WARNINGS")
        for check_id, target, reason in warnings:
            print(f"[{check_id}] {target} : {reason}")
        sys.exit(0)

    print("TRACE CHECK PASS")
    sys.exit(0)


if __name__ == "__main__":
    main()
