package main

import (
    "context"
    "encoding/json"
    "log"
    "net/http"

    "google.golang.org/grpc"
    pb "protobuf/translator/service"
    "github.com/gorilla/websocket"
)

type Message struct {
    Text  string `json:"text"`
    Array []int  `json:"array"`
}

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

func cyclicShiftArray(arr []int) []int {
    if len(arr) == 0 {
        return arr
    }
    return append([]int{arr[len(arr)-1]}, arr[:len(arr)-1]...)
}

func handler(w http.ResponseWriter, r *http.Request) {
    conn, err := upgrader.Upgrade(w, r, nil)
    if err != nil {
        log.Println(err)
        return
    }
    defer conn.Close()

    grpcConn, err := grpc.Dial("127.0.0.1:8103", grpc.WithInsecure())
    if err != nil {
        log.Fatalf("did not connect: %v", err)
    }
    defer grpcConn.Close()
    client := pb.NewMyServiceClient(grpcConn)

    for {
        _, message, err := conn.ReadMessage()
        if err != nil {
            log.Println("read:", err)
            break
        }

        log.Printf("Received: %s", message)

        var msg Message
        err = json.Unmarshal(message, &msg)
        if err != nil {
            log.Println("unmarshal:", err)
            continue
        }

        // Cyclic shift
        shiftedText := cyclicShiftString(msg.Text)
        shiftedArray := cyclicShiftArray(msg.Array)

        // Create gRPC request
        grpcRequest := &pb.DataRequest{
            Text:  shiftedText,
            Array: shiftedArray,
        }

        // Send request to gRPC server
        grpcResponse, err := client.ProcessData(context.Background(), grpcRequest)
        if err != nil {
            log.Println("grpc error:", err)
            continue
        }

        // Send response back to WebSocket client
        responseJSON, err := json.Marshal(grpcResponse)
        if err != nil {
            log.Println("marshal:", err)
            continue
        }

        err = conn.WriteMessage(websocket.TextMessage, responseJSON)
        if err != nil {
            log.Println("write:", err)
            break
        }

        log.Printf("Sent: %s", responseJSON)
    }
}

func main() {
    http.HandleFunc("/", handler)
    log.Println("Starting WebSocket server on :8102")
    log.Fatal(http.ListenAndServe(":8102", nil))
}