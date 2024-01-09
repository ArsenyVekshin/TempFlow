import random

from flask import Flask

app = Flask(__name__)


app = Flask(__name__)

@app.route('/')
def index():
    return 'Привет! Введите число в URL для получения его квадрата.'

@app.route('/<path:rack>//sens/<int:num>')
def square(rack, num):
    return f'{random.randint(0,100)}'

if __name__ == '__main__':
    app.run()