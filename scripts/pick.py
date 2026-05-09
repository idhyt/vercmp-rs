import json
import sys
from pathlib import Path


def print_ecosystem(path: Path):
    ecos = dict()
    for file in path.rglob("*.json"):
        data = json.loads(file.read_text())
        name = data["type"]
        name = name[0].upper() + name[1:].lower()
        # simple replace chrome-extension and vscode-extension
        name = name.replace("-ext", "Ext")
        assert name not in ecos
        # assert data["description"].startswith(data["type_name"]), (
        #     f"{data['description']} does not start with {data['type_name']}"
        # )
        ecos[name] = {
            "type_name": data["type_name"],
            "description": data["description"],
            "examples": data["examples"],
        }
    ecos = dict(sorted(ecos.items(), key=lambda x: x[0][0].lower()))
    # print(json.dumps(ecos, indent=2))
    print("""
/// PURL Type Enum（Based on Package URL）
///
/// reference: https://github.com/package-url/purl-spec/tree/main/types
pub enum PurlType {""")
    for k, v in ecos.items():
        print(
            """
    /// description: {}
    /// examples: {}
    {},""".format(v["description"], v["examples"], k)
        )
    print("}\n")


if __name__ == "__main__":
    print_ecosystem(Path(sys.argv[1]))
