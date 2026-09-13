# 对应 DockerNew 第 3 章：5 个核心指令
# 缓存友好顺序：先 COPY requirements.txt → RUN pip install → 最后 COPY app.py
FROM python:3.12-alpine

WORKDIR /app

# 先复制依赖文件（很少变，缓存命中）
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# 最后复制代码（常变，只影响这一层）
COPY app.py .

EXPOSE 5000

# exec 格式：PID 1 是应用本身，能接收 SIGTERM 优雅停止
CMD ["python", "app.py"]
