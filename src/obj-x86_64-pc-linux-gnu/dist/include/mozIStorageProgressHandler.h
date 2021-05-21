/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageProgressHandler.idl
 */

#ifndef __gen_mozIStorageProgressHandler_h__
#define __gen_mozIStorageProgressHandler_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageConnection; /* forward declaration */


/* starting interface:    mozIStorageProgressHandler */
#define MOZISTORAGEPROGRESSHANDLER_IID_STR "a3a6fcd4-bf89-4208-a837-bf2a73afd30c"

#define MOZISTORAGEPROGRESSHANDLER_IID \
  {0xa3a6fcd4, 0xbf89, 0x4208, \
    { 0xa8, 0x37, 0xbf, 0x2a, 0x73, 0xaf, 0xd3, 0x0c }}

class NS_NO_VTABLE mozIStorageProgressHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEPROGRESSHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageProgressHandler;

  /* boolean onProgress (in mozIStorageConnection aConnection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnProgress(mozIStorageConnection *aConnection, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageProgressHandler, MOZISTORAGEPROGRESSHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEPROGRESSHANDLER \
  NS_IMETHOD OnProgress(mozIStorageConnection *aConnection, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEPROGRESSHANDLER \
  nsresult OnProgress(mozIStorageConnection *aConnection, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEPROGRESSHANDLER(_to) \
  NS_IMETHOD OnProgress(mozIStorageConnection *aConnection, bool *_retval) override { return _to OnProgress(aConnection, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEPROGRESSHANDLER(_to) \
  NS_IMETHOD OnProgress(mozIStorageConnection *aConnection, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnProgress(aConnection, _retval); } 


#endif /* __gen_mozIStorageProgressHandler_h__ */
