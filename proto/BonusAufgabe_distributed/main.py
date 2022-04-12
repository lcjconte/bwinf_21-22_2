from typing import List
import flask
import queue
from dataclasses import dataclass

app = flask.Flask(__name__)

@dataclass
class TInput:
    n: int
    k: int
    m: int
    nums: List[int]
    @classmethod
    def read_from(cls, fname):
        

@app.route('/')
def hello_world():
    return 'Hello, World!'

@app.route("/tinput", methods=["GET"])
def tinput():
    return "Not Implemented"

message_qeues = [] 

def add_listener():
    q = queue.Queue(maxsize=4)
    message_qeues.append(q)
    return q

def send_all(msg):
    for i in reversed(range(len(message_qeues))):
        try:
            message_qeues[i].put_nowait(msg)
        except queue.Full:
            del message_qeues[i]

@app.route('/listen', methods=['GET'])
def listen():
    def stream():
        messages = add_listener()
        while True:
            msg = messages.get()
            yield msg
    return flask.Response(stream(), mimetype='text/event-stream')

if __name__ == "__main__":
    print("Awaiting connections")