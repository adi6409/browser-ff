/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageAsyncConnection.idl
 */

#ifndef __gen_mozIStorageAsyncConnection_h__
#define __gen_mozIStorageAsyncConnection_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageCompletionCallback; /* forward declaration */

class mozIStorageFunction; /* forward declaration */

class mozIStorageProgressHandler; /* forward declaration */

class mozIStorageBaseStatement; /* forward declaration */

class mozIStorageStatement; /* forward declaration */

class mozIStorageAsyncStatement; /* forward declaration */

class mozIStorageStatementCallback; /* forward declaration */

class mozIStoragePendingStatement; /* forward declaration */

class nsIFile; /* forward declaration */


/* starting interface:    mozIStorageAsyncConnection */
#define MOZISTORAGEASYNCCONNECTION_IID_STR "8bfd34d5-4ddf-4e4b-89dd-9b14f33534c6"

#define MOZISTORAGEASYNCCONNECTION_IID \
  {0x8bfd34d5, 0x4ddf, 0x4e4b, \
    { 0x89, 0xdd, 0x9b, 0x14, 0xf3, 0x35, 0x34, 0xc6 }}

class NS_NO_VTABLE mozIStorageAsyncConnection : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEASYNCCONNECTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageAsyncConnection;

  enum {
    TRANSACTION_DEFAULT = -1,
    TRANSACTION_DEFERRED = 0,
    TRANSACTION_IMMEDIATE = 1,
    TRANSACTION_EXCLUSIVE = 2
  };

  /* attribute int32_t defaultTransactionType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultTransactionType(int32_t *aDefaultTransactionType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultTransactionType(int32_t aDefaultTransactionType) = 0;

  /* readonly attribute int32_t variableLimit; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVariableLimit(int32_t *aVariableLimit) = 0;

  /* readonly attribute boolean transactionInProgress; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTransactionInProgress(bool *aTransactionInProgress) = 0;

  /* void asyncClose ([optional] in mozIStorageCompletionCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncClose(mozIStorageCompletionCallback *aCallback) = 0;

  /* [noscript] void spinningSynchronousClose (); */
  NS_IMETHOD SpinningSynchronousClose(void) = 0;

  /* void asyncClone (in boolean aReadOnly, in mozIStorageCompletionCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncClone(bool aReadOnly, mozIStorageCompletionCallback *aCallback) = 0;

  /* readonly attribute nsIFile databaseFile; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDatabaseFile(nsIFile **aDatabaseFile) = 0;

  /* void interrupt (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Interrupt(void) = 0;

  /* mozIStorageAsyncStatement createAsyncStatement (in AUTF8String aSQLStatement); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateAsyncStatement(const nsACString& aSQLStatement, mozIStorageAsyncStatement **_retval) = 0;

  /* mozIStoragePendingStatement executeAsync (in Array<mozIStorageBaseStatement> aStatements, [optional] in mozIStorageStatementCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExecuteAsync(const nsTArray<RefPtr<mozIStorageBaseStatement>>& aStatements, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) = 0;

  /* mozIStoragePendingStatement executeSimpleSQLAsync (in AUTF8String aSQLStatement, [optional] in mozIStorageStatementCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExecuteSimpleSQLAsync(const nsACString& aSQLStatement, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) = 0;

  /* void createFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageFunction aFunction); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateFunction(const nsACString& aFunctionName, int32_t aNumArguments, mozIStorageFunction *aFunction) = 0;

  /* void removeFunction (in AUTF8String aFunctionName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveFunction(const nsACString& aFunctionName) = 0;

  /* mozIStorageProgressHandler setProgressHandler (in int32_t aGranularity, in mozIStorageProgressHandler aHandler); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetProgressHandler(int32_t aGranularity, mozIStorageProgressHandler *aHandler, mozIStorageProgressHandler **_retval) = 0;

  /* mozIStorageProgressHandler removeProgressHandler (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveProgressHandler(mozIStorageProgressHandler **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageAsyncConnection, MOZISTORAGEASYNCCONNECTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEASYNCCONNECTION \
  NS_IMETHOD GetDefaultTransactionType(int32_t *aDefaultTransactionType) override; \
  NS_IMETHOD SetDefaultTransactionType(int32_t aDefaultTransactionType) override; \
  NS_IMETHOD GetVariableLimit(int32_t *aVariableLimit) override; \
  NS_IMETHOD GetTransactionInProgress(bool *aTransactionInProgress) override; \
  NS_IMETHOD AsyncClose(mozIStorageCompletionCallback *aCallback) override; \
  NS_IMETHOD SpinningSynchronousClose(void) override; \
  NS_IMETHOD AsyncClone(bool aReadOnly, mozIStorageCompletionCallback *aCallback) override; \
  NS_IMETHOD GetDatabaseFile(nsIFile **aDatabaseFile) override; \
  NS_IMETHOD Interrupt(void) override; \
  NS_IMETHOD CreateAsyncStatement(const nsACString& aSQLStatement, mozIStorageAsyncStatement **_retval) override; \
  NS_IMETHOD ExecuteAsync(const nsTArray<RefPtr<mozIStorageBaseStatement>>& aStatements, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override; \
  NS_IMETHOD ExecuteSimpleSQLAsync(const nsACString& aSQLStatement, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override; \
  NS_IMETHOD CreateFunction(const nsACString& aFunctionName, int32_t aNumArguments, mozIStorageFunction *aFunction) override; \
  NS_IMETHOD RemoveFunction(const nsACString& aFunctionName) override; \
  NS_IMETHOD SetProgressHandler(int32_t aGranularity, mozIStorageProgressHandler *aHandler, mozIStorageProgressHandler **_retval) override; \
  NS_IMETHOD RemoveProgressHandler(mozIStorageProgressHandler **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEASYNCCONNECTION \
  nsresult GetDefaultTransactionType(int32_t *aDefaultTransactionType); \
  nsresult SetDefaultTransactionType(int32_t aDefaultTransactionType); \
  nsresult GetVariableLimit(int32_t *aVariableLimit); \
  nsresult GetTransactionInProgress(bool *aTransactionInProgress); \
  nsresult AsyncClose(mozIStorageCompletionCallback *aCallback); \
  nsresult SpinningSynchronousClose(void); \
  nsresult AsyncClone(bool aReadOnly, mozIStorageCompletionCallback *aCallback); \
  nsresult GetDatabaseFile(nsIFile **aDatabaseFile); \
  nsresult Interrupt(void); \
  nsresult CreateAsyncStatement(const nsACString& aSQLStatement, mozIStorageAsyncStatement **_retval); \
  nsresult ExecuteAsync(const nsTArray<RefPtr<mozIStorageBaseStatement>>& aStatements, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval); \
  nsresult ExecuteSimpleSQLAsync(const nsACString& aSQLStatement, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval); \
  nsresult CreateFunction(const nsACString& aFunctionName, int32_t aNumArguments, mozIStorageFunction *aFunction); \
  nsresult RemoveFunction(const nsACString& aFunctionName); \
  nsresult SetProgressHandler(int32_t aGranularity, mozIStorageProgressHandler *aHandler, mozIStorageProgressHandler **_retval); \
  nsresult RemoveProgressHandler(mozIStorageProgressHandler **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEASYNCCONNECTION(_to) \
  NS_IMETHOD GetDefaultTransactionType(int32_t *aDefaultTransactionType) override { return _to GetDefaultTransactionType(aDefaultTransactionType); } \
  NS_IMETHOD SetDefaultTransactionType(int32_t aDefaultTransactionType) override { return _to SetDefaultTransactionType(aDefaultTransactionType); } \
  NS_IMETHOD GetVariableLimit(int32_t *aVariableLimit) override { return _to GetVariableLimit(aVariableLimit); } \
  NS_IMETHOD GetTransactionInProgress(bool *aTransactionInProgress) override { return _to GetTransactionInProgress(aTransactionInProgress); } \
  NS_IMETHOD AsyncClose(mozIStorageCompletionCallback *aCallback) override { return _to AsyncClose(aCallback); } \
  NS_IMETHOD SpinningSynchronousClose(void) override { return _to SpinningSynchronousClose(); } \
  NS_IMETHOD AsyncClone(bool aReadOnly, mozIStorageCompletionCallback *aCallback) override { return _to AsyncClone(aReadOnly, aCallback); } \
  NS_IMETHOD GetDatabaseFile(nsIFile **aDatabaseFile) override { return _to GetDatabaseFile(aDatabaseFile); } \
  NS_IMETHOD Interrupt(void) override { return _to Interrupt(); } \
  NS_IMETHOD CreateAsyncStatement(const nsACString& aSQLStatement, mozIStorageAsyncStatement **_retval) override { return _to CreateAsyncStatement(aSQLStatement, _retval); } \
  NS_IMETHOD ExecuteAsync(const nsTArray<RefPtr<mozIStorageBaseStatement>>& aStatements, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return _to ExecuteAsync(aStatements, aCallback, _retval); } \
  NS_IMETHOD ExecuteSimpleSQLAsync(const nsACString& aSQLStatement, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return _to ExecuteSimpleSQLAsync(aSQLStatement, aCallback, _retval); } \
  NS_IMETHOD CreateFunction(const nsACString& aFunctionName, int32_t aNumArguments, mozIStorageFunction *aFunction) override { return _to CreateFunction(aFunctionName, aNumArguments, aFunction); } \
  NS_IMETHOD RemoveFunction(const nsACString& aFunctionName) override { return _to RemoveFunction(aFunctionName); } \
  NS_IMETHOD SetProgressHandler(int32_t aGranularity, mozIStorageProgressHandler *aHandler, mozIStorageProgressHandler **_retval) override { return _to SetProgressHandler(aGranularity, aHandler, _retval); } \
  NS_IMETHOD RemoveProgressHandler(mozIStorageProgressHandler **_retval) override { return _to RemoveProgressHandler(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEASYNCCONNECTION(_to) \
  NS_IMETHOD GetDefaultTransactionType(int32_t *aDefaultTransactionType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultTransactionType(aDefaultTransactionType); } \
  NS_IMETHOD SetDefaultTransactionType(int32_t aDefaultTransactionType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultTransactionType(aDefaultTransactionType); } \
  NS_IMETHOD GetVariableLimit(int32_t *aVariableLimit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVariableLimit(aVariableLimit); } \
  NS_IMETHOD GetTransactionInProgress(bool *aTransactionInProgress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransactionInProgress(aTransactionInProgress); } \
  NS_IMETHOD AsyncClose(mozIStorageCompletionCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncClose(aCallback); } \
  NS_IMETHOD SpinningSynchronousClose(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpinningSynchronousClose(); } \
  NS_IMETHOD AsyncClone(bool aReadOnly, mozIStorageCompletionCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncClone(aReadOnly, aCallback); } \
  NS_IMETHOD GetDatabaseFile(nsIFile **aDatabaseFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDatabaseFile(aDatabaseFile); } \
  NS_IMETHOD Interrupt(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Interrupt(); } \
  NS_IMETHOD CreateAsyncStatement(const nsACString& aSQLStatement, mozIStorageAsyncStatement **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateAsyncStatement(aSQLStatement, _retval); } \
  NS_IMETHOD ExecuteAsync(const nsTArray<RefPtr<mozIStorageBaseStatement>>& aStatements, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExecuteAsync(aStatements, aCallback, _retval); } \
  NS_IMETHOD ExecuteSimpleSQLAsync(const nsACString& aSQLStatement, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExecuteSimpleSQLAsync(aSQLStatement, aCallback, _retval); } \
  NS_IMETHOD CreateFunction(const nsACString& aFunctionName, int32_t aNumArguments, mozIStorageFunction *aFunction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateFunction(aFunctionName, aNumArguments, aFunction); } \
  NS_IMETHOD RemoveFunction(const nsACString& aFunctionName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveFunction(aFunctionName); } \
  NS_IMETHOD SetProgressHandler(int32_t aGranularity, mozIStorageProgressHandler *aHandler, mozIStorageProgressHandler **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProgressHandler(aGranularity, aHandler, _retval); } \
  NS_IMETHOD RemoveProgressHandler(mozIStorageProgressHandler **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveProgressHandler(_retval); } 


#endif /* __gen_mozIStorageAsyncConnection_h__ */
