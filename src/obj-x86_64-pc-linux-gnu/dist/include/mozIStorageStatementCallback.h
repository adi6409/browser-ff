/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageStatementCallback.idl
 */

#ifndef __gen_mozIStorageStatementCallback_h__
#define __gen_mozIStorageStatementCallback_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageResultSet; /* forward declaration */

class mozIStorageError; /* forward declaration */


/* starting interface:    mozIStorageStatementCallback */
#define MOZISTORAGESTATEMENTCALLBACK_IID_STR "29383d00-d8c4-4ddd-9f8b-c2feb0f2fcfa"

#define MOZISTORAGESTATEMENTCALLBACK_IID \
  {0x29383d00, 0xd8c4, 0x4ddd, \
    { 0x9f, 0x8b, 0xc2, 0xfe, 0xb0, 0xf2, 0xfc, 0xfa }}

class NS_NO_VTABLE mozIStorageStatementCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGESTATEMENTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageStatementCallback;

  /* void handleResult (in mozIStorageResultSet aResultSet); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleResult(mozIStorageResultSet *aResultSet) = 0;

  /* void handleError (in mozIStorageError aError); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleError(mozIStorageError *aError) = 0;

  enum {
    REASON_FINISHED = 0U,
    REASON_CANCELED = 1U,
    REASON_ERROR = 2U
  };

  /* void handleCompletion (in unsigned short aReason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleCompletion(uint16_t aReason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageStatementCallback, MOZISTORAGESTATEMENTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGESTATEMENTCALLBACK \
  NS_IMETHOD HandleResult(mozIStorageResultSet *aResultSet) override; \
  NS_IMETHOD HandleError(mozIStorageError *aError) override; \
  NS_IMETHOD HandleCompletion(uint16_t aReason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGESTATEMENTCALLBACK \
  nsresult HandleResult(mozIStorageResultSet *aResultSet); \
  nsresult HandleError(mozIStorageError *aError); \
  nsresult HandleCompletion(uint16_t aReason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGESTATEMENTCALLBACK(_to) \
  NS_IMETHOD HandleResult(mozIStorageResultSet *aResultSet) override { return _to HandleResult(aResultSet); } \
  NS_IMETHOD HandleError(mozIStorageError *aError) override { return _to HandleError(aError); } \
  NS_IMETHOD HandleCompletion(uint16_t aReason) override { return _to HandleCompletion(aReason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGESTATEMENTCALLBACK(_to) \
  NS_IMETHOD HandleResult(mozIStorageResultSet *aResultSet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleResult(aResultSet); } \
  NS_IMETHOD HandleError(mozIStorageError *aError) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleError(aError); } \
  NS_IMETHOD HandleCompletion(uint16_t aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleCompletion(aReason); } 


#endif /* __gen_mozIStorageStatementCallback_h__ */
