#!/bin/bash
solana-test-validator --bpf-program DWDGo2UkBUFZ3VitBfWRBMvRnHr7E2DSh57NK27xMYaB target/deploy/fee_collector.so \
--account Ez3nzG9ofodYCvEmw73XhQ87LWNYVRM2s7diB5tBZPyM fork_accounts/Ez3nzG9ofodYCvEmw73XhQ87LWNYVRM2s7diB5tBZPyM.json \
--account 7e8LRrfeeSGfS2SSVGJMZQLQKzYhkBp8VKtt34uJMR4t fork_accounts/7e8LRrfeeSGfS2SSVGJMZQLQKzYhkBp8VKtt34uJMR4t.json \
--account 4XRXuLFfa2pT56JNtjCsGLpG9yriYt8pr3D2GseYpfkP fork_accounts/4XRXuLFfa2pT56JNtjCsGLpG9yriYt8pr3D2GseYpfkP.json \
--account AdKqXRW51SyZgepKMs2x77kNYMv4CQfsjD7vResES9EQ fork_accounts/AdKqXRW51SyZgepKMs2x77kNYMv4CQfsjD7vResES9EQ.json \
-r
