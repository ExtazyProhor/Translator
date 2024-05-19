```
C++ <-(JNI)- Java <-(gRPC)- Python <-(SQLite)- Rust <-(HTTP)- JS
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

