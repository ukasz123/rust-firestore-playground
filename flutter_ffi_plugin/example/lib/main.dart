import 'dart:ffi';
import 'dart:io';

import 'package:flutter/material.dart';
import 'dart:async';

import 'package:flutter/services.dart';
import 'package:rust_firestore_snapshot_ffi/bridge_generated.dart';
import 'package:rust_firestore_snapshot_ffi/rust_firestore_snapshot_ffi.dart';
import 'package:file_picker/file_picker.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String _platformVersion = 'Unknown';

  FirestoreClient? _bridge;

  @override
  void initState() {
    super.initState();
    initPlatformState();
    initRustFirestoreBridge();
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initPlatformState() async {
    String platformVersion;
    // Platform messages may fail, so we use a try/catch PlatformException.
    // We also handle the message potentially returning null.
    try {
      platformVersion = await RustFirestoreSnapshotFfi.platformVersion ??
          'Unknown platform version';
    } on PlatformException {
      platformVersion = 'Failed to get platform version.';
    }

    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) return;

    setState(() {
      _platformVersion = platformVersion;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: Column(
          children: [
            Text('Running on: $_platformVersion\n'),
            const SizedBox(height: 12),
            if (_bridge == null) Placeholder() else FirestoreClientUI(_bridge!),
          ],
        ),
      ),
    );
  }

  Future<void> initRustFirestoreBridge() async {
    late DynamicLibrary lib;
    if (Platform.isMacOS) {
      lib = DynamicLibrary.open('libffi.dylib');
    } else {
      throw UnimplementedError(
          "looking for current platform library is not yet implemented.");
    }
    _bridge = FirestoreClientImpl(lib);
  }
}

class FirestoreClientUI extends StatefulWidget {
  const FirestoreClientUI(this.bridge, {Key? key}) : super(key: key);

  final FirestoreClient bridge;

  @override
  _FirestoreClientUIState createState() => _FirestoreClientUIState();
}

enum Method {
  GET,
  UPDATE,
}

class _FirestoreClientUIState extends State<FirestoreClientUI> {
  Method _method = Method.GET;

  String? _firebaseProjectId;
  String? _firebaseToken;

  String? _collectionPath;

  String? _filePath;

  bool _processing = false;

  Object? _error;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0),
      child: Column(
        children: [
          Text('ready?'),
          Row(
            children: [
              Text('UPDATE'),
              Switch.adaptive(
                value: _method == Method.GET,
                onChanged: (checked) => setState(
                  () {
                    _method = checked ? Method.GET : Method.UPDATE;
                  },
                ),
              ),
              Text('GET'),
            ],
          ),
          TextField(
            decoration: InputDecoration(
              labelText: 'Firebase project id',
            ),
            onChanged: (value) => setState(() {
              _firebaseProjectId = value;
            }),
          ),
          TextField(
            decoration: InputDecoration(
              labelText: 'Firebase token',
            ),
            onChanged: (value) => setState(() {
              _firebaseToken = value;
            }),
          ),
          TextField(
            decoration: InputDecoration(
              labelText: 'Firebase collection path',
            ),
            onChanged: (value) => setState(() {
              _collectionPath = value;
            }),
          ),
          SizedBox(height: 8),
          OutlinedButton.icon(
            onPressed: () async {
              FilePickerResult? result = await FilePicker.platform.pickFiles();
              setState(() {
                if (result != null) {
                  _filePath = result.files.single.path;
                } else {
                  _filePath = null;
                }
              });
            },
            icon: Icon(Icons.file_present_outlined),
            label: Text('Select file'),
          ),
          SizedBox(height: 14),
          ElevatedButton(
            onPressed: _validated() ? _triggerOperation : null,
            child: _processing
                ? LinearProgressIndicator()
                : Text('Perform operation'),
          ),
          SizedBox(height: 16),
          if (_error != null)
            Text(
              "$_error",
              style: Theme.of(context)
                  .textTheme
                  .bodyText2!
                  .copyWith(color: Theme.of(context).errorColor),
            )
        ],
      ),
    );
  }

  bool _validated() =>
      !_processing &&
      _filePath.isNotNullOrEmpty &&
      _collectionPath.isNotNullOrEmpty &&
      _firebaseToken.isNotNullOrEmpty &&
      _firebaseProjectId.isNotNullOrEmpty;

  Future<void> _triggerOperation() async {
    try {
      setState(() {
        _error = null;
        _processing = true;
      });
      switch (_method) {
        case Method.GET:
          await widget.bridge.getCollection(
              projectId: _firebaseProjectId!,
              token: _firebaseToken!,
              collectionPath: _collectionPath!,
              outputPath: _filePath!);
          break;
        case Method.UPDATE:
          await widget.bridge.updateCollection(
              projectId: _firebaseProjectId!,
              token: _firebaseToken!,
              collectionPath: _collectionPath!,
              inputFilePath: _filePath!);
          break;
        default:
      }
    } catch (e) {
      setState(() {
        _error = e;
      });
    } finally {
      setState(() {
        _processing = false;
      });
    }
  }
}

extension on String? {
  bool get isNotNullOrEmpty => this != null && this!.isNotEmpty;
}
