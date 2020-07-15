
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'package:flutter/cupertino.dart';

final DynamicLibrary nativeExampleLib = Platform.isAndroid
    ? DynamicLibrary.open("libnodedemo.so")
    : DynamicLibrary.process();

typedef HelloFunction = Pointer<Utf8> Function(Pointer<Utf8>);

final HelloFunction rustHello = nativeExampleLib
    .lookup<NativeFunction<HelloFunction>>("hello")
    .asFunction();

String hello(String name) {
  if (nativeExampleLib == null) {
    return "Error: The library is not initialized";
  }

  debugPrint("- Mylib bindings found 👍");
  debugPrint("  ${nativeExampleLib.toString()}"); // Instance info

  final argName = Utf8.toUtf8(name);
  debugPrint("- Calling rust_greeting with argument:  $name");

  // The actual native call
  final resultPointer = rustHello(argName);
  debugPrint("- Result pointer:  $resultPointer");

  final greetingStr = Utf8.fromUtf8(resultPointer);
  debugPrint("- Response string:  $greetingStr");

  return greetingStr;
}