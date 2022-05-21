from re import sub
from typing import Literal
import subprocess


def color(string: str, mode: Literal["green", "red", "yellow", "cyan"]):
    colors = {
        "red": "\033[31m",
        "green": "\033[32m",
        "yellow": "\033[33m",
        "cyan": "\033[36m"}
    return f'{colors[mode]}{string}\033[0m'


def command(command: str) -> str:
    # deepcode ignore CommandInjection: <please specify a reason of ignoring
    # this>
    return subprocess.getoutput(command)


def kebab(s):
    return '-'.join(sub(r"[\s\_\-]+",
                        " ",
                        sub(r"[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+",
                            lambda mo: f' {mo.group(0).lower()}',
                            s)).split())


info = color("Describe the name of the branch.", "cyan")
sample_branch = ",\n          ".join([
    color(
        "feature/implement-sample-system",
        "green"),
    color(
        "feature/support-xxx-for-xxx",
        "green"),
    color(
        "feature/fix-xxx-bugs",
        "green"),
])

branch_name = input(f"{info}\n\
 Example: {sample_branch}\n\
          -----------------------------------\n\
  Create: feature/")

branch_name = kebab(branch_name)

confirmation = color(
    f"Really create {color(branch_name, 'green')}? Yes(y)/No(n): ",
    "cyan")
answer = input(confirmation).lower()

if answer in ["yes", "y"]:
    command(f"git checkout -b feature/{branch_name}")
    print(f"Created {branch_name}.")
else:
    print(color("Aborted.", "red"))
