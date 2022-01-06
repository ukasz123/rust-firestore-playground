import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:rust_firestore_snapshot_ffi/rust_firestore_snapshot_ffi.dart';

void main() {
  const MethodChannel channel = MethodChannel('rust_firestore_snapshot_ffi');

  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {
    channel.setMockMethodCallHandler((MethodCall methodCall) async {
      return '42';
    });
  });

  tearDown(() {
    channel.setMockMethodCallHandler(null);
  });

  test('getPlatformVersion', () async {
    expect(await RustFirestoreSnapshotFfi.platformVersion, '42');
  });
}
