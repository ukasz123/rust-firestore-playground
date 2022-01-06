//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <rust_firestore_snapshot_ffi/rust_firestore_snapshot_ffi_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) rust_firestore_snapshot_ffi_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "RustFirestoreSnapshotFfiPlugin");
  rust_firestore_snapshot_ffi_plugin_register_with_registrar(rust_firestore_snapshot_ffi_registrar);
}
