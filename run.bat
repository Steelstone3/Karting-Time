@echo off
setlocal enabledelayedexpansion

cargo build --release

.\target\release\karting-time.exe
