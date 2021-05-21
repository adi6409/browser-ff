/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/system/nsIOSFileConstantsService.idl
 */

#ifndef __gen_nsIOSFileConstantsService_h__
#define __gen_nsIOSFileConstantsService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIOSFileConstantsService */
#define NS_IOSFILECONSTANTSSERVICE_IID_STR "d6dd239f-34d6-4b34-baa1-f69ab4a20bc4"

#define NS_IOSFILECONSTANTSSERVICE_IID \
  {0xd6dd239f, 0x34d6, 0x4b34, \
    { 0xba, 0xa1, 0xf6, 0x9a, 0xb4, 0xa2, 0x0b, 0xc4 }}

class NS_NO_VTABLE nsIOSFileConstantsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOSFILECONSTANTSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOSFileConstantsService;

  /* [implicit_jscontext] void init (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(JSContext* cx) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOSFileConstantsService, NS_IOSFILECONSTANTSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOSFILECONSTANTSSERVICE \
  NS_IMETHOD Init(JSContext* cx) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOSFILECONSTANTSSERVICE \
  nsresult Init(JSContext* cx); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOSFILECONSTANTSSERVICE(_to) \
  NS_IMETHOD Init(JSContext* cx) override { return _to Init(cx); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOSFILECONSTANTSSERVICE(_to) \
  NS_IMETHOD Init(JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(cx); } 


// {4BBE1B96-8956-457F-A03F-9C27435F2AFA}
#define OSFILECONSTANTSSERVICE_CID {0x4BBE1B96,0x8956,0x457F,{0xA0,0x3F,0x9C,0x27,0x43,0x5F,0x2A,0xFA}}
#define OSFILECONSTANTSSERVICE_CONTRACTID "@mozilla.org/net/osfileconstantsservice;1"

#endif /* __gen_nsIOSFileConstantsService_h__ */
