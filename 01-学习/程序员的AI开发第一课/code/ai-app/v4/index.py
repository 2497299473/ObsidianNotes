"""v4 索引侧：把业务资料写入本地 Chroma。来源：11 篇索引节。
用法：先跑本脚本再跑 chat.py。默认语料 sample_data/introduction.txt（摘编自开篇词原文剪藏，
含「作者是谁/还写过哪些专栏」两个验收问题的事实）；换成自己的资料替换该文件或改 loader 路径。"""
from pathlib import Path

from langchain_community.document_loaders import TextLoader
from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_chroma import Chroma
from langchain_openai import OpenAIEmbeddings

# 相对本文件定位：从仓库根目录或 v4/ 目录启动都能找到语料
loader = TextLoader(str(Path(__file__).parent / "sample_data" / "introduction.txt"), encoding="utf-8")
docs = loader.load()

text_splitter = RecursiveCharacterTextSplitter(chunk_size=1000, chunk_overlap=200)
splits = text_splitter.split_documents(docs)

vectorstore = Chroma(
    collection_name="ai_learning",
    embedding_function=OpenAIEmbeddings(),
    persist_directory="vectordb"
)
vectorstore.add_documents(splits)
print(f"indexed {len(splits)} chunks")

# 检索自检（11 篇待办：写入后立即验证能检回）：两问都应有命中片段
for q in ("专栏的作者是谁？", "作者还写过哪些专栏？"):
    hits = vectorstore.similarity_search(q, k=1)
    print(f"Q: {q}\n  → {hits[0].page_content[:60]}…" if hits else f"Q: {q}\n  → 无命中，检查语料")