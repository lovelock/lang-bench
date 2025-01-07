package main

import (
	"context"
	"net/http"
	"strings"

	"github.com/go-redis/redis/v8"
	"github.com/labstack/echo"
)

var ctx = context.Background()

func main() {
	e := echo.New()
	rdb := redis.NewClient(&redis.Options{
		Addr: "127.0.0.1:6379",
	})

	e.GET("/", func(c echo.Context) error {
		val, err := rdb.Get(ctx, "key").Result()
		if err != nil {
			return c.JSON(http.StatusInternalServerError, err.Error())
		}
		return c.String(http.StatusOK, strings.ToUpper(val))
	})

	e.Start(":8080")
}
