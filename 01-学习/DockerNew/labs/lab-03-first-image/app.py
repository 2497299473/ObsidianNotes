from flask import Flask

app = Flask(__name__)

@app.route('/')
def hello():
    return '<h1>Hello from my Docker container!</h1>'

@app.route('/health')
def health():
    return {'status': 'ok'}, 200

if __name__ == '__main__':
    # host='0.0.0.0' 让容器外部可以访问
    app.run(host='0.0.0.0', port=5000)
