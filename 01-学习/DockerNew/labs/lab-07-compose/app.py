from flask import Flask, request, jsonify
import psycopg2
import redis
import os

app = Flask(__name__)

# 从环境变量读取配置（对应 DockerNew 07 章：环境变量管理）
DB_HOST = os.getenv('DB_HOST', 'db')
DB_USER = os.getenv('DB_USER', 'app')
DB_PASSWORD = os.getenv('DB_PASSWORD', 'devpass')
DB_NAME = os.getenv('DB_NAME', 'appdb')
REDIS_URL = os.getenv('REDIS_URL', 'redis://cache:6379')

# 连接 Redis
cache = redis.from_url(REDIS_URL, decode_responses=True)


def get_db():
    """获取 PostgreSQL 连接"""
    return psycopg2.connect(
        host=DB_HOST, user=DB_USER,
        password=DB_PASSWORD, dbname=DB_NAME
    )


@app.route('/health')
def health():
    """健康检查端点（对应 DockerNew 07 章：healthcheck）"""
    try:
        conn = get_db()
        conn.close()
        cache.ping()
        return jsonify({'status': 'ok', 'db': 'connected', 'redis': 'connected'}), 200
    except Exception as e:
        return jsonify({'status': 'error', 'detail': str(e)}), 503


@app.route('/api/todos', methods=['GET'])
def get_todos():
    """列出所有 TODO"""
    conn = get_db()
    cur = conn.cursor()
    cur.execute('SELECT id, title, done FROM todos ORDER BY id')
    rows = cur.fetchall()
    cur.close()
    conn.close()
    return jsonify([{'id': r[0], 'title': r[1], 'done': r[2]} for r in rows])


@app.route('/api/todos', methods=['POST'])
def create_todo():
    """创建 TODO"""
    data = request.get_json() or {}
    title = data.get('title', 'New Todo')
    done = data.get('done', False)
    conn = get_db()
    cur = conn.cursor()
    cur.execute(
        'INSERT INTO todos (title, done) VALUES (%s, %s) RETURNING id',
        (title, done)
    )
    todo_id = cur.fetchone()[0]
    conn.commit()
    cur.close()
    conn.close()
    return jsonify({'id': todo_id, 'title': title, 'done': done}), 201


@app.route('/api/todos/<int:tid>', methods=['GET'])
def get_todo(tid):
    """获取单个 TODO"""
    conn = get_db()
    cur = conn.cursor()
    cur.execute('SELECT id, title, done FROM todos WHERE id = %s', (tid,))
    row = cur.fetchone()
    cur.close()
    conn.close()
    if not row:
        return jsonify({'error': 'not found'}), 404
    return jsonify({'id': row[0], 'title': row[1], 'done': row[2]})


@app.route('/api/todos/<int:tid>', methods=['PUT'])
def update_todo(tid):
    """更新 TODO"""
    data = request.get_json() or {}
    title = data.get('title')
    done = data.get('done')
    conn = get_db()
    cur = conn.cursor()
    if title is not None:
        cur.execute('UPDATE todos SET title = %s WHERE id = %s', (title, tid))
    if done is not None:
        cur.execute('UPDATE todos SET done = %s WHERE id = %s', (done, tid))
    conn.commit()
    cur.close()
    conn.close()
    return jsonify({'id': tid, 'title': title, 'done': done})


@app.route('/api/todos/<int:tid>', methods=['DELETE'])
def delete_todo(tid):
    """删除 TODO"""
    conn = get_db()
    cur = conn.cursor()
    cur.execute('DELETE FROM todos WHERE id = %s', (tid,))
    conn.commit()
    cur.close()
    conn.close()
    return '', 204


@app.route('/api/cache/test')
def cache_test():
    """演示 Redis 缓存读写（对应 DockerNew 06 章：Redis 容器化）"""
    key = 'compose-lab-visit-count'
    count = cache.incr(key)
    return jsonify({'key': key, 'visit_count': count})


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
