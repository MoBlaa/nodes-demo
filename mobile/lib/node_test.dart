import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'dart:io';

final DynamicLibrary nodelibNative = Platform.isAndroid
    ? DynamicLibrary.open("libandroid_lib.so")
    : DynamicLibrary.process();

typedef HelloFunction = Pointer<Utf8> Function(Pointer<Utf8>);
typedef HelloFunctionFFI = Pointer<Utf8> Function(Pointer<Utf8>);

final HelloFunction rustHello = nodelibNative
    .lookup<NativeFunction<HelloFunctionFFI>>("hello")
    .asFunction();

String hello(String name) {
  if (nodelibNative == null) {
    return "ERROR: The library is not initialized";
  }

  print("- android_lib bindings found");
  print("  ${nodelibNative.toString()}");

  final argName = Utf8.toUtf8(name);
  print("- Calling rusts hello with argument: $argName");

  final resultPointer = rustHello(argName);
  final helloStr = Utf8.fromUtf8(resultPointer);
  print("- Response string: $helloStr");

  return helloStr;
}