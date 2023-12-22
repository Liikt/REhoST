#!/usr/bin/env python3

from flask import Flask, jsonify, request

app = Flask(__name__)


@app.route("/<rip>/", methods=["POST", "GET"])
def set_value(rip):
    ret = {"status": "ok"}
    print(request.method)
    if request.method == "GET":
        ret["value"] = 0xabad1dea
    return jsonify(ret)


if __name__ == "__main__":
    app.run()
