"""v6：+ 长期记忆（mem0 跨会话）。来源：15 篇。
增量：调用前 search 取上下文 + 调用后 add 存交互；重启进程记忆仍在。
验收：重启后仍记得用户偏好。
⚠️ mem0 版本变动剧烈（教学基线 0.1.x / version v1.1）；ada-002 已过时，新项目建议 text-embedding-3-small。"""
from mem0 import Memory
from langchain_openai import ChatOpenAI
from langchain_core.prompts import ChatPromptTemplate

config = {
    "version": "v1.1",
    "llm": {
        "provider": "openai",
        "config": {
            "model": "gpt-4o-mini",
            "temperature": 0,
            "max_tokens": 1500,
        }
    },
    "embedder": {
        "provider": "openai",
        "config": {
            "model": "text-embedding-ada-002"
        }
    },
    "vector_store": {
        "provider": "chroma",
        "config": {
            "collection_name": "mem0db",
            "path": "mem0db",
        }
    },
    "history_db_path": "history.db",
}

mem0 = Memory.from_config(config)

llm = ChatOpenAI(model="gpt-4o-mini")

prompt = ChatPromptTemplate.from_messages([
    ("system", """你现在扮演孔子的角色，尽量按照孔子的风格回复，不要出现‘子曰’。

利用提供的上下文进行个性化回复，并记住用户的偏好和以往的交互行为。

上下文：{context}"""),
    ("user", "{input}")
])

chain = prompt | llm

def retrieve_context(query: str, user_id: str) -> str:
    memories = mem0.search(query, user_id=user_id)
    return ' '.join([mem["memory"] for mem in memories['results']])

def save_interaction(user_id: str, user_input: str, assistant_response: str):
    interaction = [
        {"role": "user", "content": user_input},
        {"role": "assistant", "content": assistant_response}
    ]
    mem0.add(interaction, user_id=user_id)

def invoke(user_input: str, user_id: str) -> str:
    context = retrieve_context(user_input, user_id)
    response = chain.invoke({"context": context, "input": user_input})
    content = response.content
    save_interaction(user_id, user_input, content)
    return content

if __name__ == "__main__":
    user_id = "dreamhead"
    while True:
        user_input = input("You:> ")
        if user_input.lower() == 'exit':
            break
        print(invoke(user_input, user_id))