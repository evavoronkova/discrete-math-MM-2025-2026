#!/usr/bin/env bash
# запуск всех экспериментов последовательно
# флаг -e: упасть на первой ошибке, чтобы не маскировать проблемы
# флаг -u: ругаться на необъявленные переменные
set -eu

echo "===== run_analysis ====="
python -m experiments.run_analysis

echo "===== run_robustness ====="
python -m experiments.run_robustness

echo "===== run_landmarks ====="
python -m experiments.run_landmarks

echo "Все эксперименты завершены. Результаты в results/"
