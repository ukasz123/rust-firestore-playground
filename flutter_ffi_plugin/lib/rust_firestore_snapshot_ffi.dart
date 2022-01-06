
import 'dart:async';

import 'package:flutter/services.dart';

class RustFirestoreSnapshotFfi {
  static const MethodChannel _channel = MethodChannel('rust_firestore_snapshot_ffi');

  static Future<String?> get platformVersion async {
    final String? version = await _channel.invokeMethod('getPlatformVersion');
    return version;
  }
}
