package com.prohor.translator;

import io.grpc.Server;
import io.grpc.ServerBuilder;
import java.io.IOException;

public class Main {
    public static void main(String[] args) throws IOException, InterruptedException {
        Server server = ServerBuilder.forPort(8083)
                .addService(new TranslatorService())
                .build();
        server.start();
        server.awaitTermination();
    }
}
