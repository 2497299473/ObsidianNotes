"""v7：v7a 语义缓存 + v7b 集中接入（单文件演示两件事，非完整聊天应用）。来源：16/17 篇。
定位：并列的单点能力演示——不含 v1–v6 的 REPL/记忆/Agent（原作的教学取舍，见 README）。
增量：FixedSemanticCache(RedisSemanticCache) 语义命中；client 指向 One-API 网关。
前提：① Redis Stack：docker run -p 6379:6379 redis/redis-stack-server:latest
      ② One-API 已启动并配好渠道/令牌（见 17 篇；DSN 用 host.docker.internal 非容器内 localhost）
      ③ 环境变量 ONEAPI_KEY = One-API 令牌（不是 OpenAI 官方 Key）；未设则报错退出
验收：两句同义提示词第二次明显加速；换后端模型只改网关配置，本文件零改动。
教学基线 LangChain 0.3.x；1.x 下 set_llm_cache 改从 langchain_core.globals 导入（见 16 篇版本注记）。"""
import json
import os
from time import time
from typing import Any, Optional

from langchain.globals import set_llm_cache
from langchain_core.caches import BaseCache, RETURN_VAL_TYPE
from langchain_community.cache import RedisSemanticCache
from langchain_openai import OpenAIEmbeddings, ChatOpenAI

def prompt_key(prompt: str) -> str:
    messages = json.loads(prompt)
    result = ["('{}', '{}')".format(data['kwargs']['type'], data['kwargs']['content']) for data in messages if
              'kwargs' in data and 'type' in data['kwargs'] and 'content' in data['kwargs']]
    return ' '.join(result)

class FixedSemanticCache(BaseCache):
    """临时补丁：LangChain 消息序列化信噪比低，提取提示词+类型作键（见 16 篇）。"""
    def __init__(self, cache: BaseCache):
        self.cache = cache

    def lookup(self, prompt: str, llm_string: str) -> Optional[RETURN_VAL_TYPE]:
        return self.cache.lookup(prompt_key(prompt), llm_string)

    def update(self, prompt: str, llm_string: str, return_val: RETURN_VAL_TYPE) -> None:
        return self.cache.update(prompt_key(prompt), llm_string, return_val)

    def clear(self, **kwargs: Any) -> None:
        return self.cache.clear(**kwargs)

set_llm_cache(
    FixedSemanticCache(
        RedisSemanticCache(redis_url="redis://localhost:6379",
                           embedding=OpenAIEmbeddings())
    )
)

# 集中接入：base_url 指向 One-API（而非 api.openai.com），api_key 用 One-API 令牌。
# 换模型/换供应商只改网关渠道配置，这里不动。令牌从环境变量读，不硬编码。
model = ChatOpenAI(
    model="gpt-4o-mini",
    base_url="http://localhost:3000/v1",
    api_key=os.environ["ONEAPI_KEY"],  # One-API 令牌；未设置则 KeyError，提示见 README
)

if __name__ == "__main__":
    for prompt_text in ("请给我讲一个一句话笑话", "你能不能给我讲一个一句话笑话"):
        start = time()
        response = model.invoke(prompt_text)
        print(response.content)
        print(f"耗时: {time() - start:.4f}秒")