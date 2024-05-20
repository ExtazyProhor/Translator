<h1 align="center">Переводчик</h1>

Кто такие переводчики? Это люди, помогающие общаться носителям разных языков.
Данный проект посвящен похожей процедуре - он показывает, какими способами
могут общаться между собой программы, написанные на разных языках программирования.

---
В качестве "переводчиков" мы будем использовать следующие технологии:

- Стандартные HTTP запросы
- WebSockets
- gRPC
- Общий доступ к базе данных SQLite
- JNI

Помимо общения между приложениями, эти технологии также производят передачу данных,
поэтому этот процесс будет также рассмотрен.
Так как весь проект будет развернут на одном хосте, для примера достаточно будет
использования СУБД SQLite, которая хранит всю информацию в одном файле с расширением
`.db`. В сравнении будет рассматриваться общий случай
использования баз данных, которые, как правило, поддерживают доступ по сети и
одновременное чтение данных несколькими пользователями.

Для более подробного сравнения способов передачи данных между приложениями,
рассмотрим у них несколько критериев:
- поддержка языков программирования;
- типы данных;
- возможность работы по сети;
- безопасность при передаче данных.


<table>
    <thead>
        <tr>
            <th>Способ взаимодействия</th>
            <th>Поддерживаемые языки</th>
            <th>Типы данных</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>HTTP запросы</td>
            <td>Все современные языки</td>
            <td>JSON, XML, HTML и т.д.</td>
        </tr>
        <tr>
            <td>WebSockets</td>
            <td>Все современные языки</td>
            <td>Текст, бинарные данные</td>
        </tr>
        <tr>
            <td>gRPC</td>
            <td>C#, C++, Dart, Go, Java, Kotlin, Node.js,
                Objective-C, PHP, Python, и Ruby</td>
            <td>Protocol Buffers</td>
        </tr>
        <tr>
            <td>Общий доступ к БД</td>
            <td>Все современные языки</td>
            <td>Зависит от выбранной СУБД</td>
        </tr>
        <tr>
            <td>JNI</td>
            <td>Только из Java можно вызывать функции на C/C++</td>
            <td>Примитивы, структуры</td>
        </tr>
    </tbody>
</table>

<table>
    <thead>
        <tr>
            <th>Способ взаимодействия</th>
            <th>Работа по сети</th>
            <th>Безопасность</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>HTTP запросы</td>
            <td>Есть</td>
            <td>TLS/SSL</td>
        </tr>
        <tr>
            <td>WebSockets</td>
            <td>Есть</td>
            <td>TLS/SSL</td>
        </tr>
        <tr>
            <td>gRPC</td>
            <td>Есть</td>
            <td>TLS/SSL</td>
        </tr>
        <tr>
            <td>Общий доступ к БД</td>
            <td>Есть почти у всех СУБД</td>
            <td>TLS/SSL + система авторизации самой СУБД</td>
        </tr>
        <tr>
            <td>JNI</td>
            <td>Отсутствует</td>
            <td>Не требуется, так как работает локально</td>
        </tr>
    </tbody>
</table>






```
C++ <-(JNI)- Java <-(gRPC)- Go <-(WebSockets)- Python <-(SQLite)- Rust <-(HTTP)- JS
```

<br>

```
g++ -m64 -c -I"C:\Program Files\Java\jdk-20\include" -I"C:\Program Files\Java\jdk-20\include\win32" Native.cpp -o Native.o
```

```
g++ -m64 -shared -o approximationLib.dll Native.o -Wl,--add-stdcall-alias
```

```
-Djava.library.path="C:\Users\User\Desktop\jni"
```

```
java -Djava.library.path="C:\Users\User\Desktop\jni" -jar app.jar
```

## links

[JNI habr](https://habr.com/ru/companies/citymobil/articles/650403/)

x86_64-13.2.0-release-posix-seh-msvcrt-rt_v11-rev0.7z

[MINGW](https://github.com/niXman/mingw-builds-binaries/releases)

[SQLite](https://www.sqlite.org/download.html)

<h5 align="right">Прохоров Тимофей</h5>