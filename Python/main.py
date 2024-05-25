import asyncio
import json
import websockets
import sqlite3


async def send_and_receive(ws_uri):
    async with websockets.connect(ws_uri) as websocket:
        with sqlite3.connect('../SQL/messages.db') as conn:
            cursor = conn.cursor()
            while True:
                cursor.execute("SELECT id FROM InputMessages LIMIT 1")
                row = cursor.fetchone()

                if row:
                    message_id = row[0]
                    cursor.execute("BEGIN TRANSACTION;")
                    cursor.execute("SELECT string, arraySize FROM InputMessages WHERE id = ?", (message_id,))
                    input_message = cursor.fetchone()
                    text, array_size = input_message

                    cursor.execute("SELECT arrayIndex, intValue FROM ArrayElements WHERE messageId = ?", (message_id,))
                    array_elements = cursor.fetchall()

                    cursor.execute("DELETE FROM ArrayElements WHERE messageId = ?", (message_id,))
                    cursor.execute("DELETE FROM InputMessages WHERE id = ?", (message_id,))
                    conn.commit()

                    array = [0] * array_size
                    for array_index, int_value in array_elements:
                        array[array_index] = int_value

                    if text:
                        text = text[-1] + text[:-1]
                    if array:
                        array = [array[-1]] + array[:-1]

                    data_to_send = {
                        'text': text,
                        'array': array
                    }
                    await websocket.send(json.dumps(data_to_send))

                    response = await websocket.recv()

                    cursor.execute("INSERT INTO OutputMessages (id, string) VALUES (?, ?)", (message_id, response))
                    conn.commit()
                else:
                    await asyncio.sleep(0.1)


if __name__ == "__main__":
    uri = "ws://127.0.0.1:8102"
    asyncio.get_event_loop().run_until_complete(send_and_receive(uri))
