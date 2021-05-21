/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBCallbacks.idl
 */

#ifndef __gen_nsISDBCallbacks_h__
#define __gen_nsISDBCallbacks_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsISDBConnection; /* forward declaration */

class nsISDBRequest; /* forward declaration */


/* starting interface:    nsISDBCallback */
#define NS_ISDBCALLBACK_IID_STR "8cbd576c-c6bf-42fd-96ee-3b824dafe1d4"

#define NS_ISDBCALLBACK_IID \
  {0x8cbd576c, 0xc6bf, 0x42fd, \
    { 0x96, 0xee, 0x3b, 0x82, 0x4d, 0xaf, 0xe1, 0xd4 }}

class NS_NO_VTABLE nsISDBCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISDBCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISDBCallback;

  /* void onComplete (in nsISDBRequest aRequest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnComplete(nsISDBRequest *aRequest) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISDBCallback, NS_ISDBCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISDBCALLBACK \
  NS_IMETHOD OnComplete(nsISDBRequest *aRequest) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISDBCALLBACK \
  nsresult OnComplete(nsISDBRequest *aRequest); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISDBCALLBACK(_to) \
  NS_IMETHOD OnComplete(nsISDBRequest *aRequest) override { return _to OnComplete(aRequest); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISDBCALLBACK(_to) \
  NS_IMETHOD OnComplete(nsISDBRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnComplete(aRequest); } 


/* starting interface:    nsISDBCloseCallback */
#define NS_ISDBCLOSECALLBACK_IID_STR "e0821d43-62b9-40fe-99f8-ff9ab3184cbf"

#define NS_ISDBCLOSECALLBACK_IID \
  {0xe0821d43, 0x62b9, 0x40fe, \
    { 0x99, 0xf8, 0xff, 0x9a, 0xb3, 0x18, 0x4c, 0xbf }}

class NS_NO_VTABLE nsISDBCloseCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISDBCLOSECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISDBCloseCallback;

  /* void onClose (in nsISDBConnection aConnection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnClose(nsISDBConnection *aConnection) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISDBCloseCallback, NS_ISDBCLOSECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISDBCLOSECALLBACK \
  NS_IMETHOD OnClose(nsISDBConnection *aConnection) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISDBCLOSECALLBACK \
  nsresult OnClose(nsISDBConnection *aConnection); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISDBCLOSECALLBACK(_to) \
  NS_IMETHOD OnClose(nsISDBConnection *aConnection) override { return _to OnClose(aConnection); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISDBCLOSECALLBACK(_to) \
  NS_IMETHOD OnClose(nsISDBConnection *aConnection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnClose(aConnection); } 


#endif /* __gen_nsISDBCallbacks_h__ */
