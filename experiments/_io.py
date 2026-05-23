# вспомогательные функции для записи отчётов и создания директорий
import os
from contextlib import contextmanager

def ensure_dir(path: str) -> str:
    """Создаёт директорию (включая родительские), возвращает её путь."""
    os.makedirs(path, exist_ok=True)
    return path

def graph_output_dir(root: str, graph_name: str) -> str:
    """Возвращает (и создаёт) персональную директорию для одного графа."""
    return ensure_dir(os.path.join(root, graph_name))

@contextmanager
def open_report(path: str):
    """Открывает файл для записи отчёта, создавая родительские директории."""
    ensure_dir(os.path.dirname(path))
    f = open(path, "w", encoding="utf-8")
    try:
        yield f
    finally:
        f.close()

def log(report, msg: str = "") -> None:
    """Пишет строку и в stdout, и в файл отчёта."""
    print(msg)
    report.write(msg + "\n")
    report.flush()
