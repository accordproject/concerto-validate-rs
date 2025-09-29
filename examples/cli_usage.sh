#!/bin/bash

# Example usage of concerto-validator CLI

echo "=== Concerto Validator CLI Examples ==="
echo

echo "1. Validate a single file:"
echo "./target/debug/concerto-validator validate --input metamodel.json"
echo

echo "2. Validate multiple files:"
echo "./target/debug/concerto-validator validate --input file1.json --input file2.json --input file3.json"
echo

echo "3. Validate multiple files and stop at first error:"
echo "./target/debug/concerto-validator validate --input file1.json --input file2.json --fail-early"
echo

echo "4. Get help:"
echo "./target/debug/concerto-validator --help"
echo "./target/debug/concerto-validator validate --help"
echo

echo "5. Show version:"
echo "./target/debug/concerto-validator --version"
echo

echo "=== Running actual validation example ==="
echo

# Run an actual validation
echo "Validating metamodel.json:"
./target/debug/concerto-validator validate --input metamodel.json