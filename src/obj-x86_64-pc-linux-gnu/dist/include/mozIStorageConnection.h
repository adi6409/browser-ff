/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageConnection.idl
 */

#ifndef __gen_mozIStorageConnection_h__
#define __gen_mozIStorageConnection_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_mozIStorageAsyncConnection_h__
#include "mozIStorageAsyncConnection.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
namespace quota {
class QuotaObject;
}
}
}
class mozIStorageAggregateFunction; /* forward declaration */

class mozIStorageCompletionCallback; /* forward declaration */

class mozIStorageFunction; /* forward declaration */

class mozIStorageProgressHandler; /* forward declaration */

class mozIStorageBaseStatement; /* forward declaration */

class mozIStorageStatement; /* forward declaration */

class mozIStorageAsyncStatement; /* forward declaration */

class mozIStorageStatementCallback; /* forward declaration */

class mozIStoragePendingStatement; /* forward declaration */

class nsIFile; /* forward declaration */


/* starting interface:    mozIStorageConnection */
#define MOZISTORAGECONNECTION_IID_STR "4aa2ac47-8d24-4004-9b31-ec0bd85f0cc3"

#define MOZISTORAGECONNECTION_IID \
  {0x4aa2ac47, 0x8d24, 0x4004, \
    { 0x9b, 0x31, 0xec, 0x0b, 0xd8, 0x5f, 0x0c, 0xc3 }}

class NS_NO_VTABLE mozIStorageConnection : public mozIStorageAsyncConnection {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGECONNECTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageConnection;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* mozIStorageConnection clone ([optional] in boolean aReadOnly); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clone(bool aReadOnly, mozIStorageConnection **_retval) = 0;

  /* readonly attribute long defaultPageSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultPageSize(int32_t *aDefaultPageSize) = 0;

  /* readonly attribute boolean connectionReady; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetConnectionReady(bool *aConnectionReady) = 0;

  /* readonly attribute long long lastInsertRowID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastInsertRowID(int64_t *aLastInsertRowID) = 0;

  /* readonly attribute long affectedRows; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAffectedRows(int32_t *aAffectedRows) = 0;

  /* readonly attribute long lastError; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastError(int32_t *aLastError) = 0;

  /* readonly attribute AUTF8String lastErrorString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastErrorString(nsACString& aLastErrorString) = 0;

  /* attribute long schemaVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSchemaVersion(int32_t *aSchemaVersion) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSchemaVersion(int32_t aSchemaVersion) = 0;

  /* mozIStorageStatement createStatement (in AUTF8String aSQLStatement); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateStatement(const nsACString& aSQLStatement, mozIStorageStatement **_retval) = 0;

  /* void executeSimpleSQL (in AUTF8String aSQLStatement); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExecuteSimpleSQL(const nsACString& aSQLStatement) = 0;

  /* boolean tableExists (in AUTF8String aTableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TableExists(const nsACString& aTableName, bool *_retval) = 0;

  /* boolean indexExists (in AUTF8String aIndexName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IndexExists(const nsACString& aIndexName, bool *_retval) = 0;

  /* void beginTransaction (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BeginTransaction(void) = 0;

  /* void commitTransaction (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CommitTransaction(void) = 0;

  /* void rollbackTransaction (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RollbackTransaction(void) = 0;

  /* void createTable (in string aTableName, in string aTableSchema); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateTable(const char * aTableName, const char * aTableSchema) = 0;

  /* void setGrowthIncrement (in int32_t aIncrement, in AUTF8String aDatabaseName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetGrowthIncrement(int32_t aIncrement, const nsACString& aDatabaseName) = 0;

  /* [noscript] void enableModule (in ACString aModuleName); */
  NS_IMETHOD EnableModule(const nsACString& aModuleName) = 0;

  /* [noscript] void getQuotaObjects (out QuotaObject aDatabaseQuotaObject, out QuotaObject aJournalQuotaObject); */
  NS_IMETHOD GetQuotaObjects(mozilla::dom::quota::QuotaObject * * aDatabaseQuotaObject, mozilla::dom::quota::QuotaObject * * aJournalQuotaObject) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageConnection, MOZISTORAGECONNECTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGECONNECTION \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD Clone(bool aReadOnly, mozIStorageConnection **_retval) override; \
  NS_IMETHOD GetDefaultPageSize(int32_t *aDefaultPageSize) override; \
  NS_IMETHOD GetConnectionReady(bool *aConnectionReady) override; \
  NS_IMETHOD GetLastInsertRowID(int64_t *aLastInsertRowID) override; \
  NS_IMETHOD GetAffectedRows(int32_t *aAffectedRows) override; \
  NS_IMETHOD GetLastError(int32_t *aLastError) override; \
  NS_IMETHOD GetLastErrorString(nsACString& aLastErrorString) override; \
  NS_IMETHOD GetSchemaVersion(int32_t *aSchemaVersion) override; \
  NS_IMETHOD SetSchemaVersion(int32_t aSchemaVersion) override; \
  NS_IMETHOD CreateStatement(const nsACString& aSQLStatement, mozIStorageStatement **_retval) override; \
  NS_IMETHOD ExecuteSimpleSQL(const nsACString& aSQLStatement) override; \
  NS_IMETHOD TableExists(const nsACString& aTableName, bool *_retval) override; \
  NS_IMETHOD IndexExists(const nsACString& aIndexName, bool *_retval) override; \
  NS_IMETHOD BeginTransaction(void) override; \
  NS_IMETHOD CommitTransaction(void) override; \
  NS_IMETHOD RollbackTransaction(void) override; \
  NS_IMETHOD CreateTable(const char * aTableName, const char * aTableSchema) override; \
  NS_IMETHOD SetGrowthIncrement(int32_t aIncrement, const nsACString& aDatabaseName) override; \
  NS_IMETHOD EnableModule(const nsACString& aModuleName) override; \
  NS_IMETHOD GetQuotaObjects(mozilla::dom::quota::QuotaObject * * aDatabaseQuotaObject, mozilla::dom::quota::QuotaObject * * aJournalQuotaObject) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGECONNECTION \
  nsresult Close(void); \
  nsresult Clone(bool aReadOnly, mozIStorageConnection **_retval); \
  nsresult GetDefaultPageSize(int32_t *aDefaultPageSize); \
  nsresult GetConnectionReady(bool *aConnectionReady); \
  nsresult GetLastInsertRowID(int64_t *aLastInsertRowID); \
  nsresult GetAffectedRows(int32_t *aAffectedRows); \
  nsresult GetLastError(int32_t *aLastError); \
  nsresult GetLastErrorString(nsACString& aLastErrorString); \
  nsresult GetSchemaVersion(int32_t *aSchemaVersion); \
  nsresult SetSchemaVersion(int32_t aSchemaVersion); \
  nsresult CreateStatement(const nsACString& aSQLStatement, mozIStorageStatement **_retval); \
  nsresult ExecuteSimpleSQL(const nsACString& aSQLStatement); \
  nsresult TableExists(const nsACString& aTableName, bool *_retval); \
  nsresult IndexExists(const nsACString& aIndexName, bool *_retval); \
  nsresult BeginTransaction(void); \
  nsresult CommitTransaction(void); \
  nsresult RollbackTransaction(void); \
  nsresult CreateTable(const char * aTableName, const char * aTableSchema); \
  nsresult SetGrowthIncrement(int32_t aIncrement, const nsACString& aDatabaseName); \
  nsresult EnableModule(const nsACString& aModuleName); \
  nsresult GetQuotaObjects(mozilla::dom::quota::QuotaObject * * aDatabaseQuotaObject, mozilla::dom::quota::QuotaObject * * aJournalQuotaObject); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGECONNECTION(_to) \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD Clone(bool aReadOnly, mozIStorageConnection **_retval) override { return _to Clone(aReadOnly, _retval); } \
  NS_IMETHOD GetDefaultPageSize(int32_t *aDefaultPageSize) override { return _to GetDefaultPageSize(aDefaultPageSize); } \
  NS_IMETHOD GetConnectionReady(bool *aConnectionReady) override { return _to GetConnectionReady(aConnectionReady); } \
  NS_IMETHOD GetLastInsertRowID(int64_t *aLastInsertRowID) override { return _to GetLastInsertRowID(aLastInsertRowID); } \
  NS_IMETHOD GetAffectedRows(int32_t *aAffectedRows) override { return _to GetAffectedRows(aAffectedRows); } \
  NS_IMETHOD GetLastError(int32_t *aLastError) override { return _to GetLastError(aLastError); } \
  NS_IMETHOD GetLastErrorString(nsACString& aLastErrorString) override { return _to GetLastErrorString(aLastErrorString); } \
  NS_IMETHOD GetSchemaVersion(int32_t *aSchemaVersion) override { return _to GetSchemaVersion(aSchemaVersion); } \
  NS_IMETHOD SetSchemaVersion(int32_t aSchemaVersion) override { return _to SetSchemaVersion(aSchemaVersion); } \
  NS_IMETHOD CreateStatement(const nsACString& aSQLStatement, mozIStorageStatement **_retval) override { return _to CreateStatement(aSQLStatement, _retval); } \
  NS_IMETHOD ExecuteSimpleSQL(const nsACString& aSQLStatement) override { return _to ExecuteSimpleSQL(aSQLStatement); } \
  NS_IMETHOD TableExists(const nsACString& aTableName, bool *_retval) override { return _to TableExists(aTableName, _retval); } \
  NS_IMETHOD IndexExists(const nsACString& aIndexName, bool *_retval) override { return _to IndexExists(aIndexName, _retval); } \
  NS_IMETHOD BeginTransaction(void) override { return _to BeginTransaction(); } \
  NS_IMETHOD CommitTransaction(void) override { return _to CommitTransaction(); } \
  NS_IMETHOD RollbackTransaction(void) override { return _to RollbackTransaction(); } \
  NS_IMETHOD CreateTable(const char * aTableName, const char * aTableSchema) override { return _to CreateTable(aTableName, aTableSchema); } \
  NS_IMETHOD SetGrowthIncrement(int32_t aIncrement, const nsACString& aDatabaseName) override { return _to SetGrowthIncrement(aIncrement, aDatabaseName); } \
  NS_IMETHOD EnableModule(const nsACString& aModuleName) override { return _to EnableModule(aModuleName); } \
  NS_IMETHOD GetQuotaObjects(mozilla::dom::quota::QuotaObject * * aDatabaseQuotaObject, mozilla::dom::quota::QuotaObject * * aJournalQuotaObject) override { return _to GetQuotaObjects(aDatabaseQuotaObject, aJournalQuotaObject); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGECONNECTION(_to) \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD Clone(bool aReadOnly, mozIStorageConnection **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(aReadOnly, _retval); } \
  NS_IMETHOD GetDefaultPageSize(int32_t *aDefaultPageSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultPageSize(aDefaultPageSize); } \
  NS_IMETHOD GetConnectionReady(bool *aConnectionReady) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectionReady(aConnectionReady); } \
  NS_IMETHOD GetLastInsertRowID(int64_t *aLastInsertRowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastInsertRowID(aLastInsertRowID); } \
  NS_IMETHOD GetAffectedRows(int32_t *aAffectedRows) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAffectedRows(aAffectedRows); } \
  NS_IMETHOD GetLastError(int32_t *aLastError) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastError(aLastError); } \
  NS_IMETHOD GetLastErrorString(nsACString& aLastErrorString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastErrorString(aLastErrorString); } \
  NS_IMETHOD GetSchemaVersion(int32_t *aSchemaVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSchemaVersion(aSchemaVersion); } \
  NS_IMETHOD SetSchemaVersion(int32_t aSchemaVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSchemaVersion(aSchemaVersion); } \
  NS_IMETHOD CreateStatement(const nsACString& aSQLStatement, mozIStorageStatement **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateStatement(aSQLStatement, _retval); } \
  NS_IMETHOD ExecuteSimpleSQL(const nsACString& aSQLStatement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExecuteSimpleSQL(aSQLStatement); } \
  NS_IMETHOD TableExists(const nsACString& aTableName, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TableExists(aTableName, _retval); } \
  NS_IMETHOD IndexExists(const nsACString& aIndexName, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IndexExists(aIndexName, _retval); } \
  NS_IMETHOD BeginTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginTransaction(); } \
  NS_IMETHOD CommitTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CommitTransaction(); } \
  NS_IMETHOD RollbackTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RollbackTransaction(); } \
  NS_IMETHOD CreateTable(const char * aTableName, const char * aTableSchema) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateTable(aTableName, aTableSchema); } \
  NS_IMETHOD SetGrowthIncrement(int32_t aIncrement, const nsACString& aDatabaseName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetGrowthIncrement(aIncrement, aDatabaseName); } \
  NS_IMETHOD EnableModule(const nsACString& aModuleName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableModule(aModuleName); } \
  NS_IMETHOD GetQuotaObjects(mozilla::dom::quota::QuotaObject * * aDatabaseQuotaObject, mozilla::dom::quota::QuotaObject * * aJournalQuotaObject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQuotaObjects(aDatabaseQuotaObject, aJournalQuotaObject); } 


#endif /* __gen_mozIStorageConnection_h__ */
