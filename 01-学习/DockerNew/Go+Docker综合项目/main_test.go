package main

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gin-gonic/gin"
)

// 这些用例不依赖 PostgreSQL / Redis：非法 id 在进库前就被拦掉，
// applyUpdate 是纯函数，因此本机 `go test ./...` 无需起任何服务也能跑。

// TestRejectNonNumericID 一条请求里的 id 不是纯数字时必须返回 400。
// 这是 SQL 注入（`/api/todos/1 OR 1=1` 被当成 GORM 原始条件）的回归测试。
func TestRejectNonNumericID(t *testing.T) {
	gin.SetMode(gin.TestMode)

	handlers := map[string]gin.HandlerFunc{
		"getTodo":    getTodo,
		"updateTodo": updateTodo,
		"deleteTodo": deleteTodo,
	}
	badIDs := []string{"1 OR 1=1", "abc", "-1", "1;DROP TABLE todos"}

	for name, h := range handlers {
		for _, id := range badIDs {
			w := httptest.NewRecorder()
			c, _ := gin.CreateTestContext(w)
			c.Request = httptest.NewRequest(http.MethodGet, "/api/todos/x", nil)
			c.Params = gin.Params{{Key: "id", Value: id}}

			h(c)

			if w.Code != http.StatusBadRequest {
				t.Errorf("%s(id=%q) = %d, want 400", name, id, w.Code)
			}
		}
	}
}

// TestApplyUpdateIgnoresID 请求体里的 id 不能改写已加载记录的主键（mass assignment）
func TestApplyUpdateIgnoresID(t *testing.T) {
	todo := Todo{ID: 7, Title: "旧标题", Done: false}

	w := httptest.NewRecorder()
	c, _ := gin.CreateTestContext(w)
	c.Request = httptest.NewRequest(http.MethodPut, "/api/todos/7",
		strings.NewReader(`{"id":999,"title":"新标题","done":true}`))

	var input UpdateTodoInput
	if err := c.ShouldBindJSON(&input); err != nil {
		t.Fatalf("bind: %v", err)
	}
	applyUpdate(&todo, input)

	if todo.ID != 7 {
		t.Errorf("ID = %d, want 7（请求体里的 id 必须被忽略）", todo.ID)
	}
	if todo.Title != "新标题" || !todo.Done {
		t.Errorf("title/done 未按白名单写入: %+v", todo)
	}
}

// TestApplyUpdatePartial 只传 title 时 done 保持原值（指针字段区分"没传"与"传 false"）
func TestApplyUpdatePartial(t *testing.T) {
	todo := Todo{ID: 3, Title: "旧", Done: true}

	w := httptest.NewRecorder()
	c, _ := gin.CreateTestContext(w)
	c.Request = httptest.NewRequest(http.MethodPut, "/api/todos/3",
		strings.NewReader(`{"title":"只改标题"}`))

	var input UpdateTodoInput
	if err := c.ShouldBindJSON(&input); err != nil {
		t.Fatalf("bind: %v", err)
	}
	applyUpdate(&todo, input)

	if todo.Title != "只改标题" {
		t.Errorf("Title = %q", todo.Title)
	}
	if !todo.Done {
		t.Error("Done 被意外改成 false（应为 nil 指针、保持原值）")
	}
}
