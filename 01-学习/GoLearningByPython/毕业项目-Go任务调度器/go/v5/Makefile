APP_NAME=scheduler
VERSION=1.0.0

build:
	go build -o $(APP_NAME) ./main.go

test:
	go test -v -race -cover ./...

bench:
	go test -bench=. -benchmem

# 交叉编译（对比 Python：PyInstaller 无法轻松做到）
build-linux:
	GOOS=linux GOARCH=amd64 go build -o $(APP_NAME)-linux-amd64 ./main.go

build-windows:
	GOOS=windows GOARCH=amd64 go build -o $(APP_NAME)-windows-amd64.exe ./main.go

build-mac:
	GOOS=darwin GOARCH=arm64 go build -o $(APP_NAME)-mac-arm64 ./main.go

build-all: build-linux build-windows build-mac
	@echo "交叉编译完成"

clean:
	rm -f $(APP_NAME)*
