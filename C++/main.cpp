#include <jni.h>
#include <string>
#include <vector>
#include <sstream>
#include "com_prohor_translator_TranslatorService.h"

std::string cyclicShiftString(const std::string& str) {
    if (str.empty()) return str;
    return str.back() + str.substr(0, str.size() - 1);
}

std::vector<jint> cyclicShiftArray(const std::vector<jint>& arr) {
    if (arr.empty()) return arr;
    std::vector<jint> result(arr.size());
    result[0] = arr.back();
    for (size_t i = 1; i < arr.size(); ++i) {
        result[i] = arr[i - 1];
    }
    return result;
}

JNIEXPORT jstring JNICALL Java_com_prohor_translator_TranslatorService_nativeFunction
(JNIEnv* env, jobject obj, jstring jstr, jintArray jarr) {
    const char* strChars = env->GetStringUTFChars(jstr, nullptr);
    std::string str(strChars);
    env->ReleaseStringUTFChars(jstr, strChars);

    jsize arrLength = env->GetArrayLength(jarr);
    jint* arrElements = env->GetIntArrayElements(jarr, nullptr);
    std::vector<jint> arr(arrElements, arrElements + arrLength);
    env->ReleaseIntArrayElements(jarr, arrElements, 0);

    std::string shiftedStr = cyclicShiftString(str);
    std::vector<jint> shiftedArr = cyclicShiftArray(arr);

    std::ostringstream oss;
    oss << shiftedStr << ", [";
    for (size_t i = 0; i < shiftedArr.size(); ++i) {
        if (i > 0) oss << ", ";
        oss << shiftedArr[i];
    }
    oss << "]";

    std::string result = oss.str();
    return env->NewStringUTF(result.c_str());
}
