"""v5a（路径一）：手写 ReAct Agent（不依赖 LangChain）。来源：13 篇。
定位：单点能力演示——不继承 v2–v4 的会话记忆/角色/RAG（原作的教学取舍，见 README）。
验收：水果价格题（3kg 苹果 + 2kg 香蕉）在 max_turns 内收敛，正确答案 42。
⚠️ calculate 用 eval 执行模型生成的表达式，仅限教学演示；生产须受限求值 + 隔离环境。
注意：13 篇原文示例输出 22 是模型把 3kg 误读成 1kg 的真实失误记录——验收要校验结果与题意一致。"""
import re

from openai import OpenAI

DEFAULT_MODEL = "gpt-4o-mini"
client = OpenAI()

class Agent:
    def __init__(self, system=""):
        self.system = system
        self.messages = []
        if self.system:
            self.messages.append({"role": "system", "content": system})

    def invoke(self, message):
        self.messages.append({"role": "user", "content": message})
        result = self.execute()
        self.messages.append({"role": "assistant", "content": result})
        return result

    def execute(self):
        completion = client.chat.completions.create(
            model=DEFAULT_MODEL,
            messages=self.messages,
            temperature=0
        )
        return completion.choices[0].message.content

prompt = """
You run in a loop of Thought, Action, PAUSE, Observation.
At the end of the loop you output an Answer
Use Thought to describe your thoughts about the question you have been asked.
Use Action to run one of the actions available to you - then return PAUSE.
Observation will be the result of running those actions.

Your available actions are:

calculate:
e.g. calculate: 4 * 7 / 3
Runs a calculation and returns the number - uses Python so be sure to use floating point syntax if necessary

ask_fruit_unit_price:
e.g. ask_fruit_unit_price: apple
Asks the user for the price of a fruit

Example session:

Question: What is the unit price of apple?

Thought: I need to ask the user for the price of an apple to provide the unit price.

Action: ask_fruit_unit_price: apple

PAUSE

You will be called again with this:

Observation: Apple unit price is 10/kg

You then output:

Answer: The unit price of apple is 10 per kg.

""".strip()

def calculate(what):
    return eval(what)

def ask_fruit_unit_price(fruit):
    if fruit.casefold() == "apple":
        return "Apple unit price is 10/kg"
    elif fruit.casefold() == "banana":
        return "Banana unit price is 6/kg"
    else:
        return "{} unit price is 20/kg".format(fruit)

action_re = re.compile(r'^Action: (\w+): (.*)$')

known_actions = {
    "calculate": calculate,
    "ask_fruit_unit_price": ask_fruit_unit_price
}

def query(question, max_turns=5):
    i = 0
    agent = Agent(prompt)
    next_prompt = question
    while i < max_turns:
        i += 1
        result = agent.invoke(next_prompt)
        print(result)
        actions = [action_re.match(a) for a in result.split('\n') if action_re.match(a)]
        if actions:
            action, action_input = actions[0].groups()
            if action not in known_actions:
                raise Exception("Unknown action: {}: {}".format(action, action_input))
            print(" -- running {} {}".format(action, action_input))
            observation = known_actions[action](action_input)
            print("Observation:", observation)
            next_prompt = "Observation: {}".format(observation)
        else:
            return
    else:
        # while…else：循环因 max_turns 耗尽而退出（非正常收敛），对应 13 篇「限制最大询问次数」教学点
        print(f"[未收敛] 已达 max_turns={max_turns}，未得到 Final Answer；请检查提示词与动作名是否一致")

if __name__ == "__main__":
    query("What is the total price of 3kg of apple and 2kg of banana?")