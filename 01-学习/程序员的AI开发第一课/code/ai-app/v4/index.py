"""v4 索引侧：把业务资料写入本地 Chroma。来源：11 篇索引节。
用法：把资料存为 introduction.txt（或改 loader 路径），先跑本脚本再跑 chat.py。"""
from langchain_community.document_loaders import TextLoader
from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_chroma import Chroma
from langchain_openai import OpenAIEmbeddings

loader = TextLoader("introduction.txt", encoding="utf-8")
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