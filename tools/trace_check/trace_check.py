#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import re
import sys
import argparse

DOC_ID_PATTERN = re.compile(r"doc_id:\s*(doc-[^\s]+)")
SEC_ID_PATTERN = re.compile(r"sec_[a-z0-9]+")
REF_PATTERN = re.compile(r"@hldocs\.ref\s+(doc-[^#]+#sec_[a-z0-9]+)")
TODO_PATTERN = re.compile(r"TODO\(trace\)")

EXCLUDE_DIRS = {".git", "target", "node_modules", "__pycache__"}


def collect_files(root):
    files = []
    for base, dirs, filenames in os.walk(root):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        for f in filenames:
            path = os.path.join(base, f)
            files.append(path)
    return files


def read_file(path):
    try:
        with open(path, "r", encoding="utf-8") as f:
            return f.read()
    except Exception:
        return ""


def extract_doc_ids(text):
    return DOC_ID_PATTERN.findall(text)


def extract_sec_ids(text):
    return SEC_ID_PATTERN.findall(text)


def extract_refs(text):
    return REF_PATTERN.findall(text)


def check(root, mode):
    files = collect_files(root)

    spec_sec_ids = set()
    testspec_sec_ids = set()
    code_refs = set()
    todos = []

    for path in files:
        text = read_file(path)

        if "02_仕様" in path:
            spec_sec_ids.update(extract_sec_ids(text))

        if "03_テスト仕様" in path:
            testspec_sec_ids.update(extract_sec_ids(text))

        if "@hldocs.ref" in text:
            code_refs.update(extract_refs(text))

        if TODO_PATTERN.search(text):
            todos.append(path)

    errors = []
    warnings = []

    # CHECK-001
    for ref in code_refs:
        _, sec = ref.split("#")
        if sec not in spec_sec_ids:
            errors.append(("CHECK-001", ref, "sec_id not found in spec"))

    # CHECK-002
    if todos:
        if mode == "strict":
            for t in todos:
                errors.append(("CHECK-002", t, "TODO(trace) found"))
        else:
            for t in todos:
                warnings.append(("CHECK-002", t, "TODO(trace) found"))

    # CHECK-003
    for sec in testspec_sec_ids:
        if sec not in spec_sec_ids:
            errors.append(("CHECK-003", sec, "testspec sec_id not in spec"))

    # CHECK-004
    implemented_secs = {r.split("#")[1] for r in code_refs}
    for sec in spec_sec_ids:
        if sec not in implemented_secs:
            if mode == "strict":
                errors.append(("CHECK-004", sec, "spec sec_id not implemented"))
            else:
                warnings.append(("CHECK-004", sec, "spec sec_id not implemented"))

    # CHECK-005
    for sec in testspec_sec_ids:
        if sec not in implemented_secs:
            if mode == "strict":
                errors.append(("CHECK-005", sec, "testspec sec_id not tested"))
            else:
                warnings.append(("CHECK-005", sec, "testspec sec_id not tested"))

    return errors, warnings


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".")
    parser.add_argument("--mode", default="transitional", choices=["strict", "transitional"])
    args = parser.parse_args()

    errors, warnings = check(args.root, args.mode)

    if errors:
        print("TRACE CHECK FAIL")
        for e in errors:
            print(f"[{e[0]}] {e[1]} : {e[2]}")
        sys.exit(1)

    if warnings:
        print("TRACE CHECK PASS WITH WARNINGS")
        for w in warnings:
            print(f"[{w[0]}] {w[1]} : {w[2]}")
        sys.exit(0)

    print("TRACE CHECK PASS")
    sys.exit(0)


if __name__ == "__main__":
    main()
