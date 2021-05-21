/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageBaseStatement.idl
 */

#ifndef __gen_mozIStorageBaseStatement_h__
#define __gen_mozIStorageBaseStatement_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_mozIStorageBindingParams_h__
#include "mozIStorageBindingParams.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageConnection; /* forward declaration */

class mozIStorageStatementCallback; /* forward declaration */

class mozIStoragePendingStatement; /* forward declaration */

class mozIStorageBindingParams; /* forward declaration */

class mozIStorageBindingParamsArray; /* forward declaration */


/* starting interface:    mozIStorageBaseStatement */
#define MOZISTORAGEBASESTATEMENT_IID_STR "16ca67aa-1325-43e2-aac7-859afd1590b2"

#define MOZISTORAGEBASESTATEMENT_IID \
  {0x16ca67aa, 0x1325, 0x43e2, \
    { 0xaa, 0xc7, 0x85, 0x9a, 0xfd, 0x15, 0x90, 0xb2 }}

class NS_NO_VTABLE mozIStorageBaseStatement : public mozIStorageBindingParams {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEBASESTATEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageBaseStatement;

  /* void finalize (); */
  NS_IMETHOD Finalize(void) = 0;

  /* void bindParameters (in mozIStorageBindingParamsArray aParameters); */
  NS_IMETHOD BindParameters(mozIStorageBindingParamsArray *aParameters) = 0;

  /* mozIStorageBindingParamsArray newBindingParamsArray (); */
  NS_IMETHOD NewBindingParamsArray(mozIStorageBindingParamsArray **_retval) = 0;

  /* mozIStoragePendingStatement executeAsync ([optional] in mozIStorageStatementCallback aCallback); */
  NS_IMETHOD ExecuteAsync(mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) = 0;

  enum {
    MOZ_STORAGE_STATEMENT_INVALID = 0,
    MOZ_STORAGE_STATEMENT_READY = 1,
    MOZ_STORAGE_STATEMENT_EXECUTING = 2
  };

  /* readonly attribute long state; */
  NS_IMETHOD GetState(int32_t *aState) = 0;

  /* AString escapeStringForLIKE (in AString aValue, in wchar aEscapeChar); */
  NS_IMETHOD EscapeStringForLIKE(const nsAString& aValue, char16_t aEscapeChar, nsAString& _retval) = 0;

  /* AUTF8String escapeUTF8StringForLIKE (in AUTF8String aValue, in char aEscapeChar); */
  NS_IMETHOD EscapeUTF8StringForLIKE(const nsACString& aValue, char aEscapeChar, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageBaseStatement, MOZISTORAGEBASESTATEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEBASESTATEMENT \
  NS_IMETHOD Finalize(void) override; \
  NS_IMETHOD BindParameters(mozIStorageBindingParamsArray *aParameters) override; \
  NS_IMETHOD NewBindingParamsArray(mozIStorageBindingParamsArray **_retval) override; \
  NS_IMETHOD ExecuteAsync(mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override; \
  NS_IMETHOD GetState(int32_t *aState) override; \
  NS_IMETHOD EscapeStringForLIKE(const nsAString& aValue, char16_t aEscapeChar, nsAString& _retval) override; \
  NS_IMETHOD EscapeUTF8StringForLIKE(const nsACString& aValue, char aEscapeChar, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEBASESTATEMENT \
  nsresult Finalize(void); \
  nsresult BindParameters(mozIStorageBindingParamsArray *aParameters); \
  nsresult NewBindingParamsArray(mozIStorageBindingParamsArray **_retval); \
  nsresult ExecuteAsync(mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval); \
  nsresult GetState(int32_t *aState); \
  nsresult EscapeStringForLIKE(const nsAString& aValue, char16_t aEscapeChar, nsAString& _retval); \
  nsresult EscapeUTF8StringForLIKE(const nsACString& aValue, char aEscapeChar, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEBASESTATEMENT(_to) \
  NS_IMETHOD Finalize(void) override { return _to Finalize(); } \
  NS_IMETHOD BindParameters(mozIStorageBindingParamsArray *aParameters) override { return _to BindParameters(aParameters); } \
  NS_IMETHOD NewBindingParamsArray(mozIStorageBindingParamsArray **_retval) override { return _to NewBindingParamsArray(_retval); } \
  NS_IMETHOD ExecuteAsync(mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return _to ExecuteAsync(aCallback, _retval); } \
  NS_IMETHOD GetState(int32_t *aState) override { return _to GetState(aState); } \
  NS_IMETHOD EscapeStringForLIKE(const nsAString& aValue, char16_t aEscapeChar, nsAString& _retval) override { return _to EscapeStringForLIKE(aValue, aEscapeChar, _retval); } \
  NS_IMETHOD EscapeUTF8StringForLIKE(const nsACString& aValue, char aEscapeChar, nsACString& _retval) override { return _to EscapeUTF8StringForLIKE(aValue, aEscapeChar, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEBASESTATEMENT(_to) \
  NS_IMETHOD Finalize(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finalize(); } \
  NS_IMETHOD BindParameters(mozIStorageBindingParamsArray *aParameters) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BindParameters(aParameters); } \
  NS_IMETHOD NewBindingParamsArray(mozIStorageBindingParamsArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewBindingParamsArray(_retval); } \
  NS_IMETHOD ExecuteAsync(mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExecuteAsync(aCallback, _retval); } \
  NS_IMETHOD GetState(int32_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD EscapeStringForLIKE(const nsAString& aValue, char16_t aEscapeChar, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EscapeStringForLIKE(aValue, aEscapeChar, _retval); } \
  NS_IMETHOD EscapeUTF8StringForLIKE(const nsACString& aValue, char aEscapeChar, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EscapeUTF8StringForLIKE(aValue, aEscapeChar, _retval); } 


#endif /* __gen_mozIStorageBaseStatement_h__ */
