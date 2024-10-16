// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// The following structs are used to implement the lowest level
// of the FFI, and thus useful to multiple uniffied crates.
// We ensure they are declared exactly once, with a header guard, UNIFFI_SHARED_H.
#ifdef UNIFFI_SHARED_H
    // We also try to prevent mixing versions of shared uniffi header structs.
    // If you add anything to the #else block, you must increment the version suffix in UNIFFI_SHARED_HEADER_V4
    #ifndef UNIFFI_SHARED_HEADER_V4
        #error Combining helper code from multiple versions of uniffi is not supported
    #endif // ndef UNIFFI_SHARED_HEADER_V4
#else
#define UNIFFI_SHARED_H
#define UNIFFI_SHARED_HEADER_V4
// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version suffix in all instances of UNIFFI_SHARED_HEADER_V4 in this file.           ⚠️

typedef struct RustBuffer
{
    uint64_t capacity;
    uint64_t len;
    uint8_t *_Nullable data;
} RustBuffer;

typedef struct ForeignBytes
{
    int32_t len;
    const uint8_t *_Nullable data;
} ForeignBytes;

// Error definitions
typedef struct RustCallStatus {
    int8_t code;
    RustBuffer errorBuf;
} RustCallStatus;

// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version suffix in all instances of UNIFFI_SHARED_HEADER_V4 in this file.           ⚠️
#endif // def UNIFFI_SHARED_H
#ifndef UNIFFI_FFIDEF_RUST_FUTURE_CONTINUATION_CALLBACK
#define UNIFFI_FFIDEF_RUST_FUTURE_CONTINUATION_CALLBACK
typedef void (*UniffiRustFutureContinuationCallback)(uint64_t, int8_t
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_FREE
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_FREE
typedef void (*UniffiForeignFutureFree)(uint64_t
    );

#endif
#ifndef UNIFFI_FFIDEF_CALLBACK_INTERFACE_FREE
#define UNIFFI_FFIDEF_CALLBACK_INTERFACE_FREE
typedef void (*UniffiCallbackInterfaceFree)(uint64_t
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE
#define UNIFFI_FFIDEF_FOREIGN_FUTURE
typedef struct UniffiForeignFuture {
    uint64_t handle;
    UniffiForeignFutureFree _Nonnull free;
} UniffiForeignFuture;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U8
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U8
typedef struct UniffiForeignFutureStructU8 {
    uint8_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructU8;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U8
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U8
typedef void (*UniffiForeignFutureCompleteU8)(uint64_t, UniffiForeignFutureStructU8
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I8
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I8
typedef struct UniffiForeignFutureStructI8 {
    int8_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructI8;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I8
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I8
typedef void (*UniffiForeignFutureCompleteI8)(uint64_t, UniffiForeignFutureStructI8
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U16
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U16
typedef struct UniffiForeignFutureStructU16 {
    uint16_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructU16;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U16
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U16
typedef void (*UniffiForeignFutureCompleteU16)(uint64_t, UniffiForeignFutureStructU16
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I16
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I16
typedef struct UniffiForeignFutureStructI16 {
    int16_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructI16;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I16
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I16
typedef void (*UniffiForeignFutureCompleteI16)(uint64_t, UniffiForeignFutureStructI16
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U32
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U32
typedef struct UniffiForeignFutureStructU32 {
    uint32_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructU32;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U32
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U32
typedef void (*UniffiForeignFutureCompleteU32)(uint64_t, UniffiForeignFutureStructU32
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I32
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I32
typedef struct UniffiForeignFutureStructI32 {
    int32_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructI32;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I32
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I32
typedef void (*UniffiForeignFutureCompleteI32)(uint64_t, UniffiForeignFutureStructI32
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U64
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_U64
typedef struct UniffiForeignFutureStructU64 {
    uint64_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructU64;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U64
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_U64
typedef void (*UniffiForeignFutureCompleteU64)(uint64_t, UniffiForeignFutureStructU64
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I64
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_I64
typedef struct UniffiForeignFutureStructI64 {
    int64_t returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructI64;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I64
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_I64
typedef void (*UniffiForeignFutureCompleteI64)(uint64_t, UniffiForeignFutureStructI64
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_F32
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_F32
typedef struct UniffiForeignFutureStructF32 {
    float returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructF32;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_F32
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_F32
typedef void (*UniffiForeignFutureCompleteF32)(uint64_t, UniffiForeignFutureStructF32
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_F64
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_F64
typedef struct UniffiForeignFutureStructF64 {
    double returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructF64;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_F64
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_F64
typedef void (*UniffiForeignFutureCompleteF64)(uint64_t, UniffiForeignFutureStructF64
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_POINTER
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_POINTER
typedef struct UniffiForeignFutureStructPointer {
    void*_Nonnull returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructPointer;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_POINTER
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_POINTER
typedef void (*UniffiForeignFutureCompletePointer)(uint64_t, UniffiForeignFutureStructPointer
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_RUST_BUFFER
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_RUST_BUFFER
typedef struct UniffiForeignFutureStructRustBuffer {
    RustBuffer returnValue;
    RustCallStatus callStatus;
} UniffiForeignFutureStructRustBuffer;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_RUST_BUFFER
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_RUST_BUFFER
typedef void (*UniffiForeignFutureCompleteRustBuffer)(uint64_t, UniffiForeignFutureStructRustBuffer
    );

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_VOID
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_STRUCT_VOID
typedef struct UniffiForeignFutureStructVoid {
    RustCallStatus callStatus;
} UniffiForeignFutureStructVoid;

#endif
#ifndef UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_VOID
#define UNIFFI_FFIDEF_FOREIGN_FUTURE_COMPLETE_VOID
typedef void (*UniffiForeignFutureCompleteVoid)(uint64_t, UniffiForeignFutureStructVoid
    );

#endif
#ifndef UNIFFI_FFIDEF_CALLBACK_INTERFACE_SIGN_BY_COMPANION_METHOD0
#define UNIFFI_FFIDEF_CALLBACK_INTERFACE_SIGN_BY_COMPANION_METHOD0
typedef void (*UniffiCallbackInterfaceSignByCompanionMethod0)(uint64_t, RustBuffer, RustBuffer* _Nonnull, 
        RustCallStatus *_Nonnull uniffiCallStatus
    );

#endif
#ifndef UNIFFI_FFIDEF_CALLBACK_INTERFACE_SIGN_BY_COMPANION_METHOD1
#define UNIFFI_FFIDEF_CALLBACK_INTERFACE_SIGN_BY_COMPANION_METHOD1
typedef void (*UniffiCallbackInterfaceSignByCompanionMethod1)(uint64_t, RustBuffer* _Nonnull, 
        RustCallStatus *_Nonnull uniffiCallStatus
    );

#endif
#ifndef UNIFFI_FFIDEF_V_TABLE_CALLBACK_INTERFACE_SIGN_BY_COMPANION
#define UNIFFI_FFIDEF_V_TABLE_CALLBACK_INTERFACE_SIGN_BY_COMPANION
typedef struct UniffiVTableCallbackInterfaceSignByCompanion {
    UniffiCallbackInterfaceSignByCompanionMethod0 _Nonnull makeSignature;
    UniffiCallbackInterfaceSignByCompanionMethod1 _Nonnull exportPublicKey;
    UniffiCallbackInterfaceFree _Nonnull uniffiFree;
} UniffiVTableCallbackInterfaceSignByCompanion;

#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_ACTION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_ACTION
void*_Nonnull uniffi_siltti_fn_clone_action(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_ACTION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_ACTION
void uniffi_siltti_fn_free_action(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_ACTION_NEW_DERIVATION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_ACTION_NEW_DERIVATION
void*_Nonnull uniffi_siltti_fn_constructor_action_new_derivation(RustBuffer cut_path, int8_t has_pwd, void*_Nonnull signature_maker, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_ACTION_NEW_KAMPELA_STOP
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_ACTION_NEW_KAMPELA_STOP
void*_Nonnull uniffi_siltti_fn_constructor_action_new_kampela_stop(void*_Nonnull signature_maker, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_ACTION_NEW_PAYLOAD
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_ACTION_NEW_PAYLOAD
void*_Nonnull uniffi_siltti_fn_constructor_action_new_payload(RustBuffer payload, RustBuffer db_path, void*_Nonnull signature_maker, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_ACTION_IS_TRANSMIT
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_ACTION_IS_TRANSMIT
int8_t uniffi_siltti_fn_method_action_is_transmit(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_ACTION_MAKE_PACKET
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_ACTION_MAKE_PACKET
RustBuffer uniffi_siltti_fn_method_action_make_packet(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_COLLECTION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_COLLECTION
void*_Nonnull uniffi_siltti_fn_clone_collection(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_COLLECTION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_COLLECTION
void uniffi_siltti_fn_free_collection(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_COLLECTION_NEW
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_COLLECTION_NEW
void*_Nonnull uniffi_siltti_fn_constructor_collection_new(RustCallStatus *_Nonnull out_status
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_COLLECTION_CLEAN
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_COLLECTION_CLEAN
void uniffi_siltti_fn_method_collection_clean(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_COLLECTION_FRAMES
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_COLLECTION_FRAMES
RustBuffer uniffi_siltti_fn_method_collection_frames(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_COLLECTION_PROCESS_FRAME
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_COLLECTION_PROCESS_FRAME
RustBuffer uniffi_siltti_fn_method_collection_process_frame(void*_Nonnull ptr, RustBuffer raw_frame, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_SELECTORELEMENT
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_SELECTORELEMENT
void*_Nonnull uniffi_siltti_fn_clone_selectorelement(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_SELECTORELEMENT
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_SELECTORELEMENT
void uniffi_siltti_fn_free_selectorelement(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_SIGNBYCOMPANION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_SIGNBYCOMPANION
void*_Nonnull uniffi_siltti_fn_clone_signbycompanion(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_SIGNBYCOMPANION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_SIGNBYCOMPANION
void uniffi_siltti_fn_free_signbycompanion(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_INIT_CALLBACK_VTABLE_SIGNBYCOMPANION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_INIT_CALLBACK_VTABLE_SIGNBYCOMPANION
void uniffi_siltti_fn_init_callback_vtable_signbycompanion(UniffiVTableCallbackInterfaceSignByCompanion* _Nonnull vtable
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_SIGNBYCOMPANION_MAKE_SIGNATURE
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_SIGNBYCOMPANION_MAKE_SIGNATURE
RustBuffer uniffi_siltti_fn_method_signbycompanion_make_signature(void*_Nonnull ptr, RustBuffer data, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_SIGNBYCOMPANION_EXPORT_PUBLIC_KEY
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_SIGNBYCOMPANION_EXPORT_PUBLIC_KEY
RustBuffer uniffi_siltti_fn_method_signbycompanion_export_public_key(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_SIGNATUREMAKER
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CLONE_SIGNATUREMAKER
void*_Nonnull uniffi_siltti_fn_clone_signaturemaker(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_SIGNATUREMAKER
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FREE_SIGNATUREMAKER
void uniffi_siltti_fn_free_signaturemaker(void*_Nonnull ptr, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_SIGNATUREMAKER_NEW
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_CONSTRUCTOR_SIGNATUREMAKER_NEW
void*_Nonnull uniffi_siltti_fn_constructor_signaturemaker_new(void*_Nonnull signature_maker, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_SIGNATUREMAKER_SIGNED_DATA
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_METHOD_SIGNATUREMAKER_SIGNED_DATA
RustBuffer uniffi_siltti_fn_method_signaturemaker_signed_data(void*_Nonnull ptr, RustBuffer encoded_data, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_DELETE_BY_KEY
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_DELETE_BY_KEY
void uniffi_siltti_fn_func_delete_by_key(RustBuffer chain_key, RustBuffer db_path, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_GET_ALL_KEYS
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_GET_ALL_KEYS
RustBuffer uniffi_siltti_fn_func_get_all_keys(RustBuffer db_path, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_IS_UPDATED
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_IS_UPDATED
int8_t uniffi_siltti_fn_func_is_updated(RustBuffer db_path, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_REQUEST_DEFAULTS
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_REQUEST_DEFAULTS
void uniffi_siltti_fn_func_request_defaults(RustCallStatus *_Nonnull out_status
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_REQUEST_FULL_FETCH
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_REQUEST_FULL_FETCH
void uniffi_siltti_fn_func_request_full_fetch(RustBuffer address, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_REQUEST_UPDATE_BY_KEY
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_FN_FUNC_REQUEST_UPDATE_BY_KEY
void uniffi_siltti_fn_func_request_update_by_key(RustBuffer chain_key, RustBuffer db_path, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_ALLOC
#define UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_ALLOC
RustBuffer ffi_siltti_rustbuffer_alloc(uint64_t size, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_FROM_BYTES
#define UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_FROM_BYTES
RustBuffer ffi_siltti_rustbuffer_from_bytes(ForeignBytes bytes, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_FREE
#define UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_FREE
void ffi_siltti_rustbuffer_free(RustBuffer buf, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_RESERVE
#define UNIFFI_FFIDEF_FFI_SILTTI_RUSTBUFFER_RESERVE
RustBuffer ffi_siltti_rustbuffer_reserve(RustBuffer buf, uint64_t additional, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U8
void ffi_siltti_rust_future_poll_u8(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U8
void ffi_siltti_rust_future_cancel_u8(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U8
void ffi_siltti_rust_future_free_u8(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U8
uint8_t ffi_siltti_rust_future_complete_u8(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I8
void ffi_siltti_rust_future_poll_i8(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I8
void ffi_siltti_rust_future_cancel_i8(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I8
void ffi_siltti_rust_future_free_i8(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I8
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I8
int8_t ffi_siltti_rust_future_complete_i8(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U16
void ffi_siltti_rust_future_poll_u16(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U16
void ffi_siltti_rust_future_cancel_u16(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U16
void ffi_siltti_rust_future_free_u16(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U16
uint16_t ffi_siltti_rust_future_complete_u16(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I16
void ffi_siltti_rust_future_poll_i16(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I16
void ffi_siltti_rust_future_cancel_i16(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I16
void ffi_siltti_rust_future_free_i16(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I16
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I16
int16_t ffi_siltti_rust_future_complete_i16(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U32
void ffi_siltti_rust_future_poll_u32(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U32
void ffi_siltti_rust_future_cancel_u32(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U32
void ffi_siltti_rust_future_free_u32(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U32
uint32_t ffi_siltti_rust_future_complete_u32(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I32
void ffi_siltti_rust_future_poll_i32(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I32
void ffi_siltti_rust_future_cancel_i32(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I32
void ffi_siltti_rust_future_free_i32(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I32
int32_t ffi_siltti_rust_future_complete_i32(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_U64
void ffi_siltti_rust_future_poll_u64(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_U64
void ffi_siltti_rust_future_cancel_u64(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_U64
void ffi_siltti_rust_future_free_u64(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_U64
uint64_t ffi_siltti_rust_future_complete_u64(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_I64
void ffi_siltti_rust_future_poll_i64(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_I64
void ffi_siltti_rust_future_cancel_i64(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_I64
void ffi_siltti_rust_future_free_i64(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_I64
int64_t ffi_siltti_rust_future_complete_i64(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_F32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_F32
void ffi_siltti_rust_future_poll_f32(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_F32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_F32
void ffi_siltti_rust_future_cancel_f32(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_F32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_F32
void ffi_siltti_rust_future_free_f32(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_F32
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_F32
float ffi_siltti_rust_future_complete_f32(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_F64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_F64
void ffi_siltti_rust_future_poll_f64(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_F64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_F64
void ffi_siltti_rust_future_cancel_f64(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_F64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_F64
void ffi_siltti_rust_future_free_f64(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_F64
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_F64
double ffi_siltti_rust_future_complete_f64(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_POINTER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_POINTER
void ffi_siltti_rust_future_poll_pointer(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_POINTER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_POINTER
void ffi_siltti_rust_future_cancel_pointer(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_POINTER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_POINTER
void ffi_siltti_rust_future_free_pointer(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_POINTER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_POINTER
void*_Nonnull ffi_siltti_rust_future_complete_pointer(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_RUST_BUFFER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_RUST_BUFFER
void ffi_siltti_rust_future_poll_rust_buffer(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_RUST_BUFFER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_RUST_BUFFER
void ffi_siltti_rust_future_cancel_rust_buffer(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_RUST_BUFFER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_RUST_BUFFER
void ffi_siltti_rust_future_free_rust_buffer(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_RUST_BUFFER
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_RUST_BUFFER
RustBuffer ffi_siltti_rust_future_complete_rust_buffer(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_VOID
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_POLL_VOID
void ffi_siltti_rust_future_poll_void(uint64_t handle, UniffiRustFutureContinuationCallback _Nonnull callback, uint64_t callback_data
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_VOID
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_CANCEL_VOID
void ffi_siltti_rust_future_cancel_void(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_VOID
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_FREE_VOID
void ffi_siltti_rust_future_free_void(uint64_t handle
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_VOID
#define UNIFFI_FFIDEF_FFI_SILTTI_RUST_FUTURE_COMPLETE_VOID
void ffi_siltti_rust_future_complete_void(uint64_t handle, RustCallStatus *_Nonnull out_status
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_DELETE_BY_KEY
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_DELETE_BY_KEY
uint16_t uniffi_siltti_checksum_func_delete_by_key(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_GET_ALL_KEYS
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_GET_ALL_KEYS
uint16_t uniffi_siltti_checksum_func_get_all_keys(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_IS_UPDATED
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_IS_UPDATED
uint16_t uniffi_siltti_checksum_func_is_updated(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_REQUEST_DEFAULTS
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_REQUEST_DEFAULTS
uint16_t uniffi_siltti_checksum_func_request_defaults(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_REQUEST_FULL_FETCH
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_REQUEST_FULL_FETCH
uint16_t uniffi_siltti_checksum_func_request_full_fetch(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_REQUEST_UPDATE_BY_KEY
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_FUNC_REQUEST_UPDATE_BY_KEY
uint16_t uniffi_siltti_checksum_func_request_update_by_key(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_ACTION_IS_TRANSMIT
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_ACTION_IS_TRANSMIT
uint16_t uniffi_siltti_checksum_method_action_is_transmit(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_ACTION_MAKE_PACKET
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_ACTION_MAKE_PACKET
uint16_t uniffi_siltti_checksum_method_action_make_packet(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_COLLECTION_CLEAN
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_COLLECTION_CLEAN
uint16_t uniffi_siltti_checksum_method_collection_clean(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_COLLECTION_FRAMES
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_COLLECTION_FRAMES
uint16_t uniffi_siltti_checksum_method_collection_frames(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_COLLECTION_PROCESS_FRAME
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_COLLECTION_PROCESS_FRAME
uint16_t uniffi_siltti_checksum_method_collection_process_frame(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_SIGNBYCOMPANION_MAKE_SIGNATURE
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_SIGNBYCOMPANION_MAKE_SIGNATURE
uint16_t uniffi_siltti_checksum_method_signbycompanion_make_signature(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_SIGNBYCOMPANION_EXPORT_PUBLIC_KEY
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_SIGNBYCOMPANION_EXPORT_PUBLIC_KEY
uint16_t uniffi_siltti_checksum_method_signbycompanion_export_public_key(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_SIGNATUREMAKER_SIGNED_DATA
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_METHOD_SIGNATUREMAKER_SIGNED_DATA
uint16_t uniffi_siltti_checksum_method_signaturemaker_signed_data(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_ACTION_NEW_DERIVATION
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_ACTION_NEW_DERIVATION
uint16_t uniffi_siltti_checksum_constructor_action_new_derivation(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_ACTION_NEW_KAMPELA_STOP
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_ACTION_NEW_KAMPELA_STOP
uint16_t uniffi_siltti_checksum_constructor_action_new_kampela_stop(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_ACTION_NEW_PAYLOAD
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_ACTION_NEW_PAYLOAD
uint16_t uniffi_siltti_checksum_constructor_action_new_payload(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_COLLECTION_NEW
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_COLLECTION_NEW
uint16_t uniffi_siltti_checksum_constructor_collection_new(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_SIGNATUREMAKER_NEW
#define UNIFFI_FFIDEF_UNIFFI_SILTTI_CHECKSUM_CONSTRUCTOR_SIGNATUREMAKER_NEW
uint16_t uniffi_siltti_checksum_constructor_signaturemaker_new(void
    
);
#endif
#ifndef UNIFFI_FFIDEF_FFI_SILTTI_UNIFFI_CONTRACT_VERSION
#define UNIFFI_FFIDEF_FFI_SILTTI_UNIFFI_CONTRACT_VERSION
uint32_t ffi_siltti_uniffi_contract_version(void
    
);
#endif
