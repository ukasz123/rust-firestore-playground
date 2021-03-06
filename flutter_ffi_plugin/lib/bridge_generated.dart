// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import

import 'dart:convert';
import 'dart:typed_data';

import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:ffi' as ffi;

abstract class FirestoreClient {
  Future<void> getCollection(
      {required String projectId,
      required String token,
      required String collectionPath,
      required String outputPath,
      dynamic hint});

  Future<void> updateCollection(
      {required String projectId,
      required String token,
      required String collectionPath,
      required String inputFilePath,
      dynamic hint});
}

class FirestoreClientImpl extends FlutterRustBridgeBase<FirestoreClientWire>
    implements FirestoreClient {
  factory FirestoreClientImpl(ffi.DynamicLibrary dylib) =>
      FirestoreClientImpl.raw(FirestoreClientWire(dylib));

  FirestoreClientImpl.raw(FirestoreClientWire inner) : super(inner);

  Future<void> getCollection(
          {required String projectId,
          required String token,
          required String collectionPath,
          required String outputPath,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_get_collection(
            port,
            _api2wire_String(projectId),
            _api2wire_String(token),
            _api2wire_String(collectionPath),
            _api2wire_String(outputPath)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "get_collection",
          argNames: ["projectId", "token", "collectionPath", "outputPath"],
        ),
        argValues: [projectId, token, collectionPath, outputPath],
        hint: hint,
      ));

  Future<void> updateCollection(
          {required String projectId,
          required String token,
          required String collectionPath,
          required String inputFilePath,
          dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port) => inner.wire_update_collection(
            port,
            _api2wire_String(projectId),
            _api2wire_String(token),
            _api2wire_String(collectionPath),
            _api2wire_String(inputFilePath)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "update_collection",
          argNames: ["projectId", "token", "collectionPath", "inputFilePath"],
        ),
        argValues: [projectId, token, collectionPath, inputFilePath],
        hint: hint,
      ));

  // Section: api2wire
  ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
    return _api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  int _api2wire_u8(int raw) {
    return raw;
  }

  ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  // Section: api_fill_to_wire

}

// Section: wire2api
void _wire2api_unit(dynamic raw) {
  return;
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class FirestoreClientWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  FirestoreClientWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  FirestoreClientWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_get_collection(
    int port_,
    ffi.Pointer<wire_uint_8_list> project_id,
    ffi.Pointer<wire_uint_8_list> token,
    ffi.Pointer<wire_uint_8_list> collection_path,
    ffi.Pointer<wire_uint_8_list> output_path,
  ) {
    return _wire_get_collection(
      port_,
      project_id,
      token,
      collection_path,
      output_path,
    );
  }

  late final _wire_get_collectionPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_get_collection');
  late final _wire_get_collection = _wire_get_collectionPtr.asFunction<
      void Function(
          int,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>)>();

  void wire_update_collection(
    int port_,
    ffi.Pointer<wire_uint_8_list> project_id,
    ffi.Pointer<wire_uint_8_list> token,
    ffi.Pointer<wire_uint_8_list> collection_path,
    ffi.Pointer<wire_uint_8_list> input_file_path,
  ) {
    return _wire_update_collection(
      port_,
      project_id,
      token,
      collection_path,
      input_file_path,
    );
  }

  late final _wire_update_collectionPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_update_collection');
  late final _wire_update_collection = _wire_update_collectionPtr.asFunction<
      void Function(
          int,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>,
          ffi.Pointer<wire_uint_8_list>)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list(
    int len,
  ) {
    return _new_uint_8_list(
      len,
    );
  }

  late final _new_uint_8_listPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list');
  late final _new_uint_8_list = _new_uint_8_listPtr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
