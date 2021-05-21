/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageService.idl
 */

#ifndef __gen_mozIStorageService_h__
#define __gen_mozIStorageService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageConnection; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIFileURL; /* forward declaration */

class nsIPropertyBag2; /* forward declaration */

class nsIVariant; /* forward declaration */

class mozIStorageCompletionCallback; /* forward declaration */


/* starting interface:    mozIStorageService */
#define MOZISTORAGESERVICE_IID_STR "07b6b2f5-6d97-47b4-9584-e65bc467fe9e"

#define MOZISTORAGESERVICE_IID \
  {0x07b6b2f5, 0x6d97, 0x47b4, \
    { 0x95, 0x84, 0xe6, 0x5b, 0xc4, 0x67, 0xfe, 0x9e }}

class NS_NO_VTABLE mozIStorageService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageService;

  /* void openAsyncDatabase (in nsIVariant aDatabaseStore, [optional] in nsIPropertyBag2 aOptions, in mozIStorageCompletionCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenAsyncDatabase(nsIVariant *aDatabaseStore, nsIPropertyBag2 *aOptions, mozIStorageCompletionCallback *aCallback) = 0;

  /* mozIStorageConnection openSpecialDatabase (in ACString aStorageKey, [optional] in ACString aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenSpecialDatabase(const nsACString& aStorageKey, const nsACString& aName, mozIStorageConnection **_retval) = 0;

  /* mozIStorageConnection openDatabase (in nsIFile aDatabaseFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) = 0;

  /* mozIStorageConnection openUnsharedDatabase (in nsIFile aDatabaseFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenUnsharedDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) = 0;

  /* mozIStorageConnection openDatabaseWithFileURL (in nsIFileURL aFileURL, [optional] in ACString aTelemetryFilename); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenDatabaseWithFileURL(nsIFileURL *aFileURL, const nsACString& aTelemetryFilename, mozIStorageConnection **_retval) = 0;

  /* nsIFile backupDatabaseFile (in nsIFile aDBFile, in AString aBackupFileName, [optional] in nsIFile aBackupParentDirectory); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BackupDatabaseFile(nsIFile *aDBFile, const nsAString& aBackupFileName, nsIFile *aBackupParentDirectory, nsIFile **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageService, MOZISTORAGESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGESERVICE \
  NS_IMETHOD OpenAsyncDatabase(nsIVariant *aDatabaseStore, nsIPropertyBag2 *aOptions, mozIStorageCompletionCallback *aCallback) override; \
  NS_IMETHOD OpenSpecialDatabase(const nsACString& aStorageKey, const nsACString& aName, mozIStorageConnection **_retval) override; \
  NS_IMETHOD OpenDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) override; \
  NS_IMETHOD OpenUnsharedDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) override; \
  NS_IMETHOD OpenDatabaseWithFileURL(nsIFileURL *aFileURL, const nsACString& aTelemetryFilename, mozIStorageConnection **_retval) override; \
  NS_IMETHOD BackupDatabaseFile(nsIFile *aDBFile, const nsAString& aBackupFileName, nsIFile *aBackupParentDirectory, nsIFile **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGESERVICE \
  nsresult OpenAsyncDatabase(nsIVariant *aDatabaseStore, nsIPropertyBag2 *aOptions, mozIStorageCompletionCallback *aCallback); \
  nsresult OpenSpecialDatabase(const nsACString& aStorageKey, const nsACString& aName, mozIStorageConnection **_retval); \
  nsresult OpenDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval); \
  nsresult OpenUnsharedDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval); \
  nsresult OpenDatabaseWithFileURL(nsIFileURL *aFileURL, const nsACString& aTelemetryFilename, mozIStorageConnection **_retval); \
  nsresult BackupDatabaseFile(nsIFile *aDBFile, const nsAString& aBackupFileName, nsIFile *aBackupParentDirectory, nsIFile **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGESERVICE(_to) \
  NS_IMETHOD OpenAsyncDatabase(nsIVariant *aDatabaseStore, nsIPropertyBag2 *aOptions, mozIStorageCompletionCallback *aCallback) override { return _to OpenAsyncDatabase(aDatabaseStore, aOptions, aCallback); } \
  NS_IMETHOD OpenSpecialDatabase(const nsACString& aStorageKey, const nsACString& aName, mozIStorageConnection **_retval) override { return _to OpenSpecialDatabase(aStorageKey, aName, _retval); } \
  NS_IMETHOD OpenDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) override { return _to OpenDatabase(aDatabaseFile, _retval); } \
  NS_IMETHOD OpenUnsharedDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) override { return _to OpenUnsharedDatabase(aDatabaseFile, _retval); } \
  NS_IMETHOD OpenDatabaseWithFileURL(nsIFileURL *aFileURL, const nsACString& aTelemetryFilename, mozIStorageConnection **_retval) override { return _to OpenDatabaseWithFileURL(aFileURL, aTelemetryFilename, _retval); } \
  NS_IMETHOD BackupDatabaseFile(nsIFile *aDBFile, const nsAString& aBackupFileName, nsIFile *aBackupParentDirectory, nsIFile **_retval) override { return _to BackupDatabaseFile(aDBFile, aBackupFileName, aBackupParentDirectory, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGESERVICE(_to) \
  NS_IMETHOD OpenAsyncDatabase(nsIVariant *aDatabaseStore, nsIPropertyBag2 *aOptions, mozIStorageCompletionCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenAsyncDatabase(aDatabaseStore, aOptions, aCallback); } \
  NS_IMETHOD OpenSpecialDatabase(const nsACString& aStorageKey, const nsACString& aName, mozIStorageConnection **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenSpecialDatabase(aStorageKey, aName, _retval); } \
  NS_IMETHOD OpenDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenDatabase(aDatabaseFile, _retval); } \
  NS_IMETHOD OpenUnsharedDatabase(nsIFile *aDatabaseFile, mozIStorageConnection **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenUnsharedDatabase(aDatabaseFile, _retval); } \
  NS_IMETHOD OpenDatabaseWithFileURL(nsIFileURL *aFileURL, const nsACString& aTelemetryFilename, mozIStorageConnection **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenDatabaseWithFileURL(aFileURL, aTelemetryFilename, _retval); } \
  NS_IMETHOD BackupDatabaseFile(nsIFile *aDBFile, const nsAString& aBackupFileName, nsIFile *aBackupParentDirectory, nsIFile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BackupDatabaseFile(aDBFile, aBackupFileName, aBackupParentDirectory, _retval); } 


constexpr auto kMozStorageMemoryStorageKey = "memory"_ns;

#endif /* __gen_mozIStorageService_h__ */
