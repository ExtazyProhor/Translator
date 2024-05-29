package main

import (
    "context"
    "encoding/json"
    "log"
    "net/http"
    "time"

    "github.com/gorilla/websocket"
    "google.golang.org/grpc"
    pb "Golang/protobuf/translator"
)

var upgrader = websocket.Upgrader{
    ReadBufferSize:  1024,
    WriteBufferSize: 1024,
    CheckOrigin: func(r *http.Request) bool {
        return true
    },
}

func cyclicShiftString(s string) string {
    if len(s) == 0 {
        return s
    }
    return s[len(s)-1:] + s[:len(s)-1]
}

func cyclicShiftArray(arr []int32) []int32 {
    if len(arr) == 0 {
        return arr
    }
    return append([]int32{arr[len(arr)-1]}, arr[:len(arr)-1]...)
}

func wsHandler(w http.ResponseWriter, r *http.Request) {
    conn, err := upgrader.Upgrade(w, r, nil)
    if err != nil {
        log.Println(err)
        return
    }
    defer conn.Close()

    for {
        _, msg, err := conn.ReadMessage()
        if err != nil {
            log.Println(err)
            return
        }

        var inputData struct {
            Text  string  `json:"text"`
            Array []int32 `json:"array"`
        }
        if err := json.Unmarshal(msg, &inputData); err != nil {
            log.Println(err)
            return
        }

        inputData.Text = cyclicShiftString(inputData.Text)
        inputData.Array = cyclicShiftArray(inputData.Array)

        connGRPC, err := grpc.Dial("127.0.0.1:8103", grpc.WithInsecure())
        if err != nil {
            log.Println(err)
            return
        }
        defer connGRPC.Close()

        client := pb.NewTranslatorServiceClient(connGRPC)
        ctx, cancel := context.WithTimeout(context.Background(), time.Second)
        defer cancel()

        resp, err := client.Process(ctx, &pb.DataRequest{
            Text:  inputData.Text,
            Array: inputData.Array,
        })
        if err != nil {
            log.Println(err)
            return
        }

        conn.WriteMessage(websocket.TextMessage, []byte(resp.Response))
    }
}

func main() {
    http.HandleFunc("/", wsHandler)
    log.Fatal(http.ListenAndServe(":8102", nil))
}
