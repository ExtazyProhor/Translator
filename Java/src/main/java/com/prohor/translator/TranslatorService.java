package com.prohor.translator;

import com.prohor.grpc.DataRequest;
import com.prohor.grpc.DataResponse;
import com.prohor.grpc.TranslatorServiceGrpc;
import io.grpc.stub.StreamObserver;

import java.util.Collections;
import java.util.List;

class TranslatorService extends TranslatorServiceGrpc.TranslatorServiceImplBase {
    @Override
    public void process(DataRequest request, StreamObserver<DataResponse> responseObserver) {
        String text = request.getText();
        if (!text.isEmpty()) {
            text = text.charAt(text.length() - 1) + text.substring(0, text.length() - 1);
        }
        List<Integer> array = request.getArrayList();
        if (!array.isEmpty()) {
            Collections.rotate(array, 1);
        }

        String result = nativeFunction(text, array.stream().mapToInt(i->i).toArray());
        DataResponse response = DataResponse.newBuilder()
                .setResponse(result)
                .build();

        responseObserver.onNext(response);
        responseObserver.onCompleted();
    }

    public native String nativeFunction(String text, int[] array);
}