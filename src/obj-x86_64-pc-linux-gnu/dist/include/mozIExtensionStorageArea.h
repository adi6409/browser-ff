/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/extensions/storage/mozIExtensionStorageArea.idl
 */

#ifndef __gen_mozIExtensionStorageArea_h__
#define __gen_mozIExtensionStorageArea_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIExtensionStorageCallback; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    mozIExtensionStorageArea */
#define MOZIEXTENSIONSTORAGEAREA_IID_STR "d8eb3ff1-9b4b-435a-99ca-5b8cbaba2420"

#define MOZIEXTENSIONSTORAGEAREA_IID \
  {0xd8eb3ff1, 0x9b4b, 0x435a, \
    { 0x99, 0xca, 0x5b, 0x8c, 0xba, 0xba, 0x24, 0x20 }}

class NS_NO_VTABLE mozIExtensionStorageArea : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIEXTENSIONSTORAGEAREA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIExtensionStorageArea;

  enum {
    SYNC_QUOTA_BYTES = 102400U,
    SYNC_QUOTA_BYTES_PER_ITEM = 8192U,
    SYNC_MAX_ITEMS = 512U
  };

  /* void set (in AUTF8String extensionId, in AUTF8String json, in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Set(const nsACString& extensionId, const nsACString& json, mozIExtensionStorageCallback *callback) = 0;

  /* void get (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Get(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) = 0;

  /* void remove (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Remove(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) = 0;

  /* void clear (in AUTF8String extensionId, in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clear(const nsACString& extensionId, mozIExtensionStorageCallback *callback) = 0;

  /* void getBytesInUse (in AUTF8String extensionId, in AUTF8String keys, in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBytesInUse(const nsACString& extensionId, const nsACString& keys, mozIExtensionStorageCallback *callback) = 0;

  /* void takeMigrationInfo (in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TakeMigrationInfo(mozIExtensionStorageCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIExtensionStorageArea, MOZIEXTENSIONSTORAGEAREA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIEXTENSIONSTORAGEAREA \
  NS_IMETHOD Set(const nsACString& extensionId, const nsACString& json, mozIExtensionStorageCallback *callback) override; \
  NS_IMETHOD Get(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) override; \
  NS_IMETHOD Remove(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) override; \
  NS_IMETHOD Clear(const nsACString& extensionId, mozIExtensionStorageCallback *callback) override; \
  NS_IMETHOD GetBytesInUse(const nsACString& extensionId, const nsACString& keys, mozIExtensionStorageCallback *callback) override; \
  NS_IMETHOD TakeMigrationInfo(mozIExtensionStorageCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIEXTENSIONSTORAGEAREA \
  nsresult Set(const nsACString& extensionId, const nsACString& json, mozIExtensionStorageCallback *callback); \
  nsresult Get(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback); \
  nsresult Remove(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback); \
  nsresult Clear(const nsACString& extensionId, mozIExtensionStorageCallback *callback); \
  nsresult GetBytesInUse(const nsACString& extensionId, const nsACString& keys, mozIExtensionStorageCallback *callback); \
  nsresult TakeMigrationInfo(mozIExtensionStorageCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIEXTENSIONSTORAGEAREA(_to) \
  NS_IMETHOD Set(const nsACString& extensionId, const nsACString& json, mozIExtensionStorageCallback *callback) override { return _to Set(extensionId, json, callback); } \
  NS_IMETHOD Get(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) override { return _to Get(extensionId, key, callback); } \
  NS_IMETHOD Remove(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) override { return _to Remove(extensionId, key, callback); } \
  NS_IMETHOD Clear(const nsACString& extensionId, mozIExtensionStorageCallback *callback) override { return _to Clear(extensionId, callback); } \
  NS_IMETHOD GetBytesInUse(const nsACString& extensionId, const nsACString& keys, mozIExtensionStorageCallback *callback) override { return _to GetBytesInUse(extensionId, keys, callback); } \
  NS_IMETHOD TakeMigrationInfo(mozIExtensionStorageCallback *callback) override { return _to TakeMigrationInfo(callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIEXTENSIONSTORAGEAREA(_to) \
  NS_IMETHOD Set(const nsACString& extensionId, const nsACString& json, mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Set(extensionId, json, callback); } \
  NS_IMETHOD Get(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(extensionId, key, callback); } \
  NS_IMETHOD Remove(const nsACString& extensionId, const nsACString& key, mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Remove(extensionId, key, callback); } \
  NS_IMETHOD Clear(const nsACString& extensionId, mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clear(extensionId, callback); } \
  NS_IMETHOD GetBytesInUse(const nsACString& extensionId, const nsACString& keys, mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBytesInUse(extensionId, keys, callback); } \
  NS_IMETHOD TakeMigrationInfo(mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TakeMigrationInfo(callback); } 


/* starting interface:    mozIConfigurableExtensionStorageArea */
#define MOZICONFIGURABLEEXTENSIONSTORAGEAREA_IID_STR "2b008295-1bcc-4610-84f1-ad4cab2fa9ee"

#define MOZICONFIGURABLEEXTENSIONSTORAGEAREA_IID \
  {0x2b008295, 0x1bcc, 0x4610, \
    { 0x84, 0xf1, 0xad, 0x4c, 0xab, 0x2f, 0xa9, 0xee }}

class NS_NO_VTABLE mozIConfigurableExtensionStorageArea : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZICONFIGURABLEEXTENSIONSTORAGEAREA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIConfigurableExtensionStorageArea;

  /* void configure (in nsIFile databaseFile, in nsIFile kintoFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Configure(nsIFile *databaseFile, nsIFile *kintoFile) = 0;

  /* void teardown (in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Teardown(mozIExtensionStorageCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIConfigurableExtensionStorageArea, MOZICONFIGURABLEEXTENSIONSTORAGEAREA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZICONFIGURABLEEXTENSIONSTORAGEAREA \
  NS_IMETHOD Configure(nsIFile *databaseFile, nsIFile *kintoFile) override; \
  NS_IMETHOD Teardown(mozIExtensionStorageCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZICONFIGURABLEEXTENSIONSTORAGEAREA \
  nsresult Configure(nsIFile *databaseFile, nsIFile *kintoFile); \
  nsresult Teardown(mozIExtensionStorageCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZICONFIGURABLEEXTENSIONSTORAGEAREA(_to) \
  NS_IMETHOD Configure(nsIFile *databaseFile, nsIFile *kintoFile) override { return _to Configure(databaseFile, kintoFile); } \
  NS_IMETHOD Teardown(mozIExtensionStorageCallback *callback) override { return _to Teardown(callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZICONFIGURABLEEXTENSIONSTORAGEAREA(_to) \
  NS_IMETHOD Configure(nsIFile *databaseFile, nsIFile *kintoFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Configure(databaseFile, kintoFile); } \
  NS_IMETHOD Teardown(mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Teardown(callback); } 


/* starting interface:    mozISyncedExtensionStorageArea */
#define MOZISYNCEDEXTENSIONSTORAGEAREA_IID_STR "6dac82c9-1d8a-4893-8c0f-6e626aef802c"

#define MOZISYNCEDEXTENSIONSTORAGEAREA_IID \
  {0x6dac82c9, 0x1d8a, 0x4893, \
    { 0x8c, 0x0f, 0x6e, 0x62, 0x6a, 0xef, 0x80, 0x2c }}

class NS_NO_VTABLE mozISyncedExtensionStorageArea : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISYNCEDEXTENSIONSTORAGEAREA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISyncedExtensionStorageArea;

  /* void fetchPendingSyncChanges (in mozIExtensionStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FetchPendingSyncChanges(mozIExtensionStorageCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISyncedExtensionStorageArea, MOZISYNCEDEXTENSIONSTORAGEAREA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISYNCEDEXTENSIONSTORAGEAREA \
  NS_IMETHOD FetchPendingSyncChanges(mozIExtensionStorageCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISYNCEDEXTENSIONSTORAGEAREA \
  nsresult FetchPendingSyncChanges(mozIExtensionStorageCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISYNCEDEXTENSIONSTORAGEAREA(_to) \
  NS_IMETHOD FetchPendingSyncChanges(mozIExtensionStorageCallback *callback) override { return _to FetchPendingSyncChanges(callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISYNCEDEXTENSIONSTORAGEAREA(_to) \
  NS_IMETHOD FetchPendingSyncChanges(mozIExtensionStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FetchPendingSyncChanges(callback); } 


/* starting interface:    mozIExtensionStorageListener */
#define MOZIEXTENSIONSTORAGELISTENER_IID_STR "8cb3c7e4-d0ca-4353-bccd-2673b4e11510"

#define MOZIEXTENSIONSTORAGELISTENER_IID \
  {0x8cb3c7e4, 0xd0ca, 0x4353, \
    { 0xbc, 0xcd, 0x26, 0x73, 0xb4, 0xe1, 0x15, 0x10 }}

class NS_NO_VTABLE mozIExtensionStorageListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIEXTENSIONSTORAGELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIExtensionStorageListener;

  /* void onChanged (in AUTF8String extensionId, in AUTF8String json); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnChanged(const nsACString& extensionId, const nsACString& json) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIExtensionStorageListener, MOZIEXTENSIONSTORAGELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIEXTENSIONSTORAGELISTENER \
  NS_IMETHOD OnChanged(const nsACString& extensionId, const nsACString& json) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIEXTENSIONSTORAGELISTENER \
  nsresult OnChanged(const nsACString& extensionId, const nsACString& json); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIEXTENSIONSTORAGELISTENER(_to) \
  NS_IMETHOD OnChanged(const nsACString& extensionId, const nsACString& json) override { return _to OnChanged(extensionId, json); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIEXTENSIONSTORAGELISTENER(_to) \
  NS_IMETHOD OnChanged(const nsACString& extensionId, const nsACString& json) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnChanged(extensionId, json); } 


/* starting interface:    mozIExtensionStorageCallback */
#define MOZIEXTENSIONSTORAGECALLBACK_IID_STR "870dca40-6602-4748-8493-c4253eb7f322"

#define MOZIEXTENSIONSTORAGECALLBACK_IID \
  {0x870dca40, 0x6602, 0x4748, \
    { 0x84, 0x93, 0xc4, 0x25, 0x3e, 0xb7, 0xf3, 0x22 }}

class NS_NO_VTABLE mozIExtensionStorageCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIEXTENSIONSTORAGECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIExtensionStorageCallback;

  /* void handleSuccess (in nsIVariant result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleSuccess(nsIVariant *result) = 0;

  /* void handleError (in nsresult code, in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleError(nsresult code, const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIExtensionStorageCallback, MOZIEXTENSIONSTORAGECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIEXTENSIONSTORAGECALLBACK \
  NS_IMETHOD HandleSuccess(nsIVariant *result) override; \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIEXTENSIONSTORAGECALLBACK \
  nsresult HandleSuccess(nsIVariant *result); \
  nsresult HandleError(nsresult code, const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIEXTENSIONSTORAGECALLBACK(_to) \
  NS_IMETHOD HandleSuccess(nsIVariant *result) override { return _to HandleSuccess(result); } \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override { return _to HandleError(code, message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIEXTENSIONSTORAGECALLBACK(_to) \
  NS_IMETHOD HandleSuccess(nsIVariant *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleSuccess(result); } \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleError(code, message); } 


#endif /* __gen_mozIExtensionStorageArea_h__ */
