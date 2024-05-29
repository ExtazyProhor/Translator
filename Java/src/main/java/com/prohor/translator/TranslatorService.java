package com.prohor.translator;

import com.prohor.grpc.TranslatorOuterClass;
import com.prohor.grpc.TranslatorServiceGrpc;
import io.grpc.stub.StreamObserver;

import java.util.List;

class TranslatorService extends TranslatorServiceGrpc.TranslatorServiceImplBase {
    static {
        System.loadLibrary("nativeLib");
    }

    @Override
    public void process(TranslatorOuterClass.DataRequest request,
                        StreamObserver<TranslatorOuterClass.DataResponse> responseObserver) {
        String text = request.getText();
        if (!text.isEmpty()) {
            text = text.charAt(text.length() - 1) + text.substring(0, text.length() - 1);
        }
        List<Integer> array = request.getArrayList();
        int[] intArray = new int[array.size()];
        if (array.size() > 0) {
            for(int i = 1; i < array.size(); ++i) {
                intArray[i] = array.get(i - 1);
            }
            intArray[0] = array.getLast();
        }

        String result = nativeFunction(text, intArray);
        TranslatorOuterClass.DataResponse response = TranslatorOuterClass.DataResponse.newBuilder()
                .setResponse(result)
                .build();

        responseObserver.onNext(response);
        responseObserver.onCompleted();
    }

    public native String nativeFunction(String text, int[] array);
}