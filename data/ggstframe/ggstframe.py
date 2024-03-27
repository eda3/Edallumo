import json
import requests

# APIのURL
url = "https://dustloop.com/wiki/api.php?action=cargoquery&format=json&limit=100&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.damage%2C%20MoveData_GGST.guard%2C%20MoveData_GGST.invuln%2C%20MoveData_GGST.startup%2C%20MoveData_GGST.active%2C%20MoveData_GGST.recovery%2C%20MoveData_GGST.onHit%2C%20MoveData_GGST.onBlock%2C%20MoveData_GGST.level%2C%20MoveData_GGST.riscGain%2C%20MoveData_GGST.prorate%2C%20MoveData_GGST.counter&where=chara%3D%22A.B.A%22"

# APIからデータを取得
response = requests.get(url)

# レスポンスのステータスコードをチェック
if response.status_code == 200:
    # JSON形式のデータを取得
    data = response.json()

    # 変換後のフォーマット
    formatted_data = []

    # データの変換
    for item in data["cargoquery"]:

        _input = item["title"].get("input") if item["title"].get("input") is not None else "-"
        _name = item["title"].get("name") if item["title"].get("name") is not None else "-"
        _damage = item["title"].get("damage") if item["title"].get("damage") is not None else "-"
        _guard = item["title"].get("guard") if item["title"].get("guard") is not None else "-"
        _startup = item["title"].get("startup") if item["title"].get("startup") is not None else "-"
        _active = item["title"].get("active") if item["title"].get("active") is not None else "-"
        _recovery = item["title"].get("recovery") if item["title"].get("recovery") is not None else "-"
        _hit = item["title"].get("hit") if item["title"].get("hit") is not None else "-"
        _block = item["title"].get("block") if item["title"].get("block") is not None else "-"
        _level = item["title"].get("level") if item["title"].get("level") is not None else "-"
        _counter = item["title"].get("counter") if item["title"].get("counter") is not None else "-"
        _scaling = item["title"].get("scaling") if item["title"].get("scaling") is not None else "-"
        _riscgain = item["title"].get("riscgain") if item["title"].get("riscgain") is not None else "-"
        _invincibility = item["title"].get("invincibility") if item["title"].get("invincibility") is not None else "none"

        if _input[-1] == "H":
            _input += "S"

        _input = _input.replace("j.", "j")
        _input = _input.replace("c.S", "近S")
        _input = _input.replace("f.S", "遠S")

        new_item = {
            "input": _input,
            "name": _name,
            "damage": _damage,
            "guard": _guard,
            "startup": _startup,
            "active": _active,
            "recovery": _recovery,
            "hit": _hit,
            "block": _block,
            "level": _level,
            "counter": _counter,
            "scaling": _scaling,
            "riscgain": _riscgain,
            "invincibility": _invincibility
        }

        """
        new_item = {
            "input": item["title"].get("input"),
            "name": item["title"].get("name"),
            "damage": item["title"].get("damage"),
            "guard": item["title"].get("guard"),
            "startup": item["title"].get("startup"),
            "active": item["title"].get("active"),
            "recovery": item["title"].get("recovery"),
            "hit": item["title"].get("onHit"),
            "block": item["title"].get("onBlock"),
            "level": item["title"].get("level"),
            "counter": item["title"].get("counter"),
            "scaling": item["title"].get("prorate"),
            "riscgain": item["title"].get("riscGain"),
            "invincibility": item["title"].get("invuln")
        }
        """
        formatted_data.append(new_item)

    # JSON形式に変換して出力
    formatted_json = json.dumps(formatted_data, indent=4)
    # ファイル出力
    with open("ABA.json", "w") as outfile:
        outfile.write(formatted_json)
else:
    # エラーメッセージを表示
    print("Failed to retrieve data from the API")
