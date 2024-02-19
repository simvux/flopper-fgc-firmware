#!/bin/env sh

cargo build && sudo espflash flash --monitor target/risc*/debug/test
