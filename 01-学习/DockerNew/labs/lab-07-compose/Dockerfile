# 对应 DockerNew 第 3 章：Dockerfile 5 个核心指令
# 缓存友好顺序：先 COPY requirements.txt → RUN pip install → 最后 COPY app.py
FROM python:3.12-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY app.py .

EXPOSE 5000

# exec 格式：PID 1 是应用本身，能接收 SIGTERM 优雅停止
CMD ["python", "app.py"]
