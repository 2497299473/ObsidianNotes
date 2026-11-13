"""v2：+ 会话记忆。来源：09 篇第二节。
增量：RunnableWithMessageHistory + InMemoryChatMessageHistory + session_id。
验收：报过名字后问「我是谁」能答对。"""
from langchain_core.chat_history import BaseChatMessageHistory, InMemoryChatMessageHistory
from langchain_core.runnables import RunnableWithMessageHistory
from langchain_openai import ChatOpenAI
from langchain_core.messages import HumanMessage

chat_model = ChatOpenAI(model="gpt-4o-mini")

store = {}

def get_session_history(session_id: str) -> BaseChatMessageHistory:
    if session_id not in store:
        store[session_id] = InMemoryChatMessageHistory()
    return store[session_id]

with_message_history = RunnableWithMessageHistory(chat_model, get_session_history)

config = {"configurable": {"session_id": "dreamhead"}}

while True:
    user_input = input("You:> ")
    if user_input.lower() == 'exit':
        break
    stream = with_message_history.stream(
        [HumanMessage(content=user_input)],
        config=config
    )
    for chunk in stream:
        print(chunk.content, end='', flush=True)
    print()