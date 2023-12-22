#!/usr/bin/env python3

from flask import Flask, jsonify, request

app = Flask(__name__)


@app.route("/<rip>/", methods=["POST", "GET"])
def set_value(rip):
    try:
        base = 16 if rip.startswith("0x") else 10
        rip = int(rip, base)
    except ValueError:
        return jsonify({"status": "error", "msg": "RIP is not a number"})

    ret = {"status": "ok"}

    if request.method == "GET":
        ret["value"] = 0xabad1dea
    elif request.method == "POST" and request.is_json:
        data = request.get_json()

        if "value" not in data:
            ret["status"] = "error"
            ret["msg"] = "Please include a 'value' key in the json"
        value = data["value"]

        if isinstance(value, str):
            try:
                base = 16 if value.startswith("0x") else 10
                value = int(value, base)
            except ValueError:
                ret["status"] = "error"
                ret["msg"] = "Please provide a number in hex or decimal as a value"
                return jsonify(ret)

        ret["msg"] = f"0x{value:x} has been put at address 0x{rip:x}"
    else:
        ret["status"] = "error"
        ret["msg"] = "No json found in POST request"

    return jsonify(ret)


if __name__ == "__main__":
    app.run()
