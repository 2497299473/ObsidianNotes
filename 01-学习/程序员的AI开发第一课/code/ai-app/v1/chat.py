"""v1：最简命令行聊天机器人（无记忆）。来源：09 篇第一节。
验收：问「我是谁」应答不出——复现 HTTP 无状态问题。"""
from langchain_openai import ChatOpenAI
from langchain_core.messages import HumanMessage

chat_model = ChatOpenAI(model="gpt-4o-mini")

while True:
    user_input = input("You:> ")
    if user_input.lower() == 'exit':
        break
    stream = chat_model.stream([HumanMessage(content=user_input)])
    for chunk in stream:
        print(chunk.content, end='', flush=True)
    print()